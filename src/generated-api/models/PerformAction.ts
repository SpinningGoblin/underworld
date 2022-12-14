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
    ActionName,
    ActionNameFromJSON,
    ActionNameFromJSONTyped,
    ActionNameToJSON,
} from './ActionName';

/**
 * Actions, via a web call, that can be taken.
 * @export
 * @interface PerformAction
 */
export interface PerformAction {
    /**
     * 
     * @type {ActionName}
     * @memberof PerformAction
     */
    name: ActionName | null;
    /**
     * What the action does, in English.
     * @type {string}
     * @memberof PerformAction
     */
    description: string;
    /**
     * The web link to complete the action, to put onto the base url of the server.
     * @type {string}
     * @memberof PerformAction
     */
    link: string;
    /**
     * What HTTP action to use.
     * @type {string}
     * @memberof PerformAction
     */
    http_action: string;
    /**
     * Any required args for the action, as JSON.
     * @type {any}
     * @memberof PerformAction
     */
    args?: any | null;
}

export function PerformActionFromJSON(json: any): PerformAction {
    return PerformActionFromJSONTyped(json, false);
}

export function PerformActionFromJSONTyped(json: any, ignoreDiscriminator: boolean): PerformAction {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'name': ActionNameFromJSON(json['name']),
        'description': json['description'],
        'link': json['link'],
        'http_action': json['http_action'],
        'args': !exists(json, 'args') ? undefined : json['args'],
    };
}

export function PerformActionToJSON(value?: PerformAction | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'name': ActionNameToJSON(value.name),
        'description': value.description,
        'link': value.link,
        'http_action': value.http_action,
        'args': value.args,
    };
}

