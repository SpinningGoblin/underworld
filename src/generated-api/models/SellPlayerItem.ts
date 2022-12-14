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
/**
 * 
 * @export
 * @interface SellPlayerItem
 */
export interface SellPlayerItem {
    /**
     * 
     * @type {string}
     * @memberof SellPlayerItem
     */
    item_id: string;
}

export function SellPlayerItemFromJSON(json: any): SellPlayerItem {
    return SellPlayerItemFromJSONTyped(json, false);
}

export function SellPlayerItemFromJSONTyped(json: any, ignoreDiscriminator: boolean): SellPlayerItem {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'item_id': json['item_id'],
    };
}

export function SellPlayerItemToJSON(value?: SellPlayerItem | null): any {
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

