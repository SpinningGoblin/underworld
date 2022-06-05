use crate::components::{
    character::{Character, CharacterView, CharacterViewArgs},
    effects::EffectsView,
    spells::{learned_spell::LearnedSpellView, spell::SpellView, spell_memory::SpellMemoryView},
    stats::StatsView,
};

pub fn view(character: &Character, args: &CharacterViewArgs, knows_all: bool) -> CharacterView {
    let (health, health_known) = if args.knows_health || knows_all {
        (character.stats.health.clone(), true)
    } else {
        (None, false)
    };

    let (inventory, inventory_known) = if args.knows_inventory || knows_all {
        (
            Some(super::inventory::view(
                &character.inventory,
                args.knows_hidden_in_inventory,
                args.knows_packed_in_inventory,
                knows_all,
            )),
            true,
        )
    } else {
        (None, false)
    };

    let (spell_memory, spell_memory_known) = if knows_all {
        (
            Some(SpellMemoryView {
                spells: character
                    .spell_memory
                    .spells
                    .iter()
                    .map(|learned_spell| LearnedSpellView {
                        id: learned_spell.id.to_string(),
                        spell: SpellView {
                            name: learned_spell.spell.name.clone(),
                            attack: learned_spell.spell.attack.clone(),
                            knows_attack: true,
                            defense: learned_spell.spell.defense.clone(),
                            knows_defense: true,
                            uses: learned_spell.spell.uses,
                            knows_uses: true,
                        },
                        learned_at: learned_spell.learned_at.to_rfc3339(),
                    })
                    .collect(),
                knows_spells: true,
            }),
            true,
        )
    } else {
        (None, false)
    };

    let (current_effects, current_effects_known) = if knows_all {
        (
            Some(EffectsView {
                shield_aura: character.current_effects.shield_aura.clone(),
                knows_has_shield_aura: true,
                retribution_aura: character.current_effects.retribution_aura.clone(),
                knows_has_retribution_aura: true,
                resurrection_aura: character.current_effects.resurrection_aura,
                knows_has_resurrection_aura: true,
            }),
            true,
        )
    } else {
        (None, false)
    };

    CharacterView {
        stats: StatsView {
            health,
            health_known,
            height: character.stats.height.clone(),
        },
        species: character.species.clone(),
        life_modifier: character.life_modifier.clone(),
        inventory,
        inventory_known,
        spell_memory,
        spell_memory_known,
        current_effects,
        current_effects_known,
    }
}
