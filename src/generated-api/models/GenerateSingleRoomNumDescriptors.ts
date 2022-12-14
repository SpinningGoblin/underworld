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
 * @interface GenerateSingleRoomNumDescriptors
 */
export interface GenerateSingleRoomNumDescriptors {
    /**
     * 
     * @type {number}
     * @memberof GenerateSingleRoomNumDescriptors
     */
    min: number;
    /**
     * 
     * @type {number}
     * @memberof GenerateSingleRoomNumDescriptors
     */
    max_inclusive: number;
}

export function GenerateSingleRoomNumDescriptorsFromJSON(json: any): GenerateSingleRoomNumDescriptors {
    return GenerateSingleRoomNumDescriptorsFromJSONTyped(json, false);
}

export function GenerateSingleRoomNumDescriptorsFromJSONTyped(json: any, ignoreDiscriminator: boolean): GenerateSingleRoomNumDescriptors {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'min': json['min'],
        'max_inclusive': json['max_inclusive'],
    };
}

export function GenerateSingleRoomNumDescriptorsToJSON(value?: GenerateSingleRoomNumDescriptors | null): any {
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

