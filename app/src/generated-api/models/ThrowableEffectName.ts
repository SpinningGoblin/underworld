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


/**
 * 
 * @export
 */
export const ThrowableEffectName = {
    OilSplash: 'oil_splash'
} as const;
export type ThrowableEffectName = typeof ThrowableEffectName[keyof typeof ThrowableEffectName];


export function ThrowableEffectNameFromJSON(json: any): ThrowableEffectName {
    return ThrowableEffectNameFromJSONTyped(json, false);
}

export function ThrowableEffectNameFromJSONTyped(json: any, ignoreDiscriminator: boolean): ThrowableEffectName {
    return json as ThrowableEffectName;
}

export function ThrowableEffectNameToJSON(value?: ThrowableEffectName | null): any {
    return value as any;
}

