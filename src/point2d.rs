use crate::{
    Component,
    bindings::exports::ardo314::math::{
        point2d::Guest,
        types::{Point2d, Vector2d},
    },
};

impl Guest for Component {
    fn add_vector2d(p: Point2d, v: Vector2d) -> Point2d {
        (p.0 + v.0, p.1 + v.1)
    }

    fn sub_vector2d(p: Point2d, v: Vector2d) -> Point2d {
        (p.0 - v.0, p.1 - v.1)
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

    macro_rules! assert_vec2d_eq {
        ($left:expr, $right:expr, $tolerance:expr) => {
            assert_float_eq!($left.0, $right.0, $tolerance);
            assert_float_eq!($left.1, $right.1, $tolerance);
        };
    }

    const TOLERANCE: f32 = 1e-6;

    #[test]
    fn test_add_vector2d() {
        let point = (1.0, 2.0);
        let vector = (3.0, 4.0);
        let result = <Component as point2d::Guest>::add_vector2d(point, vector);
        assert_vec2d_eq!(result, (4.0, 6.0), TOLERANCE);
    }

    #[test]
    fn test_sub_vector2d() {
        let point = (5.0, 7.0);
        let vector = (2.0, 3.0);
        let result = <Component as point2d::Guest>::sub_vector2d(point, vector);
        assert_vec2d_eq!(result, (3.0, 4.0), TOLERANCE);
    }

    #[test]
    fn test_point_vector_relationships() {
        let point = (3.0, 4.0);
        let vector = (1.0, 1.0);

        // Test that adding a vector to a point and then subtracting it gives the original point
        let moved_point = <Component as point2d::Guest>::add_vector2d(point, vector);
        let back_to_original = <Component as point2d::Guest>::sub_vector2d(moved_point, vector);
        assert_vec2d_eq!(back_to_original, point, TOLERANCE);
    }
}
