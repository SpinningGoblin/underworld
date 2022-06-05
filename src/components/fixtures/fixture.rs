#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::components::{
    items::{
        descriptor::Descriptor,
        fixture_item::{FixtureItem, FixtureItemView},
    },
    material::Material,
    size::Size,
};

use super::fixture_type::FixtureType;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub struct Fixture {
    pub id: Uuid,
    pub name: Option<String>,
    pub fixture_type: FixtureType,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub material: Option<Material>,
    pub size: Size,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub descriptors: Vec<Descriptor>,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub items: Vec<FixtureItem>,
    pub has_hidden_compartment: bool,
    pub can_be_opened: bool,
    pub open: bool,
    pub hidden_compartment_open: bool,
}

impl Fixture {
    pub fn remove_item(&mut self, item_id: &Uuid) -> Option<FixtureItem> {
        let index = self
            .items
            .iter()
            .enumerate()
            .find(|(_, fixture_item)| fixture_item.item.id.eq(item_id))
            .map(|(index, _)| index);

        match index {
            Some(it) => Some(self.items.remove(it)),
            None => None,
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename = "Fixture"))]
pub struct FixtureView {
    pub id: String,
    pub name: Option<String>,
    pub fixture_type: FixtureType,
    #[cfg_attr(feature = "serialization", serde(default))]
    pub material: Option<Material>,
    pub size: Size,
    pub descriptors: Vec<Descriptor>,
    pub items: Vec<FixtureItemView>,
    pub knows_contained_items: bool,
    pub knows_hidden_compartment_items: bool,
    pub has_hidden_compartment: bool,
    pub knows_if_hidden_compartment: bool,
    pub open: bool,
    pub can_be_opened: bool,
    pub knows_if_can_be_opened: bool,
    pub hidden_compartment_open: bool,
}

#[derive(Clone, Debug, Default)]
pub struct FixtureViewArgs {
    pub knows_items: bool,
    pub knows_hidden: bool,
    pub knows_has_hidden: bool,
    pub knows_can_be_opened: bool,
}
