use crate::binding;

// pub type Constraint = binding::Slvs_Constraint;

impl From<binding::Slvs_Constraint> for binding::Slvs_hConstraint {
    fn from(value: binding::Slvs_Constraint) -> Self {
        value.h
    }
}

pub enum ConstraintType {
    pointsCoincident = binding::SLVS_C_POINTS_COINCIDENT as _,
    PtPtDistance = binding::SLVS_C_PT_PT_DISTANCE as _,
    PtPlaneDistance = binding::SLVS_C_PT_PLANE_DISTANCE as _,
    PtLineDistance = binding::SLVS_C_PT_LINE_DISTANCE as _,
    PtFaceDistance = binding::SLVS_C_PT_FACE_DISTANCE as _,
    PtInPlane = binding::SLVS_C_PT_IN_PLANE as _,
    PtOnLine = binding::SLVS_C_PT_ON_LINE as _,
    PtOnFace = binding::SLVS_C_PT_ON_FACE as _,
    EqualLengthLines = binding::SLVS_C_EQUAL_LENGTH_LINES as _,
    LengthRatio = binding::SLVS_C_LENGTH_RATIO as _,
    EqLenPtLineD = binding::SLVS_C_EQ_LEN_PT_LINE_D as _,
    EqPtLnDistances = binding::SLVS_C_EQ_PT_LN_DISTANCES as _,
    EqualAngle = binding::SLVS_C_EQUAL_ANGLE as _,
    EqualLineArcLen = binding::SLVS_C_EQUAL_LINE_ARC_LEN as _,
    Symmetric = binding::SLVS_C_SYMMETRIC as _,
    SymmetricHoriz = binding::SLVS_C_SYMMETRIC_HORIZ as _,
    SymmetricVert = binding::SLVS_C_SYMMETRIC_VERT as _,
    SymmetricLine = binding::SLVS_C_SYMMETRIC_LINE as _,
    AtMidpoint = binding::SLVS_C_AT_MIDPOINT as _,
    Horizontal = binding::SLVS_C_HORIZONTAL as _,
    Vertical = binding::SLVS_C_VERTICAL as _,
    Diameter = binding::SLVS_C_DIAMETER as _,
    PtOnCircle = binding::SLVS_C_PT_ON_CIRCLE as _,
    SameOrientation = binding::SLVS_C_SAME_ORIENTATION as _,
    Angle = binding::SLVS_C_ANGLE as _,
    Parallel = binding::SLVS_C_PARALLEL as _,
    Perpendicular = binding::SLVS_C_PERPENDICULAR as _,
    ArcLineTangent = binding::SLVS_C_ARC_LINE_TANGENT as _,
    CubicLineTangent = binding::SLVS_C_CUBIC_LINE_TANGENT as _,
    EqualRadius = binding::SLVS_C_EQUAL_RADIUS as _,
    ProjPtDistance = binding::SLVS_C_PROJ_PT_DISTANCE as _,
    WhereDragged = binding::SLVS_C_WHERE_DRAGGED as _,
    CurveCurveTangent = binding::SLVS_C_CURVE_CURVE_TANGENT as _,
    LengthDifference = binding::SLVS_C_LENGTH_DIFFERENCE as _,
    ArcArcLenRatio = binding::SLVS_C_ARC_ARC_LEN_RATIO as _,
    ArcLineLenRatio = binding::SLVS_C_ARC_LINE_LEN_RATIO as _,
    ArcArcDifference = binding::SLVS_C_ARC_ARC_DIFFERENCE as _,
    ArcLineDifference = binding::SLVS_C_ARC_LINE_DIFFERENCE as _,
}

pub trait AsConstraint {
    fn type_(&self) -> ConstraintType;
    fn wrkpl(&self) -> Option<binding::Slvs_hEntity>;
    fn valA(&self) -> f64;
    fn pt(&self) -> [Option<binding::Slvs_hEntity>; 2];
    fn entity(&self) -> [Option<binding::Slvs_hEntity>; 4];
    fn other(&self) -> [bool; 2];
}

pub struct PtPtDistance<'a> {
    val: f64,
    wrkpl: Option<&'a binding::Slvs_Entity>,
    pt_a: &'a binding::Slvs_Entity,
    pt_b: &'a binding::Slvs_Entity,
}

impl AsConstraint for PtPtDistance<'_> {
    fn type_(&self) -> ConstraintType {
        ConstraintType::PtPtDistance
    }
    fn wrkpl(&self) -> Option<binding::Slvs_hEntity> {
        self.wrkpl.map(|e| (*e).into())
    }
    fn valA(&self) -> f64 {
        self.val
    }
    fn pt(&self) -> [Option<binding::Slvs_hEntity>; 2] {
        [Some((*self.pt_a).into()), Some((*self.pt_b).into())]
    }
    fn entity(&self) -> [Option<binding::Slvs_hEntity>; 4] {
        [None; 4]
    }
    fn other(&self) -> [bool; 2] {
        [false; 2]
    }
}
