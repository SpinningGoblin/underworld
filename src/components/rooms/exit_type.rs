#[cfg(feature = "bevy_components")]
use bevy_ecs::prelude::Component;
#[cfg(feature = "openapi")]
use poem_openapi::Enum;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Debug, PartialEq, EnumIter)]
#[cfg_attr(feature = "bevy_components", derive(Component))]
#[cfg_attr(
    feature = "serialization",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "openapi", derive(Enum), oai(rename_all = "snake_case"))]
pub enum ExitType {
    Door,
    HoleInTheWall,
    OpeningToTheVoid,
    HoleInTheFloor,
    StaircaseUp,
    StaircaseDown,
    Hallway,
    DugOutTunnelEntrance,
}
