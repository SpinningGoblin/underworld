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
    InclusiveRange,
    InclusiveRangeFromJSON,
    InclusiveRangeFromJSONTyped,
    InclusiveRangeToJSON,
} from './InclusiveRange';

/**
 * 
 * @export
 * @interface RoomFixturesGenerationArgsNumGroups
 */
export interface RoomFixturesGenerationArgsNumGroups {
    /**
     * 
     * @type {number}
     * @memberof RoomFixturesGenerationArgsNumGroups
     */
    min: number;
    /**
     * 
     * @type {number}
     * @memberof RoomFixturesGenerationArgsNumGroups
     */
    max_inclusive: number;
}

export function RoomFixturesGenerationArgsNumGroupsFromJSON(json: any): RoomFixturesGenerationArgsNumGroups {
    return RoomFixturesGenerationArgsNumGroupsFromJSONTyped(json, false);
}

export function RoomFixturesGenerationArgsNumGroupsFromJSONTyped(json: any, ignoreDiscriminator: boolean): RoomFixturesGenerationArgsNumGroups {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'min': json['min'],
        'max_inclusive': json['max_inclusive'],
    };
}

export function RoomFixturesGenerationArgsNumGroupsToJSON(value?: RoomFixturesGenerationArgsNumGroups | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'min': value.min,
        'max_inclusive': value.max_inclusive,
    };
}

