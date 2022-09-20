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
    FixtureType,
    FixtureTypeFromJSON,
    FixtureTypeFromJSONTyped,
    FixtureTypeToJSON,
} from './FixtureType';
import {
    RoomFixturesGenerationArgsNumGroups,
    RoomFixturesGenerationArgsNumGroupsFromJSON,
    RoomFixturesGenerationArgsNumGroupsFromJSONTyped,
    RoomFixturesGenerationArgsNumGroupsToJSON,
} from './RoomFixturesGenerationArgsNumGroups';

/**
 * Args to modify the fixture generation inside of the room.
 * @export
 * @interface RoomFixturesGenerationArgs
 */
export interface RoomFixturesGenerationArgs {
    /**
     * 
     * @type {RoomFixturesGenerationArgsNumGroups}
     * @memberof RoomFixturesGenerationArgs
     */
    num_groups?: RoomFixturesGenerationArgsNumGroups;
    /**
     * If you want to limit the fixture types that can be spawned,
     * set them here. Otherwise they will be chosen by the room
     * type that is being used.
     * @type {Array<FixtureType>}
     * @memberof RoomFixturesGenerationArgs
     */
    possible_fixture_types?: Array<FixtureType>;
}

export function RoomFixturesGenerationArgsFromJSON(json: any): RoomFixturesGenerationArgs {
    return RoomFixturesGenerationArgsFromJSONTyped(json, false);
}

export function RoomFixturesGenerationArgsFromJSONTyped(json: any, ignoreDiscriminator: boolean): RoomFixturesGenerationArgs {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'num_groups': !exists(json, 'num_groups') ? undefined : RoomFixturesGenerationArgsNumGroupsFromJSON(json['num_groups']),
        'possible_fixture_types': !exists(json, 'possible_fixture_types') ? undefined : ((json['possible_fixture_types'] as Array<any>).map(FixtureTypeFromJSON)),
    };
}

export function RoomFixturesGenerationArgsToJSON(value?: RoomFixturesGenerationArgs | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'num_groups': RoomFixturesGenerationArgsNumGroupsToJSON(value.num_groups),
        'possible_fixture_types': value.possible_fixture_types === undefined ? undefined : ((value.possible_fixture_types as Array<any>).map(FixtureTypeToJSON)),
    };
}
