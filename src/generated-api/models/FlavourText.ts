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
export const FlavourText = {
    AStrangeBreezeBlows: 'a_strange_breeze_blows',
    IsSomethingWatchingYou: 'is_something_watching_you',
    MoldMossCoversWalls: 'mold_moss_covers_walls',
    SmellsLikeOldGoblinSocks: 'smells_like_old_goblin_socks',
    SomethingSquishyAllOverFloor: 'something_squishy_all_over_floor',
    UnseenLightsFlickerWalls: 'unseen_lights_flicker_walls',
    YouHearScratchingAllAroundYou: 'you_hear_scratching_all_around_you'
} as const;
export type FlavourText = typeof FlavourText[keyof typeof FlavourText];


export function FlavourTextFromJSON(json: any): FlavourText {
    return FlavourTextFromJSONTyped(json, false);
}

export function FlavourTextFromJSONTyped(json: any, ignoreDiscriminator: boolean): FlavourText {
    return json as FlavourText;
}

export function FlavourTextToJSON(value?: FlavourText | null): any {
    return value as any;
}

