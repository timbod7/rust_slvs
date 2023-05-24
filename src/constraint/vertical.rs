use serde::{Deserialize, Serialize};

use super::AsConstraintData;
use crate::{
    bindings::{Slvs_hEntity, Slvs_hGroup, SLVS_C_VERTICAL},
    element::{AsGroup, AsHandle, AsSlvsType},
    entity::{EntityHandle, LineSegment, Point, Workplane},
    group::Group,
};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Vertical {
    Points {
        group: Group,
        workplane: EntityHandle<Workplane>,
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
    },
    Line {
        group: Group,
        workplane: EntityHandle<Workplane>,
        line: EntityHandle<LineSegment>,
    },
}

impl Vertical {
    fn new_points(
        group: Group,
        workplane: EntityHandle<Workplane>,
        point_a: EntityHandle<Point>,
        point_b: EntityHandle<Point>,
    ) -> Self {
        Vertical::Points {
            group,
            workplane,
            point_a,
            point_b,
        }
    }

    fn new_line(
        group: Group,
        workplane: EntityHandle<Workplane>,
        line: EntityHandle<LineSegment>,
    ) -> Self {
        Vertical::Line {
            group,
            workplane,
            line,
        }
    }
}

impl AsGroup for Vertical {
    fn group(&self) -> Slvs_hGroup {
        match self {
            Vertical::Points { group, .. } => group.handle(),
            Vertical::Line { group, .. } => group.handle(),
        }
    }
}

impl AsSlvsType for Vertical {
    fn slvs_type(&self) -> i32 {
        SLVS_C_VERTICAL as _
    }
}

impl AsConstraintData for Vertical {
    // fn from_system(
    //     sys: &
    //     constraint_handle: &ConstraintHandle<Self>,
    // ) -> Result<Self, &'static str> {
    //     let slvs_constraint = sys.slvs_constraint(constraint_handle.handle())?;

    //     if let (Ok(point_a), Ok(point_b)) = (
    //         (*sys.slvs_entity(slvs_constraint.ptA)?).try_into(),
    //         (*sys.slvs_entity(slvs_constraint.ptB)?).try_into(),
    //     ) {
    //         Ok(Self::Points {
    //             group: Group(slvs_constraint.group),
    //             workplane: EntityHandle::new(slvs_constraint.wrkpl),
    //             point_a,
    //             point_b,
    //         })
    //     } else if let Ok(line) = (*sys.slvs_entity(slvs_constraint.entityA)?).try_into() {
    //         Ok(Self::Line {
    //             group: Group(slvs_constraint.group),
    //             workplane: EntityHandle::new(slvs_constraint.wrkpl),
    //             line,
    //         })
    //     } else {
    //         Err("Constraint should be of type Vertical")
    //     }
    // }

    fn workplane(&self) -> Option<Slvs_hEntity> {
        match self {
            Vertical::Points { workplane, .. } => Some(workplane.handle()),
            Vertical::Line { workplane, .. } => Some(workplane.handle()),
        }
    }

    fn points(&self) -> Option<Vec<Slvs_hEntity>> {
        match self {
            Vertical::Points {
                point_a, point_b, ..
            } => Some(vec![point_a.handle(), point_b.handle()]),
            Vertical::Line { .. } => None,
        }
    }

    fn entities(&self) -> Option<Vec<Slvs_hEntity>> {
        match self {
            Vertical::Points { .. } => None,
            Vertical::Line { line, .. } => Some(vec![line.handle()]),
        }
    }
}
