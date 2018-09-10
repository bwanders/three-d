use gl;
use mesh;
use core::surface;
use core::program;

pub fn render(gl: &gl::Gl, program: &program::Program)
{
    unsafe {
        static mut FULL_SCREEN__QUAD: Option<surface::TriangleSurface> = None;
        match FULL_SCREEN__QUAD
        {
            None => {
                let positions: Vec<f32> = vec![
                    -3.0, -1.0, 0.0,
                    3.0, -1.0, 0.0,
                    0.0, 2.0, 0.0
                ];
                let uv_coordinates: Vec<f32> = vec![
                    -1.0, 0.0,
                    2.0, 0.0,
                    0.5, 1.5
                ];
                let mut mesh = mesh::Mesh::create(positions).unwrap();
                mesh.add_custom_vec2_attribute("uv_coordinate", uv_coordinates).unwrap();

                let surface = surface::TriangleSurface::create(gl, &mesh, program).unwrap();
                surface.render().unwrap();
                FULL_SCREEN__QUAD = Some(surface);
            },
            Some(ref x) => {x.render().unwrap();}
        }
    }
}