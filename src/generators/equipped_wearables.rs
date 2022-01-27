use crate::components::wearable::{Wearable, WearableType};

use super::{equipped_items::EquippedItemPrototype, wearables::WearableGenerator};

impl EquippedItemPrototype<Wearable> {
    pub fn armour(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearableGenerator::for_wearable_type(&WearableType::Armour)),
            multiple: true,
        }
    }

    pub fn cloak(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearableGenerator::for_wearable_type(&WearableType::Cloak)),
            multiple: false,
        }
    }

    pub fn clothing(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearableGenerator::for_wearable_type(
                &WearableType::Clothing,
            )),
            multiple: true,
        }
    }

    pub fn plate_mail(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearableGenerator::for_wearable_type(
                &WearableType::PlateMailHelmet,
            )),
            multiple: true,
        }
    }

    pub fn shackles(hidden_chance: usize, equipped_location_chance: usize) -> Self {
        Self {
            hidden_chance,
            equipped_location_chance,
            generator: Box::new(WearableGenerator::for_wearable_type(
                &WearableType::Shackles,
            )),
            multiple: false,
        }
    }
}
