#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Object;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

use super::damage::{Attack, Defense};

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub struct Effects {
    pub shield_aura: Option<Defense>,
    pub retribution_aura: Option<Attack>,
    pub resurrection_aura: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Object), oai(rename_all = "snake_case"))]
pub struct EffectsView {
    pub shield_aura: Option<Defense>,
    pub knows_has_shield_aura: bool,
    pub retribution_aura: Option<Attack>,
    pub knows_has_retribution_aura: bool,
    pub resurrection_aura: bool,
    pub knows_has_resurrection_aura: bool,
}
