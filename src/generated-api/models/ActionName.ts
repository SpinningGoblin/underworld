/* tslint:disable */
/* eslint-disable */
/**
 * Underworld
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.6.0
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
export const ActionName = {
    AttackNpc: 'attack_npc',
    CastSpellOnNpc: 'cast_spell_on_npc',
    CastSpellOnPlayer: 'cast_spell_on_player',
    CheckPlayerCharacter: 'check_player_character',
    ExitRoom: 'exit_room',
    InspectFixture: 'inspect_fixture',
    InspectNpc: 'inspect_npc',
    LookAtFixture: 'look_at_fixture',
    LookAtNpc: 'look_at_npc',
    LookAtRoom: 'look_at_room',
    LootFixture: 'loot_fixture',
    LootNpc: 'loot_npc',
    MovePlayerItem: 'move_player_item',
    OpenFixture: 'open_fixture',
    OpenFixtureHiddenCompartment: 'open_fixture_hidden_compartment',
    PickUpItem: 'pick_up_item',
    QuickLookRoom: 'quick_look_room',
    SellPlayerItem: 'sell_player_item',
    SetCurrentPlayerCharacter: 'set_current_player_character',
    ThrowItemAtNpc: 'throw_item_at_npc',
    UseItemOnPlayer: 'use_item_on_player'
} as const;
export type ActionName = typeof ActionName[keyof typeof ActionName];


export function ActionNameFromJSON(json: any): ActionName {
    return ActionNameFromJSONTyped(json, false);
}

export function ActionNameFromJSONTyped(json: any, ignoreDiscriminator: boolean): ActionName {
    return json as ActionName;
}

export function ActionNameToJSON(value?: ActionName | null): any {
    return value as any;
}

