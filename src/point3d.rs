use crate::{
    Component,
    bindings::exports::ardo314::math::{
        point3d::Guest,
        types::{Point3d, Vector3d},
    },
};

impl Guest for Component {
    fn add_vector3d(p: Point3d, v: Vector3d) -> Point3d {
        (p.0 + v.0, p.1 + v.1, p.2 + v.2)
    }

    fn sub_vector3d(p: Point3d, v: Vector3d) -> Point3d {
        (p.0 - v.0, p.1 - v.1, p.2 - v.2)
    }
}

#[cfg(test)]
mod tests {
    use super::Component;
    use crate::bindings::exports::ardo314::math::{point2d, point3d, vector2d, vector3d};

    // Helper macro for floating point comparison with tolerance
    macro_rules! assert_float_eq {
        ($left:expr, $right:expr, $tolerance:expr) => {
            assert!(
                ($left - $right).abs() < $tolerance,
                "assertion failed: `(left == right)` (left: `{}`, right: `{}`, tolerance: `{}`)",
                $left,
                $right,
                $tolerance
            );
        };
    }

    macro_rules! assert_vec3d_eq {
        ($left:expr, $right:expr, $tolerance:expr) => {
            assert_float_eq!($left.0, $right.0, $tolerance);
            assert_float_eq!($left.1, $right.1, $tolerance);
            assert_float_eq!($left.2, $right.2, $tolerance);
        };
    }

    const TOLERANCE: f32 = 1e-6;

    #[test]
    fn test_point3d_add_vector3d() {
        let point = (1.0, 2.0, 3.0);
        let vector = (4.0, 5.0, 6.0);
        let result = <Component as point3d::Guest>::add_vector3d(point, vector);
        assert_vec3d_eq!(result, (5.0, 7.0, 9.0), TOLERANCE);
    }

    #[test]
    fn test_point3d_sub_vector3d() {
        let point = (7.0, 8.0, 9.0);
        let vector = (2.0, 3.0, 4.0);
        let result = <Component as point3d::Guest>::sub_vector3d(point, vector);
        assert_vec3d_eq!(result, (5.0, 5.0, 5.0), TOLERANCE);
    }

    #[test]
    fn test_point_vector_relationships() {
        let point_3d = (1.0, 2.0, 3.0);
        let vector_3d = (4.0, 5.0, 6.0);
        let moved_3d = <Component as point3d::Guest>::add_vector3d(point_3d, vector_3d);
        let back_3d = <Component as point3d::Guest>::sub_vector3d(moved_3d, vector_3d);
        assert_vec3d_eq!(back_3d, point_3d, TOLERANCE);
    }
}
