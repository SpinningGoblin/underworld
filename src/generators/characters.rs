mod prototypes;

use rand::Rng;

use crate::components::{
    character::Character, effects::Effects, inventory::Inventory, life_modifier::LifeModifier,
    species::Species, spells::spell_memory::SpellMemory,
};

use self::prototypes::{basic_character, overloaded_character, undead_character};

use super::{generator::Generator, stats::build_default_health_rolls};

pub struct CharacterPrototype {
    pub inventory_generator: Box<dyn Generator<Inventory>>,
    pub species: Species,
    pub life_modifier: Option<LifeModifier>,
    pub has_inventory: bool,
}

pub fn random_character_generator() -> impl Generator<Character> {
    CharacterPrototype::random_species_character()
}

pub fn random_overloaded_character_generator() -> impl Generator<Character> {
    CharacterPrototype::random_species_overloaded()
}

pub fn species_character_generator(species: Species) -> impl Generator<Character> {
    basic_character(species)
}

pub fn overloaded_species_character_generator(species: Species) -> impl Generator<Character> {
    overloaded_character(species)
}

impl CharacterPrototype {
    pub fn basic_goblin() -> Self {
        basic_character(Species::Goblin)
    }

    pub fn undead_goblin() -> Self {
        undead_character(Species::Goblin)
    }

    pub fn overloaded_goblin() -> Self {
        overloaded_character(Species::Goblin)
    }

    pub fn basic_kobold() -> Self {
        basic_character(Species::Kobold)
    }

    pub fn undead_kobold() -> Self {
        undead_character(Species::Kobold)
    }

    pub fn overloaded_kobold() -> Self {
        overloaded_character(Species::Kobold)
    }

    pub fn overloaded_character(species: Species) -> Self {
        overloaded_character(species)
    }

    pub fn random_species_character() -> Self {
        let mut rng = rand::thread_rng();
        let all_species = vec![
            Species::Bugbear,
            Species::Goblin,
            Species::Kobold,
            Species::Ogre,
            Species::Orc,
            Species::Shadow,
        ];
        let index = rng.gen_range(0..all_species.len());
        let species = all_species.get(index).cloned().unwrap_or(Species::Shadow);

        basic_character(species)
    }

    pub fn random_species_overloaded() -> Self {
        let mut rng = rand::thread_rng();
        let all_species = vec![
            Species::Bugbear,
            Species::Goblin,
            Species::Kobold,
            Species::Ogre,
            Species::Orc,
            Species::Shadow,
        ];
        let index = rng.gen_range(0..all_species.len());
        let species = all_species.get(index).cloned().unwrap_or(Species::Shadow);

        overloaded_character(species)
    }
}

impl Generator<Character> for CharacterPrototype {
    fn generate(&self) -> Character {
        let inventory = if self.has_inventory {
            self.inventory_generator.generate()
        } else {
            Inventory::default()
        };

        let stats_generator = build_default_health_rolls(&self.species);
        let stats = stats_generator.generate();

        Character {
            stats,
            inventory,
            species: self.species.clone(),
            life_modifier: self.life_modifier.clone(),
            current_effects: Effects::default(),
            spell_memory: SpellMemory::default(),
        }
    }
}

#[cfg(test)]
mod goblin_tests {
    use crate::generators::generator::Generator;

    use super::CharacterPrototype;

    #[test]
    fn basic_goblin() {
        let prototype = CharacterPrototype::basic_goblin();
        let goblin = prototype.generate();
        assert!(!goblin.inventory.equipment.is_empty());
    }
}
