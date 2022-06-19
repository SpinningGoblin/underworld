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
    Size,
    SizeFromJSON,
    SizeFromJSONTyped,
    SizeToJSON,
} from './Size';

/**
 * 
 * @export
 * @interface Dimensions
 */
export interface Dimensions {
    /**
     * 
     * @type {Size}
     * @memberof Dimensions
     */
    height: Size;
    /**
     * 
     * @type {Size}
     * @memberof Dimensions
     */
    width: Size;
    /**
     * 
     * @type {Size}
     * @memberof Dimensions
     */
    length: Size;
}

export function DimensionsFromJSON(json: any): Dimensions {
    return DimensionsFromJSONTyped(json, false);
}

export function DimensionsFromJSONTyped(json: any, ignoreDiscriminator: boolean): Dimensions {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'height': SizeFromJSON(json['height']),
        'width': SizeFromJSON(json['width']),
        'length': SizeFromJSON(json['length']),
    };
}

export function DimensionsToJSON(value?: Dimensions | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'height': SizeToJSON(value.height),
        'width': SizeToJSON(value.width),
        'length': SizeToJSON(value.length),
    };
}

