use rand::Rng;

use crate::components::{
    fixtures::{fixture::Fixture, fixture_type::FixtureType},
    identifier::Identifier,
    items::descriptor::Descriptor,
    material::BuiltWithMaterial,
    size::Size,
    tag::{Tag, Tagged},
};

use super::{generator::Generator, utils::item_descriptors::matches_tags};

const HAS_MATERIAL_CHANCE: usize = 90;
const HAS_NON_STANDARD_SIZE: usize = 50;

pub struct FixturePrototype {
    pub fixture_type: FixtureType,
}

pub fn get_generator(fixture_type: &FixtureType) -> impl Generator<Fixture> {
    FixturePrototype {
        fixture_type: fixture_type.clone(),
    }
}

impl Generator<Fixture> for FixturePrototype {
    fn generate(&self) -> Fixture {
        let mut rng = rand::thread_rng();
        let has_material = rng.gen_range(0..=100) <= HAS_MATERIAL_CHANCE;

        let material = if has_material {
            let possible_materials = self.fixture_type.possible_materials();
            let index = rng.gen_range(0..possible_materials.len());
            possible_materials.get(index).cloned()
        } else {
            None
        };

        let non_average_size_roll: usize = rng.gen_range(0..=100);
        let size = if non_average_size_roll <= HAS_NON_STANDARD_SIZE {
            let possibilities = non_average_sizes();
            let index = rng.gen_range(0..possibilities.len());
            match possibilities.get(index) {
                Some(height) => height.clone(),
                None => Size::Average,
            }
        } else {
            Size::Average
        };

        let mut num_descriptors = rng.gen_range(0..=2);
        let mut possible_descriptors: Vec<Descriptor> = match &material {
            Some(material) => {
                let tags: Vec<Tag> = self
                    .fixture_type
                    .tags()
                    .into_iter()
                    .chain(material.tags().into_iter())
                    .collect();
                matches_tags(&tags)
            }
            None => matches_tags(&self.fixture_type.tags()),
        };
        let mut descriptors: Vec<Descriptor> = Vec::new();
        while num_descriptors > 0 {
            if possible_descriptors.is_empty() {
                break;
            }

            let index = rng.gen_range(0..possible_descriptors.len());
            let descriptor = possible_descriptors.remove(index);
            descriptors.push(descriptor);

            num_descriptors -= 1;
        }

        Fixture {
            identifier: Identifier::just_id(),
            material,
            fixture_type: self.fixture_type.clone(),
            size,
            descriptors,
            contained_items: Vec::new(),
            hidden_compartment_items: Vec::new(),
            has_hidden_compartment: false,
        }
    }
}

fn non_average_sizes() -> Vec<Size> {
    vec![
        Size::Small,
        Size::Tiny,
        Size::Large,
        Size::Huge,
        Size::Massive,
    ]
}
