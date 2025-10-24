#[allow(warnings)]
mod bindings;

use bindings::exports::ardo314::math::{
    point2d::Point2d, point3d::Point3d, types::Guest, vector2d::Vector2d, vector3d::Vector3d,
};

struct Component;

impl Guest for Component {}

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
