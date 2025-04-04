use crate::resources::{Secs, Time};
use serde::{Deserialize, Serialize};
use specs::{Component, DerefFlaggedStorage};
use std::time::Duration;
use vek::Vec3;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Object {
    DeleteAfter {
        spawned_at: Time,
        timeout: Duration,
    },
    Portal {
        target: Vec3<f32>,
        requires_no_aggro: bool,
        buildup_time: Secs,
    },
}

impl Component for Object {
    type Storage = DerefFlaggedStorage<Self, specs::VecStorage<Self>>;
}

#[derive(Clone, Debug)]
pub struct PortalData {
    pub target: Vec3<f32>,
    pub requires_no_aggro: bool,
    pub buildup_time: Secs,
}

impl From<PortalData> for Object {
    fn from(
        PortalData {
            target,
            requires_no_aggro,
            buildup_time,
        }: PortalData,
    ) -> Self {
        Self::Portal {
            target,
            requires_no_aggro,
            buildup_time,
        }
    }
}
