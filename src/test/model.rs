use typenum::U4;
use crate::{algebra::vector::Vectorf, pipeline::model::*, vectorf};
#[test]
fn load_from_obj(){
    let m = Model::from_obj("box.obj");
    assert_eq!(m.len(),  1);
    assert_eq!(m[0].indices().len(), 12);
    assert_eq!(m[0].vertexs().len(), 24);
    assert_eq!(m[0].vertexs()[0], vectorf!(U4; 1.0, 1.0, -1.0, 1.0));
}