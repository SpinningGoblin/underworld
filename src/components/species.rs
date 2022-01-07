use std::ops::Range;

use crate::describable::Describable;

#[derive(Clone, Debug)]
pub enum Species {
    Bugbear,
    Goblin,
    Kobold,
    Ogre,
    Orc,
    Unknown,
}

const SMALL_HEIGHT: Range<f32> = 0.6..1.2;
const MEDIUM_HEIGHT: Range<f32> = 1.2..2.05;
const LARGE_HEIGHT: Range<f32> = 2.05..4.4;
const UNKNOWN_HEIGHT: Range<f32> = 0.6..4.4;

impl Species {
    pub fn height_range(&self) -> Range<f32> {
        match *self {
            Self::Bugbear => MEDIUM_HEIGHT,
            Self::Goblin => SMALL_HEIGHT,
            Self::Kobold => SMALL_HEIGHT,
            Self::Ogre => LARGE_HEIGHT,
            Self::Orc => MEDIUM_HEIGHT,
            Self::Unknown => UNKNOWN_HEIGHT,
        }
    }
}

impl Describable for Species {
    fn describe(&self) -> String {
        match *self {
            Self::Bugbear => "Bugbear".to_string(),
            Self::Goblin => "Goblin".to_string(),
            Self::Kobold => "Kobold".to_string(),
            Self::Ogre => "Ogre".to_string(),
            Self::Orc => "Orc".to_string(),
            _ => "Mysterious Entity".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{components::species::Species, describable::Describable};

    #[test]
    fn describe_when_bugbear() {
        assert_eq!("Bugbear", Species::Bugbear.describe());
    }

    #[test]
    fn describe_when_goblin() {
        assert_eq!("Goblin", Species::Goblin.describe());
    }

    #[test]
    fn describe_when_kobold() {
        assert_eq!("Kobold", Species::Kobold.describe());
    }

    #[test]
    fn describe_when_orc() {
        assert_eq!("Orc", Species::Orc.describe());
    }

    #[test]
    fn describe_when_unknown() {
        assert_eq!("Mysterious Entity", Species::Unknown.describe());
    }
}
