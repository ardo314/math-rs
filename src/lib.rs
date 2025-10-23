#[allow(warnings)]
mod bindings;

use bindings::exports::ardo314::math::{
    point2d::Point2d, point3d::Point3d, types::Guest, vector2d::Vector2d, vector3d::Vector3d,
};

struct Component;

impl Guest for Component {}

impl bindings::exports::ardo314::math::vector2d::Guest for Component {
    fn add(lhs: Vector2d, rhs: Vector2d) -> Vector2d {
        unimplemented!()
    }

    fn sub(lhs: Vector2d, rhs: Vector2d) -> Vector2d {
        unimplemented!()
    }

    fn dot(lhs: Vector2d, rhs: Vector2d) -> f32 {
        unimplemented!()
    }

    fn mul(lhs: Vector2d, rhs: f32) -> Vector2d {
        unimplemented!()
    }

    fn div(lhs: Vector2d, rhs: f32) -> Vector2d {
        unimplemented!()
    }

    fn neg(v: Vector2d) -> Vector2d {
        unimplemented!()
    }

    fn sqr_length(v: Vector2d) -> f32 {
        unimplemented!()
    }

    fn length(v: Vector2d) -> f32 {
        unimplemented!()
    }

    fn normalize(v: Vector2d) -> Vector2d {
        unimplemented!()
    }
}

impl bindings::exports::ardo314::math::vector3d::Guest for Component {
    fn add(lhs: Vector3d, rhs: Vector3d) -> Vector3d {
        unimplemented!()
    }

    fn sub(lhs: Vector3d, rhs: Vector3d) -> Vector3d {
        unimplemented!()
    }

    fn dot(lhs: Vector3d, rhs: Vector3d) -> f32 {
        unimplemented!()
    }

    fn mul(lhs: Vector3d, rhs: f32) -> Vector3d {
        unimplemented!()
    }

    fn div(lhs: Vector3d, rhs: f32) -> Vector3d {
        unimplemented!()
    }

    fn neg(v: Vector3d) -> Vector3d {
        unimplemented!()
    }

    fn sqr_length(v: Vector3d) -> f32 {
        unimplemented!()
    }

    fn length(v: Vector3d) -> f32 {
        unimplemented!()
    }

    fn normalize(v: Vector3d) -> Vector3d {
        unimplemented!()
    }
}

impl bindings::exports::ardo314::math::point2d::Guest for Component {
    fn add_vector2d(p: Point2d, v: bindings::exports::ardo314::math::point2d::Vector2d) -> Point2d {
        unimplemented!()
    }

    fn sub_vector2d(p: Point2d, v: bindings::exports::ardo314::math::point2d::Vector2d) -> Point2d {
        unimplemented!()
    }
}

impl bindings::exports::ardo314::math::point3d::Guest for Component {
    fn add_vector3d(p: Point3d, v: bindings::exports::ardo314::math::point3d::Vector3d) -> Point3d {
        unimplemented!()
    }

    fn sub_vector3d(p: Point3d, v: bindings::exports::ardo314::math::point3d::Vector3d) -> Point3d {
        unimplemented!()
    }
}

bindings::export!(Component with_types_in bindings);
