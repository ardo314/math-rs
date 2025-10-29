#[allow(warnings)]
mod bindings;
mod point2d;
mod point3d;
mod rotation_vector;
mod vector2d;
mod vector3d;
mod vector4d;

struct Component;

bindings::export!(Component with_types_in bindings);
