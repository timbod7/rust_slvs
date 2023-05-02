use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_Constraint, Slvs_hEntity, Slvs_hGroup, SLVS_C_ARC_LINE_TANGENT},
    element::{AsHandle, TypeInfo},
    entity::{ArcOfCircle, AsLineSegment, Entity, Workplane},
    group::Group,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ArcLineTangent<L: AsLineSegment> {
    pub group: Group,
    pub workplane: Entity<Workplane>,
    pub arc: Entity<ArcOfCircle>,
    pub line: Entity<L>,
    pub to_beginning: bool,
}

impl<L: AsLineSegment> ArcLineTangent<L> {
    pub fn new(
        group: Group,
        workplane: Entity<Workplane>,
        arc: Entity<ArcOfCircle>,
        line: Entity<L>,
        to_beginning: bool,
    ) -> Self {
        Self {
            group,
            workplane,
            arc,
            line,
            to_beginning,
        }
    }
}

impl<L: AsLineSegment> AsConstraintData for ArcLineTangent<L> {
    fn type_(&self) -> i32 {
        SLVS_C_ARC_LINE_TANGENT as _
    }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        Some(self.workplane.handle())
    }

    fn group(&self) -> Slvs_hGroup {
        self.group.handle()
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        Some(vec![self.arc.handle(), self.line.handle()])
    }

    fn others(&self) -> [bool; 2] {
        [self.to_beginning, false]
    }
}

impl<L: AsLineSegment> TypeInfo for ArcLineTangent<L> {
    fn type_of() -> String {
        format!("ArcLineTangent<{}>", L::type_of())
    }
}

impl<L: AsLineSegment> From<Slvs_Constraint> for ArcLineTangent<L> {
    fn from(value: Slvs_Constraint) -> Self {
        Self {
            group: Group(value.group),
            workplane: Entity::new(value.wrkpl),
            arc: Entity::new(value.entityA),
            line: Entity::new(value.entityB),
            to_beginning: value.other != 0,
        }
    }
}
