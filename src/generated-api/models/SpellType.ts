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


/**
 * 
 * @export
 */
export const SpellType = {
    Attack: 'attack',
    Healing: 'healing',
    PlayerEffect: 'player_effect'
} as const;
export type SpellType = typeof SpellType[keyof typeof SpellType];


export function SpellTypeFromJSON(json: any): SpellType {
    return SpellTypeFromJSONTyped(json, false);
}

export function SpellTypeFromJSONTyped(json: any, ignoreDiscriminator: boolean): SpellType {
    return json as SpellType;
}

export function SpellTypeToJSON(value?: SpellType | null): any {
    return value as any;
}

