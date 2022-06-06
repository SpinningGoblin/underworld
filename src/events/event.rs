#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::components::{
    games::game_state::GameState,
    items::{character_item::CharacterItem, location_tag::LocationTag},
    player::PlayerCharacter,
    spells::learned_spell::LearnedSpell,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case", tag = "event_type")
)]
pub enum Event {
    DeadNpcBeaten(super::DeadNpcBeaten),
    FixtureCanBeOpenedDiscovered(super::FixtureCanBeOpenedDiscovered),
    FixtureContainedDiscovered(super::FixtureContainedDiscovered),
    FixtureHasHiddenDiscovered(super::FixtureHasHiddenDiscovered),
    FixtureHiddenItemsDiscovered(super::FixtureHiddenItemsDiscovered),
    FixtureViewed(super::FixtureViewed),
    ItemTakenFromFixture(super::ItemTakenFromFixture),
    ItemTakenFromNpc(super::ItemTakenFromNpc),
    NpcHealthDiscovered(super::NpcHealthDiscovered),
    NpcHiddenDiscovered(super::NpcHiddenDiscovered),
    NpcMissed(super::NpcMissed),
    NpcPackedDiscovered(super::NpcPackedDiscovered),
    NpcViewed(super::NpcViewed),
    NpcWeaponReadied(super::NpcWeaponReadied),
    PlayerGainsResurrectionAura(super::PlayerGainsResurrectionAura),
    PlayerGainsRetributionAura(super::PlayerGainsRetributionAura),
    PlayerGainsShieldAura(super::PlayerGainsShieldAura),
    PlayerHealed(super::PlayerHealed),
    PlayerHit(super::PlayerHit),
    PlayerHitNpc(super::PlayerHitNpc),
    PlayerItemMoved(super::PlayerItemMoved),
    PlayerItemRemoved(super::PlayerItemRemoved),
    PlayerItemUsed(super::PlayerItemUsed),
    PlayerKilled(super::PlayerKilled),
    PlayerKilledNpc(super::PlayerKilledNpc),
    PlayerMissed(super::PlayerMissed),
    PlayerResurrected(super::PlayerResurrected),
    PlayerRetributionAuraDissipated(super::PlayerRetributionAuraDissipated),
    PlayerSpellForgotten(super::PlayerSpellForgotten),
    PlayerSpellLearned(super::PlayerSpellLearned),
    PlayerSpellUsed(super::PlayerSpellUsed),
    RoomExited(super::RoomExited),
    RoomGenerated(super::RoomGenerated),
    RoomFirstSeen(super::RoomFirstSeen),
}

pub fn apply_events(
    events: &[Event],
    state: &GameState,
    player: &PlayerCharacter,
) -> (GameState, PlayerCharacter) {
    let mut new_game = state.clone();
    let mut new_player = player.clone();

    for event in events.iter() {
        match event {
            Event::RoomExited(room_exited) => {
                new_game.current_room_id = room_exited.new_room_id;
            }
            Event::RoomGenerated(room_generated) => new_game
                .world
                .add_room(room_generated.entrance_id, room_generated.room.clone()),
            Event::PlayerHitNpc(npc_hit) => {
                if let Some(position) = new_game.current_room_mut().find_npc_mut(&npc_hit.npc_id) {
                    position.npc.character.damage(npc_hit.damage);
                }
                new_game.add_player_damage_done_to_stats(&npc_hit.attacker_id, npc_hit.damage);
            }
            Event::PlayerKilledNpc(npc_killed) => {
                let room = new_game.current_room_mut();
                if let Some(position) = room.find_npc_mut(&npc_killed.npc_id) {
                    position.npc.character.kill();
                    position.position_descriptor = None;
                }
                new_game.add_player_kill_to_stats(&npc_killed.killer_id);
            }
            Event::PlayerHit(player_hit) => {
                new_player.character.damage(player_hit.damage);
                new_game.add_player_damage_taken_to_stats(&player_hit.player_id, player_hit.damage);
            }
            Event::PlayerKilled(_) => new_player.character.kill(),
            Event::ItemTakenFromNpc(item_taken_from_npc) => take_item_from_npc(
                &mut new_game,
                &mut new_player,
                &item_taken_from_npc.npc_id,
                &item_taken_from_npc.item_id,
            ),
            Event::NpcWeaponReadied(weapon_readied) => ready_npc_weapon(
                &mut new_game,
                &weapon_readied.npc_id,
                &weapon_readied.item_id,
            ),
            Event::PlayerItemMoved(item_moved) => {
                let mut character_item = new_player
                    .character
                    .remove_item(&item_moved.item_id)
                    .unwrap();
                character_item.at_the_ready = item_moved.at_the_ready;
                character_item.equipped_location = item_moved.location.clone();
                new_player.character.add_item(character_item);
            }
            Event::NpcHealthDiscovered(health_discovered) => {
                let mut knowledge = new_game.npc_knowledge(&health_discovered.npc_id);
                knowledge.knows_health = true;
                new_game.set_npc_knowledge(health_discovered.npc_id, knowledge);
            }
            Event::NpcHiddenDiscovered(hidden_discovered) => {
                let mut knowledge = new_game.npc_knowledge(&hidden_discovered.npc_id);
                knowledge.knows_hidden_in_inventory = true;
                new_game.set_npc_knowledge(hidden_discovered.npc_id, knowledge);
            }
            Event::NpcPackedDiscovered(packed_discovered) => {
                let mut knowledge = new_game.npc_knowledge(&packed_discovered.npc_id);
                knowledge.knows_packed_in_inventory = true;
                new_game.set_npc_knowledge(packed_discovered.npc_id, knowledge);
            }
            Event::FixtureCanBeOpenedDiscovered(opened_discovered) => {
                let mut knowledge = new_game.fixture_knowledge(&opened_discovered.fixture_id);
                knowledge.knows_can_be_opened = true;
                new_game.set_fixture_knowledge(opened_discovered.fixture_id, knowledge);
            }
            Event::FixtureContainedDiscovered(contained_discovered) => {
                let mut knowledge = new_game.fixture_knowledge(&contained_discovered.fixture_id);
                knowledge.knows_items = true;
                new_game.set_fixture_knowledge(contained_discovered.fixture_id, knowledge);
            }
            Event::FixtureHasHiddenDiscovered(has_hidden) => {
                let mut knowledge = new_game.fixture_knowledge(&has_hidden.fixture_id);
                knowledge.knows_has_hidden = true;
                new_game.set_fixture_knowledge(has_hidden.fixture_id, knowledge);
            }
            Event::FixtureHiddenItemsDiscovered(hidden_items) => {
                let mut knowledge = new_game.fixture_knowledge(&hidden_items.fixture_id);
                knowledge.knows_hidden_items = true;
                new_game.set_fixture_knowledge(hidden_items.fixture_id, knowledge);
            }
            Event::RoomFirstSeen(first_seen) => {
                new_game.rooms_seen.push(first_seen.room_id);
            }
            Event::ItemTakenFromFixture(item_taken_from_fixture) => take_item_from_fixture(
                &mut new_game,
                &mut new_player,
                &item_taken_from_fixture.fixture_id,
                &item_taken_from_fixture.item_id,
            ),
            Event::PlayerHealed(player_healed) => {
                new_player.character.heal(player_healed.damage_healed)
            }
            Event::PlayerGainsResurrectionAura(_) => {
                new_player.character.current_effects.resurrection_aura = true;
            }
            Event::PlayerGainsRetributionAura(gain_retribution_aura) => {
                new_player.character.current_effects.retribution_aura =
                    Some(gain_retribution_aura.attack.clone())
            }
            Event::PlayerGainsShieldAura(gain_shield_aura) => {
                new_player.character.current_effects.shield_aura =
                    Some(gain_shield_aura.defense.clone())
            }
            Event::PlayerResurrected(_) => {
                new_player.character.heal_to_max();
                new_player.character.current_effects.resurrection_aura = false;
            }
            Event::PlayerRetributionAuraDissipated(_) => {
                new_player.character.current_effects.retribution_aura = None;
            }
            Event::PlayerSpellForgotten(player_spell_forgotten) => {
                new_player
                    .character
                    .forget_spell(&player_spell_forgotten.spell_id);
            }
            Event::PlayerSpellUsed(player_spell_used) => {
                if let Some(learned_spell) = new_player
                    .character
                    .find_spell_mut(&player_spell_used.spell_id)
                {
                    learned_spell.spell.uses -= 1;
                }
            }
            Event::PlayerItemRemoved(player_item_removed) => {
                new_player
                    .character
                    .remove_item(&player_item_removed.item_id);
            }
            Event::PlayerSpellLearned(player_spell_learned) => {
                new_player.character.spell_memory.add_spell(LearnedSpell {
                    id: player_spell_learned.spell_id,
                    spell: player_spell_learned.spell.clone(),
                    learned_at: player_spell_learned.learned_at,
                });
            }
            Event::PlayerItemUsed(player_item_used) => {
                let mut character_item =
                    match new_player.character.remove_item(&player_item_used.item_id) {
                        Some(it) => it,
                        None => continue,
                    };
                character_item.decrease_uses();
                new_player.character.add_item(character_item);
            }
            Event::NpcMissed(_)
            | Event::DeadNpcBeaten(_)
            | Event::PlayerMissed(_)
            | Event::NpcViewed(_)
            | Event::FixtureViewed(_) => {}
        }
    }

    (new_game, new_player)
}

fn take_item_from_fixture(
    new_game: &mut GameState,
    new_player: &mut PlayerCharacter,
    fixture_id: &Uuid,
    item_id: &Uuid,
) {
    let fixture_position = new_game
        .current_room_mut()
        .find_fixture_mut(fixture_id)
        .unwrap();
    let fixture_item = fixture_position.fixture.remove_item(item_id).unwrap();

    let packed_item = CharacterItem {
        is_hidden: false,
        equipped_location: LocationTag::Packed,
        is_multiple: false,
        item: fixture_item.item,
        at_the_ready: false,
    };
    new_player.character.add_item(packed_item)
}

fn take_item_from_npc(
    new_game: &mut GameState,
    new_player: &mut PlayerCharacter,
    npc_id: &Uuid,
    item_id: &Uuid,
) {
    let position = new_game.current_room_mut().find_npc_mut(npc_id).unwrap();
    let character_item = position.npc.character.remove_item(item_id).unwrap();

    let packed_item = CharacterItem {
        is_hidden: false,
        equipped_location: LocationTag::Packed,
        is_multiple: character_item.is_multiple,
        item: character_item.item,
        at_the_ready: false,
    };
    new_player.character.add_item(packed_item);
}

fn ready_npc_weapon(new_game: &mut GameState, npc_id: &Uuid, item_id: &Uuid) {
    let position = new_game.current_room_mut().find_npc_mut(npc_id).unwrap();
    let mut character_item = position.npc.character.remove_item(item_id).unwrap();
    character_item.at_the_ready = true;
    character_item.equipped_location = LocationTag::Hand;
    position.npc.character.add_item(character_item);
}
