use crate::{
    Component,
    bindings::exports::ardo314::math::{
        axis_angle,
        quaternion::Guest,
        types::{AxisAngle, Matrix3x3, Quaternion, RotationVector, Vector3d},
    },
};

impl Guest for Component {
    fn identity() -> Quaternion {
        (0.0, 0.0, 0.0, 1.0)
    }

    fn to_rotation_vector(q: Quaternion) -> RotationVector {
        let aa = Self::to_axis_angle(q);
        <Component as axis_angle::Guest>::to_rotation_vector(aa)
    }

    fn to_axis_angle(q: Quaternion) -> AxisAngle {
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
        (axis, angle)
    }

    fn to_matrix3x3(q: Quaternion) -> Matrix3x3 {
        let x = q.0;
        let y = q.1;
        let z = q.2;
        let w = q.3;
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;
        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;

        te[0] = (1 - (yy + zz));
        te[1] = (xy + wz);
        te[2] = (xz - wy);
        te[3] = 0;

        te[4] = (xy - wz);
        te[5] = (1 - (xx + zz));
        te[6] = (yz + wx);
        te[7] = 0;

        te[8] = (xz + wy);
        te[9] = (yz - wx);
        te[10] = (1 - (xx + yy));
        te[11] = 0;

        Matrix3x3 {
            m00: (),
            m10: (),
            m20: (),
            m01: (),
            m11: (),
            m21: (),
            m02: (),
            m12: (),
            m22: (),
        }
    }
}
