/* tslint:disable */
/* eslint-disable */
/**
 * Underworld
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { exists, mapValues } from '../runtime';
import {
    NonPlayer,
    NonPlayerFromJSON,
    NonPlayerFromJSONTyped,
    NonPlayerToJSON,
} from './NonPlayer';

/**
 * 
 * @export
 * @interface GeneratedNpc
 */
export interface GeneratedNpc {
    /**
     * 
     * @type {NonPlayer}
     * @memberof GeneratedNpc
     */
    non_player: NonPlayer;
    /**
     * 
     * @type {string}
     * @memberof GeneratedNpc
     */
    inventory_description: string;
    /**
     * 
     * @type {string}
     * @memberof GeneratedNpc
     */
    species_description: string;
}

export function GeneratedNpcFromJSON(json: any): GeneratedNpc {
    return GeneratedNpcFromJSONTyped(json, false);
}

export function GeneratedNpcFromJSONTyped(json: any, ignoreDiscriminator: boolean): GeneratedNpc {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'non_player': NonPlayerFromJSON(json['non_player']),
        'inventory_description': json['inventory_description'],
        'species_description': json['species_description'],
    };
}

export function GeneratedNpcToJSON(value?: GeneratedNpc | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'non_player': NonPlayerToJSON(value.non_player),
        'inventory_description': value.inventory_description,
        'species_description': value.species_description,
    };
}

