/* tslint:disable */
/* eslint-disable */
/**
 * Underworld
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.5.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


/**
 * 
 * @export
 */
export const EventName = {
    DeadNpcBeaten: 'dead_npc_beaten',
    FixtureCanBeOpenedDiscovered: 'fixture_can_be_opened_discovered',
    FixtureContainedDiscovered: 'fixture_contained_discovered',
    FixtureHasHiddenDiscovered: 'fixture_has_hidden_discovered',
    FixtureHiddenItemsDiscovered: 'fixture_hidden_items_discovered',
    FixtureViewed: 'fixture_viewed',
    GameDangerLevelIncreased: 'game_danger_level_increased',
    ItemTakenFromFixture: 'item_taken_from_fixture',
    ItemTakenFromNpc: 'item_taken_from_npc',
    NpcHealthDiscovered: 'npc_health_discovered',
    NpcHiddenDiscovered: 'npc_hidden_discovered',
    NpcMissed: 'npc_missed',
    NpcPackedDiscovered: 'npc_packed_discovered',
    NpcViewed: 'npc_viewed',
    NpcWeaponReadied: 'npc_weapon_readied',
    PlayerGainsResurrectionAura: 'player_gains_resurrection_aura',
    PlayerGainsRetributionAura: 'player_gains_retribution_aura',
    PlayerGainsShieldAura: 'player_gains_shield_aura',
    PlayerHealed: 'player_healed',
    PlayerHit: 'player_hit',
    PlayerHitNpc: 'player_hit_npc',
    PlayerItemMoved: 'player_item_moved',
    PlayerItemRemoved: 'player_item_removed',
    PlayerItemUsed: 'player_item_used',
    PlayerKilled: 'player_killed',
    PlayerKilledNpc: 'player_killed_npc',
    PlayerMissed: 'player_missed',
    PlayerResurrected: 'player_resurrected',
    PlayerRetributionAuraDissipated: 'player_retribution_aura_dissipated',
    PlayerSpellForgotten: 'player_spell_forgotten',
    PlayerSpellLearned: 'player_spell_learned',
    PlayerSpellUsed: 'player_spell_used',
    RoomExited: 'room_exited',
    RoomFirstSeen: 'room_first_seen',
    RoomGenerated: 'room_generated'
} as const;
export type EventName = typeof EventName[keyof typeof EventName];


export function EventNameFromJSON(json: any): EventName {
    return EventNameFromJSONTyped(json, false);
}

export function EventNameFromJSONTyped(json: any, ignoreDiscriminator: boolean): EventName {
    return json as EventName;
}

export function EventNameToJSON(value?: EventName | null): any {
    return value as any;
}

