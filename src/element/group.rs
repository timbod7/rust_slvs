use std::sync::atomic::{AtomicU32, Ordering};

use crate::binding;

static NEXT_GROUP_H: AtomicU32 = AtomicU32::new(1);

#[repr(C)]
pub struct GroupH(binding::Slvs_hGroup);

impl GroupH {
    fn new() -> Self {
        Self(NEXT_GROUP_H.fetch_add(1, Ordering::SeqCst))
    }
}

impl Default for GroupH {
    fn default() -> Self {
        Self::new()
    }
}

impl From<GroupH> for binding::Slvs_hGroup {
    fn from(value: GroupH) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::binding;

    use super::GroupH;

    #[test]
    fn incremental_handle_generated() {
        // handle starts from 1
        let mut g = GroupH::new();
        assert_eq!(binding::Slvs_hGroup::from(g), 1);

        // increments to 2
        g = GroupH::new();
        assert_eq!(binding::Slvs_hGroup::from(g), 2);

        // increments to 3
        g = GroupH::new();
        assert_eq!(binding::Slvs_hGroup::from(g), 3);
    }
}
