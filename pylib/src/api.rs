use std::path::{Path, PathBuf};
use xmodits_lib::*;

pub fn rip(
    path: String,
    destination: String,
    index_raw: Option<bool>,
    index_padding: Option<usize>,
    index_only: Option<bool>,
    with_folder: Option<bool>,
) -> Result<(), Error> {
    let namer: SampleNamer = SampleNamer{
        index_only,
        index_padding,
        index_raw,
        with_folder,
    };

    let destination: PathBuf = match namer.with_folder {
        Some(true) => {
            let modname: String = Path::new(&path)
                .file_name().unwrap()
                .to_str().unwrap()
                .replace(".", "_");
            
            let new_folder: PathBuf = PathBuf::new()
                .join(&destination).join(modname);

            new_folder
        },
        _ => PathBuf::new().join(&destination),
    };

    let create_if_absent: bool = namer.with_folder.is_some();

    xmodits_lib::load_module(path)?
        .dump_advanced(
            &destination,
            &namer.to_func(),
            create_if_absent
        )
}

#[derive(Default)]
struct SampleNamer {    
    index_only: Option<bool>,
    index_padding: Option<usize>,
    index_raw: Option<bool>,
    with_folder: Option<bool>,
}

impl SampleNamer {
    /// Dynamically build a function to format sample names given its internal parameters
    fn to_func(self) -> Box<SampleNamerFunc> {
        const DEFAULT_PADDING: usize = 2;

        Box::new(
            move |smp: &TrackerSample, idx: usize| {
                format!(
                    "{}{}.wav",
                    // Index component
                    {
                        let index = match self.index_raw {
                            Some(true)  => smp.raw_index(),
                            _           => idx + 1,
                        };
                        match self.index_padding {
                            Some(padding)   => format!("{:0padding$}", index),
                            None            => format!("{:0DEFAULT_PADDING$}", index),
                        }
                    },
                    // Name component
                    match self.index_only {
                        Some(true) => "".to_string(),
                        _ => match smp.filename.trim() 
                        {
                            name if name.is_empty() => "".to_string(),
                            name => format!(
                                " - {}", 
                                name.replace(".wav", "").replace(".", "_")
                            ),
                        }
                    }
                )
            }
        )
    }
}