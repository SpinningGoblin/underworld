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
    GameEvent,
    GameEventFromJSON,
    GameEventFromJSONTyped,
    GameEventToJSON,
} from './GameEvent';
import {
    PerformAction,
    PerformActionFromJSON,
    PerformActionFromJSONTyped,
    PerformActionToJSON,
} from './PerformAction';

/**
 * 
 * @export
 * @interface NpcInspected
 */
export interface NpcInspected {
    /**
     * 
     * @type {boolean}
     * @memberof NpcInspected
     */
    health_discovered: boolean;
    /**
     * 
     * @type {boolean}
     * @memberof NpcInspected
     */
    packed_items_discovered: boolean;
    /**
     * 
     * @type {boolean}
     * @memberof NpcInspected
     */
    hidden_items_discovered: boolean;
    /**
     * 
     * @type {Array<PerformAction>}
     * @memberof NpcInspected
     */
    actions: Array<PerformAction>;
    /**
     * 
     * @type {Array<GameEvent>}
     * @memberof NpcInspected
     */
    events: Array<GameEvent>;
}

export function NpcInspectedFromJSON(json: any): NpcInspected {
    return NpcInspectedFromJSONTyped(json, false);
}

export function NpcInspectedFromJSONTyped(json: any, ignoreDiscriminator: boolean): NpcInspected {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'health_discovered': json['health_discovered'],
        'packed_items_discovered': json['packed_items_discovered'],
        'hidden_items_discovered': json['hidden_items_discovered'],
        'actions': ((json['actions'] as Array<any>).map(PerformActionFromJSON)),
        'events': ((json['events'] as Array<any>).map(GameEventFromJSON)),
    };
}

export function NpcInspectedToJSON(value?: NpcInspected | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'health_discovered': value.health_discovered,
        'packed_items_discovered': value.packed_items_discovered,
        'hidden_items_discovered': value.hidden_items_discovered,
        'actions': ((value.actions as Array<any>).map(PerformActionToJSON)),
        'events': ((value.events as Array<any>).map(GameEventToJSON)),
    };
}
