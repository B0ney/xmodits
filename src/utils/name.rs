use crate::{SampleNamerFunc, TrackerSample};

/// Format name of exported sample.
///
/// If the sample name is empty it'll just be: $n.wav e.g 0.wav
///
/// If the sample does have a name, it'll be "$n - $name.wav"
pub fn name_sample(sample: &TrackerSample, idx: usize) -> String {
    format!(
        "{:02}{}.wav",
        idx + 1, // use human readable indexing.
        match &sample.filename.trim() {
            x if x.is_empty() => "".to_string(),
            x => format!(" - {}", x.replace(".wav", "").replace('.', "_")),
        }
    )
}

#[derive(Default)]
pub struct SampleNamer {
    index_only: bool,
    index_padding: Option<usize>,
    index_raw: bool,
    lower: bool,
    upper: bool,
}

impl SampleNamer {
    /// Dynamically build a function to format sample names given its internal parameters
    fn to_func(self) -> Box<SampleNamerFunc> {
        const DEFAULT_PADDING: usize = 2;

        Box::new(move |smp: &TrackerSample, idx: usize| -> String {
            format!(
                "{}{}.wav",
                // Index component
                {
                    let index = match self.index_raw {
                        true => smp.raw_index(),
                        _ => idx + 1,
                    };
                    match self.index_padding {
                        Some(padding) => format!("{:0padding$}", index),
                        None => format!("{:0DEFAULT_PADDING$}", index),
                    }
                },
                // Name component
                match self.index_only {
                    true => "".to_string(),
                    _ => match smp.filename.trim() {
                        name if name.is_empty() => "".to_string(),
                        name => format!(" - {}", {
                            let name = name.replace(".wav", "").replace('.', "_");

                            match (self.upper, self.lower) {
                                (true, false) => name.to_ascii_uppercase(),
                                (false, true) => name.to_ascii_lowercase(),
                                _ => name,
                            }
                        }),
                    },
                }
            )
        })
    }

    pub fn build_func(
        index_only: bool,
        index_padding: Option<usize>,
        index_raw: bool,
        lower: bool,
        upper: bool,
    ) -> Box<SampleNamerFunc> {
        SampleNamer {
            index_only,
            index_padding,
            index_raw,
            lower,
            upper,
        }
        .to_func()
    }
}
