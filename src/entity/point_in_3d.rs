use crate::bindings;

use super::AsEntity;

#[derive(Clone, Copy)]
pub struct PointIn3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl AsEntity for PointIn3d {
    fn type_(&self) -> i32 {
        bindings::SLVS_E_POINT_IN_3D as _
    }

    fn workplane(&self) -> Option<bindings::Slvs_hEntity> {
        None
    }

    fn point(&self) -> [Option<bindings::Slvs_hEntity>; 4] {
        [None; 4]
    }

    fn normal(&self) -> Option<bindings::Slvs_hEntity> {
        None
    }

    fn distance(&self) -> Option<bindings::Slvs_hEntity> {
        None
    }

    fn param_vals(&self) -> Option<Vec<f64>> {
        Some(vec![self.x, self.y, self.z])
    }
}
