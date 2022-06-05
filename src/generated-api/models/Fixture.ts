/* tslint:disable */
/* eslint-disable */
/**
 * Underworld
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { exists, mapValues } from '../runtime';
import {
    FixtureItem,
    FixtureItemFromJSON,
    FixtureItemFromJSONTyped,
    FixtureItemToJSON,
} from './FixtureItem';
import {
    FixtureType,
    FixtureTypeFromJSON,
    FixtureTypeFromJSONTyped,
    FixtureTypeToJSON,
} from './FixtureType';
import {
    ItemDescriptor,
    ItemDescriptorFromJSON,
    ItemDescriptorFromJSONTyped,
    ItemDescriptorToJSON,
} from './ItemDescriptor';
import {
    Material,
    MaterialFromJSON,
    MaterialFromJSONTyped,
    MaterialToJSON,
} from './Material';
import {
    Size,
    SizeFromJSON,
    SizeFromJSONTyped,
    SizeToJSON,
} from './Size';

/**
 * 
 * @export
 * @interface Fixture
 */
export interface Fixture {
    /**
     * 
     * @type {string}
     * @memberof Fixture
     */
    id: string;
    /**
     * 
     * @type {string}
     * @memberof Fixture
     */
    name?: string;
    /**
     * 
     * @type {FixtureType}
     * @memberof Fixture
     */
    fixture_type: FixtureType;
    /**
     * 
     * @type {Material}
     * @memberof Fixture
     */
    material?: Material;
    /**
     * 
     * @type {Size}
     * @memberof Fixture
     */
    size: Size;
    /**
     * 
     * @type {Array<ItemDescriptor>}
     * @memberof Fixture
     */
    descriptors: Array<ItemDescriptor>;
    /**
     * 
     * @type {Array<FixtureItem>}
     * @memberof Fixture
     */
    items: Array<FixtureItem>;
    /**
     * 
     * @type {boolean}
     * @memberof Fixture
     */
    knows_contained_items: boolean;
    /**
     * 
     * @type {boolean}
     * @memberof Fixture
     */
    knows_hidden_compartment_items: boolean;
    /**
     * 
     * @type {boolean}
     * @memberof Fixture
     */
    has_hidden_compartment: boolean;
    /**
     * 
     * @type {boolean}
     * @memberof Fixture
     */
    knows_if_hidden_compartment: boolean;
    /**
     * 
     * @type {boolean}
     * @memberof Fixture
     */
    open: boolean;
    /**
     * 
     * @type {boolean}
     * @memberof Fixture
     */
    can_be_opened: boolean;
    /**
     * 
     * @type {boolean}
     * @memberof Fixture
     */
    knows_if_can_be_opened: boolean;
    /**
     * 
     * @type {boolean}
     * @memberof Fixture
     */
    hidden_compartment_open: boolean;
}

export function FixtureFromJSON(json: any): Fixture {
    return FixtureFromJSONTyped(json, false);
}

export function FixtureFromJSONTyped(json: any, ignoreDiscriminator: boolean): Fixture {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'id': json['id'],
        'name': !exists(json, 'name') ? undefined : json['name'],
        'fixture_type': FixtureTypeFromJSON(json['fixture_type']),
        'material': !exists(json, 'material') ? undefined : MaterialFromJSON(json['material']),
        'size': SizeFromJSON(json['size']),
        'descriptors': ((json['descriptors'] as Array<any>).map(ItemDescriptorFromJSON)),
        'items': ((json['items'] as Array<any>).map(FixtureItemFromJSON)),
        'knows_contained_items': json['knows_contained_items'],
        'knows_hidden_compartment_items': json['knows_hidden_compartment_items'],
        'has_hidden_compartment': json['has_hidden_compartment'],
        'knows_if_hidden_compartment': json['knows_if_hidden_compartment'],
        'open': json['open'],
        'can_be_opened': json['can_be_opened'],
        'knows_if_can_be_opened': json['knows_if_can_be_opened'],
        'hidden_compartment_open': json['hidden_compartment_open'],
    };
}

export function FixtureToJSON(value?: Fixture | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'id': value.id,
        'name': value.name,
        'fixture_type': FixtureTypeToJSON(value.fixture_type),
        'material': MaterialToJSON(value.material),
        'size': SizeToJSON(value.size),
        'descriptors': ((value.descriptors as Array<any>).map(ItemDescriptorToJSON)),
        'items': ((value.items as Array<any>).map(FixtureItemToJSON)),
        'knows_contained_items': value.knows_contained_items,
        'knows_hidden_compartment_items': value.knows_hidden_compartment_items,
        'has_hidden_compartment': value.has_hidden_compartment,
        'knows_if_hidden_compartment': value.knows_if_hidden_compartment,
        'open': value.open,
        'can_be_opened': value.can_be_opened,
        'knows_if_can_be_opened': value.knows_if_can_be_opened,
        'hidden_compartment_open': value.hidden_compartment_open,
    };
}

