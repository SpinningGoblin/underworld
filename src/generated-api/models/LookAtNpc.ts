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
 * Look at the specified NPC, NPC is viewed using what player
 * currently knows about NPC.
 * @export
 * @interface LookAtNpc
 */
export interface LookAtNpc {
    /**
     * 
     * @type {string}
     * @memberof LookAtNpc
     */
    npc_id: string;
}

export function LookAtNpcFromJSON(json: any): LookAtNpc {
    return LookAtNpcFromJSONTyped(json, false);
}

export function LookAtNpcFromJSONTyped(json: any, ignoreDiscriminator: boolean): LookAtNpc {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'npc_id': json['npc_id'],
    };
}

export function LookAtNpcToJSON(value?: LookAtNpc | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'npc_id': value.npc_id,
    };
}

