use crate::{
    Component,
    bindings::exports::ardo314::math::{rotation_matrix2x2::Guest, types::Matrix2x2},
};

impl Guest for Component {
    fn to_angle(m: Matrix2x2) -> f32 {
        todo!()
    }

    fn from_angle(a: f32) -> Matrix2x2 {
        todo!()
    }
}
