use crate::{
    Component,
    bindings::exports::ardo314::math::{
        rotation_vector::Guest,
        types::{Quaternion, RotationVector, Vector3d},
    },
};

fn length(rv: RotationVector) -> f32 {
    (rv.0 * rv.0 + rv.1 * rv.1 + rv.2 * rv.2).sqrt()
}

fn div_f32(lhs: RotationVector, rhs: f32) -> RotationVector {
    (lhs.0 / rhs, lhs.1 / rhs, lhs.2 / rhs)
}

fn to_components(rv: RotationVector) -> (Vector3d, f32) {
    let angle = length(rv);
    let axis = if angle > 0.0 {
        div_f32(rv, angle)
    } else {
        (0.0, 0.0, 0.0)
    };
    (axis, angle)
}

impl Guest for Component {
    fn axis(rv: RotationVector) -> Vector3d {
        let (axis, _angle) = to_components(rv);
        axis
    }

    fn angle(rv: RotationVector) -> f32 {
        length(rv)
    }

    fn to_quaternion(rv: RotationVector) -> Quaternion {
        let (axis, angle) = to_components(rv);
        let half_angle = angle / 2.0;
        let s = half_angle.sin();

        let x = axis.0 * s;
        let y = axis.1 * s;
        let z = axis.2 * s;
        let w = half_angle.cos();
        (x, y, z, w)
    }
}
