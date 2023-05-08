use serde::{Deserialize, Serialize};

use crate::{
    bindings::{SLVS_E_POINT_IN_2D, SLVS_E_POINT_IN_3D},
    element::TypeInfo,
};
use std::fmt::Debug;

pub trait AsTarget:
    Copy + TypeInfo + Send + Sync + Default + From<Vec<f64>> + Into<Vec<f64>>
{
    fn type_() -> i32;
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct OnWorkplane(pub f64, pub f64);

impl AsTarget for OnWorkplane {
    fn type_() -> i32 {
        SLVS_E_POINT_IN_2D as _
    }
}

impl TypeInfo for OnWorkplane {
    fn type_of() -> String {
        "OnWorkplane".to_string()
    }
}

impl From<Vec<f64>> for OnWorkplane {
    fn from(value: Vec<f64>) -> Self {
        Self(value[0], value[1])
    }
}

impl From<OnWorkplane> for Vec<f64> {
    fn from(value: OnWorkplane) -> Self {
        vec![value.0, value.1]
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct In3d(pub f64, pub f64, pub f64);

impl AsTarget for In3d {
    fn type_() -> i32 {
        SLVS_E_POINT_IN_3D as _
    }
}

impl TypeInfo for In3d {
    fn type_of() -> String {
        "In3d".to_string()
    }
}

impl From<Vec<f64>> for In3d {
    fn from(value: Vec<f64>) -> Self {
        Self(value[0], value[1], value[2])
    }
}

impl From<In3d> for Vec<f64> {
    fn from(value: In3d) -> Self {
        vec![value.0, value.1, value.2]
    }
}
