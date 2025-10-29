use crate::{
    Component,
    bindings::exports::ardo314::math::{
        quaternion::Guest,
        types::{Quaternion, RotationVector, Vector3d},
        vector3d,
    },
};

impl Guest for Component {
    fn to_rotation_vector(q: Quaternion) -> RotationVector {
        let mut axis: Vector3d = (0.0, 0.0, 0.0);
        let angle = 2.0 * q.3.acos();
        if 1.0 - (q.3 * q.3) < 0.000001 {
            axis.0 = q.0;
            axis.1 = q.1;
            axis.2 = q.2;
        } else {
            // http://www.euclideanspace.com/maths/geometry/rotations/conversions/quaternionToAngle/
            let s = (1.0 - (q.3 * q.3)).sqrt();
            axis.0 = q.0 / s;
            axis.1 = q.1 / s;
            axis.2 = q.2 / s;
        }
        <Component as vector3d::Guest>::mul_f32(axis, angle)
    }
}
