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
    ExitMap,
    ExitMapFromJSON,
    ExitMapFromJSONTyped,
    ExitMapToJSON,
} from './ExitMap';
import {
    Room,
    RoomFromJSON,
    RoomFromJSONTyped,
    RoomToJSON,
} from './Room';

/**
 * 
 * @export
 * @interface World
 */
export interface World {
    /**
     * 
     * @type {Array<Room>}
     * @memberof World
     */
    rooms: Array<Room>;
    /**
     * 
     * @type {Array<ExitMap>}
     * @memberof World
     */
    exit_graph: Array<ExitMap>;
}

export function WorldFromJSON(json: any): World {
    return WorldFromJSONTyped(json, false);
}

export function WorldFromJSONTyped(json: any, ignoreDiscriminator: boolean): World {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'rooms': ((json['rooms'] as Array<any>).map(RoomFromJSON)),
        'exit_graph': ((json['exit_graph'] as Array<any>).map(ExitMapFromJSON)),
    };
}

export function WorldToJSON(value?: World | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'rooms': ((value.rooms as Array<any>).map(RoomToJSON)),
        'exit_graph': ((value.exit_graph as Array<any>).map(ExitMapToJSON)),
    };
}

