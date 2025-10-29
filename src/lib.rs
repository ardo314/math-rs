#[allow(warnings)]
mod bindings;
mod point2d;
mod point2d_test;
mod point3d;
mod pose2d;
mod pose3d;
mod quaternion;
mod rotation_vector;
mod vector2d;
mod vector3d;
mod vector4d;

struct Component;

bindings::export!(Component with_types_in bindings);
