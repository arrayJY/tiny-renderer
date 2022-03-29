use crate::{Color, pipeline::model::Triangle, algebra::vector_new::Vector4};

type FrameBuffer = Vec<Option<Color>>;



pub fn pbr_shade(width: usize, height: usize, triangles: Vec<Triangle>) -> FrameBuffer {
    let mut framebuffer: FrameBuffer = vec![None; width*height];


    framebuffer
}

fn shade_pixel(x: (usize, usize), wo: &Vector4) {

    // Contribute from the light source.
    


    // Contribute from other reflectors.
    todo!()

}