#[allow(warnings)]
mod bindings;

use bindings::exports::ardo314::math::{
    point2d::Point2d, point3d::Point3d, vector2d::Vector2d, vector3d::Vector3d,
};

struct Component;

impl bindings::exports::ardo314::math::vector2d::Guest for Component {
    fn add(lhs: Vector2d, rhs: Vector2d) -> Vector2d {
        (lhs.0 + rhs.0, lhs.1 + rhs.1)
    }

    fn sub(lhs: Vector2d, rhs: Vector2d) -> Vector2d {
        (lhs.0 - rhs.0, lhs.1 - rhs.1)
    }

    fn dot(lhs: Vector2d, rhs: Vector2d) -> f32 {
        lhs.0 * rhs.0 + lhs.1 * rhs.1
    }

    fn mul(lhs: Vector2d, rhs: f32) -> Vector2d {
        (lhs.0 * rhs, lhs.1 * rhs)
    }

    fn div(lhs: Vector2d, rhs: f32) -> Vector2d {
        (lhs.0 / rhs, lhs.1 / rhs)
    }

    fn neg(v: Vector2d) -> Vector2d {
        (-v.0, -v.1)
    }

    fn sqr_length(v: Vector2d) -> f32 {
        Self::dot(v, v)
    }

    fn length(v: Vector2d) -> f32 {
        Self::sqr_length(v).sqrt()
    }

    fn normalize(v: Vector2d) -> Vector2d {
        let len = Self::length(v);
        if len > 0.0 {
            (v.0 / len, v.1 / len)
        } else {
            (0.0, 0.0)
        }
    }
}

impl bindings::exports::ardo314::math::vector3d::Guest for Component {
    fn add(lhs: Vector3d, rhs: Vector3d) -> Vector3d {
        (lhs.0 + rhs.0, lhs.1 + rhs.1, lhs.2 + rhs.2)
    }

    fn sub(lhs: Vector3d, rhs: Vector3d) -> Vector3d {
        (lhs.0 - rhs.0, lhs.1 - rhs.1, lhs.2 - rhs.2)
    }

    fn dot(lhs: Vector3d, rhs: Vector3d) -> f32 {
        lhs.0 * rhs.0 + lhs.1 * rhs.1 + lhs.2 * rhs.2
    }

    fn mul(lhs: Vector3d, rhs: f32) -> Vector3d {
        (lhs.0 * rhs, lhs.1 * rhs, lhs.2 * rhs)
    }

    fn div(lhs: Vector3d, rhs: f32) -> Vector3d {
        (lhs.0 / rhs, lhs.1 / rhs, lhs.2 / rhs)
    }

    fn neg(v: Vector3d) -> Vector3d {
        (-v.0, -v.1, -v.2)
    }

    fn sqr_length(v: Vector3d) -> f32 {
        Self::dot(v, v)
    }

    fn length(v: Vector3d) -> f32 {
        Self::sqr_length(v).sqrt()
    }

    fn normalize(v: Vector3d) -> Vector3d {
        let len = Self::length(v);
        if len > 0.0 {
            (v.0 / len, v.1 / len, v.2 / len)
        } else {
            (0.0, 0.0, 0.0)
        }
    }
}

impl bindings::exports::ardo314::math::point2d::Guest for Component {
    fn add_vector2d(p: Point2d, v: bindings::exports::ardo314::math::point2d::Vector2d) -> Point2d {
        (p.0 + v.0, p.1 + v.1)
    }

    fn sub_vector2d(p: Point2d, v: bindings::exports::ardo314::math::point2d::Vector2d) -> Point2d {
        (p.0 - v.0, p.1 - v.1)
    }
}

impl bindings::exports::ardo314::math::point3d::Guest for Component {
    fn add_vector3d(p: Point3d, v: bindings::exports::ardo314::math::point3d::Vector3d) -> Point3d {
        (p.0 + v.0, p.1 + v.1, p.2 + v.2)
    }

    fn sub_vector3d(p: Point3d, v: bindings::exports::ardo314::math::point3d::Vector3d) -> Point3d {
        (p.0 - v.0, p.1 - v.1, p.2 - v.2)
    }
}

bindings::export!(Component with_types_in bindings);

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

    macro_rules! assert_vec3d_eq {
        ($left:expr, $right:expr, $tolerance:expr) => {
            assert_float_eq!($left.0, $right.0, $tolerance);
            assert_float_eq!($left.1, $right.1, $tolerance);
            assert_float_eq!($left.2, $right.2, $tolerance);
        };
    }

    const TOLERANCE: f32 = 1e-6;

    #[test]
    fn test_vector2d_add() {
        let v1 = (1.0, 2.0);
        let v2 = (3.0, 4.0);
        let result = <Component as vector2d::Guest>::add(v1, v2);
        assert_vec2d_eq!(result, (4.0, 6.0), TOLERANCE);
    }

    #[test]
    fn test_vector2d_sub() {
        let v1 = (5.0, 7.0);
        let v2 = (2.0, 3.0);
        let result = <Component as vector2d::Guest>::sub(v1, v2);
        assert_vec2d_eq!(result, (3.0, 4.0), TOLERANCE);
    }

    #[test]
    fn test_vector2d_dot() {
        let v1 = (1.0, 2.0);
        let v2 = (3.0, 4.0);
        let result = <Component as vector2d::Guest>::dot(v1, v2);
        assert_float_eq!(result, 11.0, TOLERANCE); // 1*3 + 2*4 = 11
    }

    #[test]
    fn test_vector2d_mul() {
        let v = (2.0, 3.0);
        let scalar = 2.5;
        let result = <Component as vector2d::Guest>::mul(v, scalar);
        assert_vec2d_eq!(result, (5.0, 7.5), TOLERANCE);
    }

    #[test]
    fn test_vector2d_div() {
        let v = (6.0, 8.0);
        let scalar = 2.0;
        let result = <Component as vector2d::Guest>::div(v, scalar);
        assert_vec2d_eq!(result, (3.0, 4.0), TOLERANCE);
    }

    #[test]
    fn test_vector2d_neg() {
        let v = (3.0, -4.0);
        let result = <Component as vector2d::Guest>::neg(v);
        assert_vec2d_eq!(result, (-3.0, 4.0), TOLERANCE);
    }

    #[test]
    fn test_vector2d_sqr_length() {
        let v = (3.0, 4.0);
        let result = <Component as vector2d::Guest>::sqr_length(v);
        assert_float_eq!(result, 25.0, TOLERANCE); // 3^2 + 4^2 = 25
    }

    #[test]
    fn test_vector2d_length() {
        let v = (3.0, 4.0);
        let result = <Component as vector2d::Guest>::length(v);
        assert_float_eq!(result, 5.0, TOLERANCE); // sqrt(25) = 5
    }

    #[test]
    fn test_vector2d_normalize() {
        let v = (3.0, 4.0);
        let result = <Component as vector2d::Guest>::normalize(v);
        assert_vec2d_eq!(result, (0.6, 0.8), TOLERANCE); // (3/5, 4/5)

        // Test that normalized vector has length 1
        let length = <Component as vector2d::Guest>::length(result);
        assert_float_eq!(length, 1.0, TOLERANCE);
    }

    #[test]
    fn test_vector2d_normalize_zero_vector() {
        let v = (0.0, 0.0);
        let result = <Component as vector2d::Guest>::normalize(v);
        assert_vec2d_eq!(result, (0.0, 0.0), TOLERANCE);
    }

    #[test]
    fn test_vector3d_add() {
        let v1 = (1.0, 2.0, 3.0);
        let v2 = (4.0, 5.0, 6.0);
        let result = <Component as vector3d::Guest>::add(v1, v2);
        assert_vec3d_eq!(result, (5.0, 7.0, 9.0), TOLERANCE);
    }

    #[test]
    fn test_vector3d_sub() {
        let v1 = (7.0, 8.0, 9.0);
        let v2 = (2.0, 3.0, 4.0);
        let result = <Component as vector3d::Guest>::sub(v1, v2);
        assert_vec3d_eq!(result, (5.0, 5.0, 5.0), TOLERANCE);
    }

    #[test]
    fn test_vector3d_dot() {
        let v1 = (1.0, 2.0, 3.0);
        let v2 = (4.0, 5.0, 6.0);
        let result = <Component as vector3d::Guest>::dot(v1, v2);
        assert_float_eq!(result, 32.0, TOLERANCE); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_vector3d_mul() {
        let v = (2.0, 3.0, 4.0);
        let scalar = 2.5;
        let result = <Component as vector3d::Guest>::mul(v, scalar);
        assert_vec3d_eq!(result, (5.0, 7.5, 10.0), TOLERANCE);
    }

    #[test]
    fn test_vector3d_div() {
        let v = (6.0, 8.0, 10.0);
        let scalar = 2.0;
        let result = <Component as vector3d::Guest>::div(v, scalar);
        assert_vec3d_eq!(result, (3.0, 4.0, 5.0), TOLERANCE);
    }

    #[test]
    fn test_vector3d_neg() {
        let v = (3.0, -4.0, 5.0);
        let result = <Component as vector3d::Guest>::neg(v);
        assert_vec3d_eq!(result, (-3.0, 4.0, -5.0), TOLERANCE);
    }

    #[test]
    fn test_vector3d_sqr_length() {
        let v = (2.0, 3.0, 6.0);
        let result = <Component as vector3d::Guest>::sqr_length(v);
        assert_float_eq!(result, 49.0, TOLERANCE); // 2^2 + 3^2 + 6^2 = 4 + 9 + 36 = 49
    }

    #[test]
    fn test_vector3d_length() {
        let v = (2.0, 3.0, 6.0);
        let result = <Component as vector3d::Guest>::length(v);
        assert_float_eq!(result, 7.0, TOLERANCE); // sqrt(49) = 7
    }

    #[test]
    fn test_vector3d_normalize() {
        let v = (3.0, 4.0, 0.0);
        let result = <Component as vector3d::Guest>::normalize(v);
        assert_vec3d_eq!(result, (0.6, 0.8, 0.0), TOLERANCE); // (3/5, 4/5, 0)

        // Test that normalized vector has length 1
        let length = <Component as vector3d::Guest>::length(result);
        assert_float_eq!(length, 1.0, TOLERANCE);
    }

    #[test]
    fn test_vector3d_normalize_zero_vector() {
        let v = (0.0, 0.0, 0.0);
        let result = <Component as vector3d::Guest>::normalize(v);
        assert_vec3d_eq!(result, (0.0, 0.0, 0.0), TOLERANCE);
    }

    #[test]
    fn test_point2d_add_vector2d() {
        let point = (1.0, 2.0);
        let vector = (3.0, 4.0);
        let result = <Component as point2d::Guest>::add_vector2d(point, vector);
        assert_vec2d_eq!(result, (4.0, 6.0), TOLERANCE);
    }

    #[test]
    fn test_point2d_sub_vector2d() {
        let point = (5.0, 7.0);
        let vector = (2.0, 3.0);
        let result = <Component as point2d::Guest>::sub_vector2d(point, vector);
        assert_vec2d_eq!(result, (3.0, 4.0), TOLERANCE);
    }

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

    // Integration tests combining multiple operations
    #[test]
    fn test_vector2d_operations_combination() {
        let v1 = (1.0, 0.0);
        let v2 = (0.0, 1.0);

        // Test perpendicular vectors dot product
        let dot_result = <Component as vector2d::Guest>::dot(v1, v2);
        assert_float_eq!(dot_result, 0.0, TOLERANCE);

        // Test adding and normalizing
        let sum = <Component as vector2d::Guest>::add(v1, v2);
        let normalized = <Component as vector2d::Guest>::normalize(sum);
        let expected_len = std::f32::consts::FRAC_1_SQRT_2;
        assert_vec2d_eq!(normalized, (expected_len, expected_len), TOLERANCE);
    }

    #[test]
    fn test_vector3d_operations_combination() {
        let v1 = (1.0, 0.0, 0.0);
        let v2 = (0.0, 1.0, 0.0);
        let v3 = (0.0, 0.0, 1.0);

        // Test orthogonal vectors
        assert_float_eq!(<Component as vector3d::Guest>::dot(v1, v2), 0.0, TOLERANCE);
        assert_float_eq!(<Component as vector3d::Guest>::dot(v1, v3), 0.0, TOLERANCE);
        assert_float_eq!(<Component as vector3d::Guest>::dot(v2, v3), 0.0, TOLERANCE);

        // Test that all unit vectors have length 1
        assert_float_eq!(<Component as vector3d::Guest>::length(v1), 1.0, TOLERANCE);
        assert_float_eq!(<Component as vector3d::Guest>::length(v2), 1.0, TOLERANCE);
        assert_float_eq!(<Component as vector3d::Guest>::length(v3), 1.0, TOLERANCE);
    }

    #[test]
    fn test_edge_cases() {
        // Test division by very small numbers (use larger tolerance for floating point precision)
        let v = (1.0, 1.0);
        let result = <Component as vector2d::Guest>::div(v, 0.001);
        assert_vec2d_eq!(result, (1000.0, 1000.0), 1e-3);

        // Test multiplication by zero
        let result = <Component as vector2d::Guest>::mul(v, 0.0);
        assert_vec2d_eq!(result, (0.0, 0.0), TOLERANCE);

        // Test negative scalars
        let result = <Component as vector3d::Guest>::mul((1.0, 2.0, 3.0), -2.0);
        assert_vec3d_eq!(result, (-2.0, -4.0, -6.0), TOLERANCE);
    }

    // Property-based tests
    #[test]
    fn test_vector_properties() {
        let v1 = (3.0, 4.0);
        let v2 = (1.0, 2.0);

        // Test commutativity of addition
        let sum1 = <Component as vector2d::Guest>::add(v1, v2);
        let sum2 = <Component as vector2d::Guest>::add(v2, v1);
        assert_vec2d_eq!(sum1, sum2, TOLERANCE);

        // Test commutativity of dot product
        let dot1 = <Component as vector2d::Guest>::dot(v1, v2);
        let dot2 = <Component as vector2d::Guest>::dot(v2, v1);
        assert_float_eq!(dot1, dot2, TOLERANCE);

        // Test that v - v = 0
        let zero = <Component as vector2d::Guest>::sub(v1, v1);
        assert_vec2d_eq!(zero, (0.0, 0.0), TOLERANCE);

        // Test that -(-v) = v
        let neg_v = <Component as vector2d::Guest>::neg(v1);
        let neg_neg_v = <Component as vector2d::Guest>::neg(neg_v);
        assert_vec2d_eq!(neg_neg_v, v1, TOLERANCE);
    }

    #[test]
    fn test_vector_algebra_properties() {
        let v1 = (2.0, 3.0);
        let v2 = (4.0, 1.0);
        let v3 = (1.0, 5.0);
        let scalar = 2.5;

        // Test associativity of addition: (v1 + v2) + v3 = v1 + (v2 + v3)
        let left =
            <Component as vector2d::Guest>::add(<Component as vector2d::Guest>::add(v1, v2), v3);
        let right =
            <Component as vector2d::Guest>::add(v1, <Component as vector2d::Guest>::add(v2, v3));
        assert_vec2d_eq!(left, right, TOLERANCE);

        // Test distributivity: scalar * (v1 + v2) = scalar * v1 + scalar * v2
        let left = <Component as vector2d::Guest>::mul(
            <Component as vector2d::Guest>::add(v1, v2),
            scalar,
        );
        let right = <Component as vector2d::Guest>::add(
            <Component as vector2d::Guest>::mul(v1, scalar),
            <Component as vector2d::Guest>::mul(v2, scalar),
        );
        assert_vec2d_eq!(left, right, TOLERANCE);

        // Test scalar multiplication associativity: (a * b) * v = a * (b * v)
        let scalar_a = 2.0;
        let scalar_b = 3.0;
        let left = <Component as vector2d::Guest>::mul(v1, scalar_a * scalar_b);
        let right = <Component as vector2d::Guest>::mul(
            <Component as vector2d::Guest>::mul(v1, scalar_b),
            scalar_a,
        );
        assert_vec2d_eq!(left, right, TOLERANCE);
    }

    #[test]
    fn test_3d_vector_algebra_properties() {
        let v1 = (1.0, 2.0, 3.0);
        let v2 = (4.0, 5.0, 6.0);
        let v3 = (7.0, 8.0, 9.0);

        // Test associativity of addition
        let left =
            <Component as vector3d::Guest>::add(<Component as vector3d::Guest>::add(v1, v2), v3);
        let right =
            <Component as vector3d::Guest>::add(v1, <Component as vector3d::Guest>::add(v2, v3));
        assert_vec3d_eq!(left, right, TOLERANCE);

        // Test bilinearity of dot product: (a*v1 + b*v2) · v3 = a*(v1·v3) + b*(v2·v3)
        let a = 2.0;
        let b = 3.0;
        let left_vec = <Component as vector3d::Guest>::add(
            <Component as vector3d::Guest>::mul(v1, a),
            <Component as vector3d::Guest>::mul(v2, b),
        );
        let left = <Component as vector3d::Guest>::dot(left_vec, v3);
        let right = a * <Component as vector3d::Guest>::dot(v1, v3)
            + b * <Component as vector3d::Guest>::dot(v2, v3);
        assert_float_eq!(left, right, TOLERANCE);
    }

    #[test]
    fn test_boundary_values() {
        // Test with very large numbers
        let large_v = (1e6, 1e6);
        let result = <Component as vector2d::Guest>::normalize(large_v);
        let expected = (
            std::f32::consts::FRAC_1_SQRT_2,
            std::f32::consts::FRAC_1_SQRT_2,
        );
        assert_vec2d_eq!(result, expected, 1e-5);

        // Test with very small numbers
        let small_v = (1e-6, 1e-6);
        let result = <Component as vector2d::Guest>::normalize(small_v);
        assert_vec2d_eq!(result, expected, 1e-5);

        // Test with mixed scales
        let mixed_v = (1e6, 1e-6);
        let result = <Component as vector2d::Guest>::normalize(mixed_v);
        let length = <Component as vector2d::Guest>::length(result);
        assert_float_eq!(length, 1.0, 1e-5);
    }

    #[test]
    fn test_point_vector_relationships() {
        let point = (3.0, 4.0);
        let vector = (1.0, 1.0);

        // Test that adding a vector to a point and then subtracting it gives the original point
        let moved_point = <Component as point2d::Guest>::add_vector2d(point, vector);
        let back_to_original = <Component as point2d::Guest>::sub_vector2d(moved_point, vector);
        assert_vec2d_eq!(back_to_original, point, TOLERANCE);

        // Test with 3D points
        let point_3d = (1.0, 2.0, 3.0);
        let vector_3d = (4.0, 5.0, 6.0);
        let moved_3d = <Component as point3d::Guest>::add_vector3d(point_3d, vector_3d);
        let back_3d = <Component as point3d::Guest>::sub_vector3d(moved_3d, vector_3d);
        assert_vec3d_eq!(back_3d, point_3d, TOLERANCE);
    }

    #[test]
    fn test_normalization_edge_cases() {
        // Test normalization of already normalized vector
        let unit_v = (1.0, 0.0);
        let result = <Component as vector2d::Guest>::normalize(unit_v);
        assert_vec2d_eq!(result, unit_v, TOLERANCE);

        // Test normalization of very small vector (but not zero)
        let tiny_v = (1e-10, 1e-10);
        let result = <Component as vector2d::Guest>::normalize(tiny_v);
        let length = <Component as vector2d::Guest>::length(result);
        // Should still normalize to unit length if not exactly zero
        if result.0 != 0.0 || result.1 != 0.0 {
            assert_float_eq!(length, 1.0, 1e-5);
        }
    }

    #[test]
    fn test_mathematical_constants() {
        // Test that sqrt(2) approximation is correct using vectors
        let v = (1.0, 1.0);
        let length = <Component as vector2d::Guest>::length(v);
        assert_float_eq!(length, std::f32::consts::SQRT_2, TOLERANCE);

        // Test with 3D diagonal vector
        let v3d = (1.0, 1.0, 1.0);
        let length_3d = <Component as vector3d::Guest>::length(v3d);
        assert_float_eq!(length_3d, 3.0_f32.sqrt(), TOLERANCE);
    }
}
