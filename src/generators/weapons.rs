use std::ops::Range;

use rand::Rng;

use crate::components::{
    attack::Attack,
    weapon::{Weapon, WeaponDescriptor, WeaponType},
};

use super::generator::Generator;

pub struct WeaponGenerator;

impl WeaponGenerator {
    pub fn for_weapon_type(weapon_type: &WeaponType) -> impl Generator<Weapon> {
        match *weapon_type {
            WeaponType::Club => WeaponPrototype::club(),
            WeaponType::Dagger => WeaponPrototype::dagger(),
            WeaponType::Hammer => WeaponPrototype::hammer(),
            WeaponType::LongSword => WeaponPrototype::long_sword(),
            WeaponType::ShortSword => WeaponPrototype::short_sword(),
        }
    }
}

struct WeaponPrototype {
    pub weapon_type: WeaponType,
    pub num_descriptors: Range<usize>,
    pub attack: Option<Attack>,
}

impl WeaponPrototype {
    pub fn dagger() -> Self {
        Self {
            weapon_type: WeaponType::Dagger,
            num_descriptors: 0..3,
            attack: Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
        }
    }

    pub fn club() -> Self {
        Self {
            weapon_type: WeaponType::Club,
            num_descriptors: 0..2,
            attack: Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
        }
    }

    pub fn hammer() -> Self {
        Self {
            weapon_type: WeaponType::Hammer,
            num_descriptors: 0..2,
            attack: Some(Attack {
                minimum: 1,
                maximum: 3,
            }),
        }
    }

    pub fn long_sword() -> Self {
        Self {
            weapon_type: WeaponType::LongSword,
            num_descriptors: 0..3,
            attack: Some(Attack {
                minimum: 2,
                maximum: 6,
            }),
        }
    }

    pub fn short_sword() -> Self {
        Self {
            weapon_type: WeaponType::ShortSword,
            num_descriptors: 0..3,
            attack: Some(Attack {
                minimum: 2,
                maximum: 4,
            }),
        }
    }
}

impl Generator<Weapon> for WeaponPrototype {
    fn generate(&self) -> Weapon {
        let mut rng = rand::thread_rng();
        let mut num_descriptors: usize = rng.gen_range(self.num_descriptors.clone());

        let mut possible_descriptors: Vec<WeaponDescriptor> =
            self.weapon_type.possible_descriptors().to_vec();
        let mut descriptors: Vec<WeaponDescriptor> = Vec::new();
        while num_descriptors > 0 {
            if possible_descriptors.is_empty() {
                break;
            }

            let index = rng.gen_range(0..possible_descriptors.len());
            let descriptor = possible_descriptors.remove(index);
            descriptors.push(descriptor);

            num_descriptors -= 1;
        }

        Weapon {
            attack: self.attack.clone(),
            weapon_type: self.weapon_type.clone(),
            descriptors,
        }
    }
}
