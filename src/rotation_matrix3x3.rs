use crate::{
    Component,
    bindings::exports::ardo314::math::{
        rotation_matrix3x3::Guest,
        types::{AxisAngle, Quaternion, RotationVector},
    },
};

impl Guest for Component {
    fn to_axis_angle(rv: RotationVector) -> AxisAngle {
        todo!()
    }

    fn to_rotation_vector(aa: AxisAngle) -> RotationVector {
        todo!()
    }

    fn to_quaternion(rv: RotationVector) -> Quaternion {
        todo!()
    }
}
