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

import { exists, mapValues } from '../runtime';
import {
    CharacterKnowledge,
    CharacterKnowledgeFromJSON,
    CharacterKnowledgeFromJSONTyped,
    CharacterKnowledgeToJSON,
} from './CharacterKnowledge';
import {
    FixtureKnowledge,
    FixtureKnowledgeFromJSON,
    FixtureKnowledgeFromJSONTyped,
    FixtureKnowledgeToJSON,
} from './FixtureKnowledge';
import {
    Statistics,
    StatisticsFromJSON,
    StatisticsFromJSONTyped,
    StatisticsToJSON,
} from './Statistics';
import {
    World,
    WorldFromJSON,
    WorldFromJSONTyped,
    WorldToJSON,
} from './World';

/**
 * 
 * @export
 * @interface GameState
 */
export interface GameState {
    /**
     * 
     * @type {string}
     * @memberof GameState
     */
    id: string;
    /**
     * 
     * @type {string}
     * @memberof GameState
     */
    name?: string;
    /**
     * 
     * @type {World}
     * @memberof GameState
     */
    world: World;
    /**
     * 
     * @type {string}
     * @memberof GameState
     */
    current_room_id: string;
    /**
     * 
     * @type {Array<string>}
     * @memberof GameState
     */
    rooms_seen: Array<string>;
    /**
     * 
     * @type {boolean}
     * @memberof GameState
     */
    all_knowledge_unlocked: boolean;
    /**
     * 
     * @type {{ [key: string]: CharacterKnowledge; }}
     * @memberof GameState
     */
    player_npc_knowledge: { [key: string]: CharacterKnowledge; };
    /**
     * 
     * @type {{ [key: string]: FixtureKnowledge; }}
     * @memberof GameState
     */
    player_fixture_knowledge: { [key: string]: FixtureKnowledge; };
    /**
     * 
     * @type {{ [key: string]: Statistics; }}
     * @memberof GameState
     */
    player_statistics: { [key: string]: Statistics; };
    /**
     * 
     * @type {number}
     * @memberof GameState
     */
    danger_level: number;
}

export function GameStateFromJSON(json: any): GameState {
    return GameStateFromJSONTyped(json, false);
}

export function GameStateFromJSONTyped(json: any, ignoreDiscriminator: boolean): GameState {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'id': json['id'],
        'name': !exists(json, 'name') ? undefined : json['name'],
        'world': WorldFromJSON(json['world']),
        'current_room_id': json['current_room_id'],
        'rooms_seen': json['rooms_seen'],
        'all_knowledge_unlocked': json['all_knowledge_unlocked'],
        'player_npc_knowledge': (mapValues(json['player_npc_knowledge'], CharacterKnowledgeFromJSON)),
        'player_fixture_knowledge': (mapValues(json['player_fixture_knowledge'], FixtureKnowledgeFromJSON)),
        'player_statistics': (mapValues(json['player_statistics'], StatisticsFromJSON)),
        'danger_level': json['danger_level'],
    };
}

export function GameStateToJSON(value?: GameState | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'id': value.id,
        'name': value.name,
        'world': WorldToJSON(value.world),
        'current_room_id': value.current_room_id,
        'rooms_seen': value.rooms_seen,
        'all_knowledge_unlocked': value.all_knowledge_unlocked,
        'player_npc_knowledge': (mapValues(value.player_npc_knowledge, CharacterKnowledgeToJSON)),
        'player_fixture_knowledge': (mapValues(value.player_fixture_knowledge, FixtureKnowledgeToJSON)),
        'player_statistics': (mapValues(value.player_statistics, StatisticsToJSON)),
        'danger_level': value.danger_level,
    };
}

