//! Basic audio engine to provide sample plaback from trackers (and maybe sound effects)

mod sample;
mod sample_pack;
mod player;


pub use player::{SamplePlayer, PlayerHandle};
pub use sample_pack::SamplePack;
