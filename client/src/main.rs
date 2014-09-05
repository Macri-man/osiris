#![feature(phase)]
extern crate gfx;

#[phase(plugin)]
extern crate gfx_macros;
extern crate glfw;

#[vertex_format]
struct Vertex {
    #[as_float]
    #[name = "a_Pos"]
    pos: [i8, ..3],

    #[as_float]
    #[name = "a_TexCoord"]
    tex_coord: [u8, ..2],
}

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    
}
