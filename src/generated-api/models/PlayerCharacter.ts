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

import { exists, mapValues } from '../runtime';
import {
    Character,
    CharacterFromJSON,
    CharacterFromJSONTyped,
    CharacterToJSON,
} from './Character';

/**
 * 
 * @export
 * @interface PlayerCharacter
 */
export interface PlayerCharacter {
    /**
     * 
     * @type {Character}
     * @memberof PlayerCharacter
     */
    character: Character;
    /**
     * 
     * @type {string}
     * @memberof PlayerCharacter
     */
    id: string;
    /**
     * 
     * @type {string}
     * @memberof PlayerCharacter
     */
    name?: string;
}

export function PlayerCharacterFromJSON(json: any): PlayerCharacter {
    return PlayerCharacterFromJSONTyped(json, false);
}

export function PlayerCharacterFromJSONTyped(json: any, ignoreDiscriminator: boolean): PlayerCharacter {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'character': CharacterFromJSON(json['character']),
        'id': json['id'],
        'name': !exists(json, 'name') ? undefined : json['name'],
    };
}

export function PlayerCharacterToJSON(value?: PlayerCharacter | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'character': CharacterToJSON(value.character),
        'id': value.id,
        'name': value.name,
    };
}

