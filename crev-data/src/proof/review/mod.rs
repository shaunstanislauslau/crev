use crate::level::Level;
use crate::proof;
use std::default::Default;

pub mod code;
pub mod project;


pub use self::code::*;
pub use self::project::*;

pub trait Common : super::ContentCommon {
    fn project_id(&self) ->&str ;
    fn score(&self) ->&Score ;
}

#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
pub struct Score {
    #[builder(default = "Default::default()")]
    pub thoroughness: Level,
    #[builder(default = "Default::default()")]
    pub understanding: Level,
    #[builder(default = "Default::default()")]
    pub trust: Level,
    #[builder(default = "proof::default_distrust_level()")]
    #[serde(
        skip_serializing_if = "proof::equals_default_distrust_level",
        default = "proof::default_distrust_level"
    )]
    pub distrust: Level,
}

impl Default for Score {
    fn default() -> Self {
        Score {
            thoroughness: Level::Low,
            understanding: Level::Medium,
            trust: Level::Medium,
            distrust: Level::None,
        }
    }
}