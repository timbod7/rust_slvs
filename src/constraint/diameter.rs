use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_DIAMETER},
    element::{AsGroup, AsHandle, AsSlvsType, FromSystem},
    entity::{AsRadiused, EntityHandle},
    group::Group,
    System,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Diameter<R: AsRadiused> {
    pub group: Group,
    pub radius: EntityHandle<R>,
    pub diameter: f64,
}

impl<R: AsRadiused> Diameter<R> {
    fn new(group: Group, radius: EntityHandle<R>, diameter: f64) -> Self {
        Self {
            group,
            radius,
            diameter,
        }
    }
}

impl<R: AsRadiused> AsGroup for Diameter<R> {
    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }
}

impl<R: AsRadiused> AsSlvsType for Diameter<R> {
    fn slvs_type(&self) -> i32 {
        SLVS_C_DIAMETER as _
    }
}

impl<R: AsRadiused> AsConstraintData for Diameter<R> {
    fn workplane(&self) -> Option<Slvs_hEntity> {
        None
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.radius.handle()])
    }

    fn val(&self) -> Option<f64> {
        Some(self.diameter)
    }
}

impl<R: AsRadiused> FromSystem for Diameter<R> {
    fn from_system(sys: &System, element: &impl AsHandle) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let slvs_constraint = sys.slvs_constraint(element.handle())?;

        if SLVS_C_DIAMETER == slvs_constraint.type_ as _ {
            Ok(Self {
                group: Group(slvs_constraint.group),
                radius: EntityHandle::new(slvs_constraint.entityA),
                diameter: slvs_constraint.valA,
            })
        } else {
            Err("Expected constraint to have type SLVS_C_DIAMETER.")
        }
    }
}
