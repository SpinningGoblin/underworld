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
/**
 * 
 * @export
 * @interface UseItemOnPlayer
 */
export interface UseItemOnPlayer {
    /**
     * 
     * @type {string}
     * @memberof UseItemOnPlayer
     */
    item_id: string;
}

export function UseItemOnPlayerFromJSON(json: any): UseItemOnPlayer {
    return UseItemOnPlayerFromJSONTyped(json, false);
}

export function UseItemOnPlayerFromJSONTyped(json: any, ignoreDiscriminator: boolean): UseItemOnPlayer {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'item_id': json['item_id'],
    };
}

export function UseItemOnPlayerToJSON(value?: UseItemOnPlayer | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'item_id': value.item_id,
    };
}
