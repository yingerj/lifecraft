#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate cgmath;

use gfx::traits::FactoryExt;
use gfx::Device;
use cgmath::{SquareMatrix, Matrix4, Vector3};

//mod game;
//use game::{BOARD_SIZE, CELL_SIZE, GameOfLife};

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_vertex_struct!(Vertex {
    pos: [f32; 3] = "a_Pos",
});

gfx_constant_struct!( Locals {
    model: [[f32; 4]; 4] = "u_Model",
    view: [[f32; 4]; 4] = "u_View",
    proj: [[f32; 4]; 4] = "u_Proj",
});

gfx_pipeline!(pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    locals: gfx::ConstantBuffer<Locals> = "Locals",
    model: gfx::Global<[[f32; 4]; 4]> = "u_Model",
    view: gfx::Global<[[f32; 4]; 4]> = "u_View",
    proj: gfx::Global<[[f32; 4]; 4]> = "u_Proj",
    out: gfx::RenderTarget<ColorFormat> = "Target0",
});

const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];


fn main() {

    // Make Glutin window:
    let builder = glutin::WindowBuilder::new()
        .with_title("Lifecraft".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();
    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    // Set up gfx
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso_result = factory.create_pipeline_simple(
        include_bytes!("shader/game_150.glslv"),
        include_bytes!("shader/game_150.glslf"),
        gfx::state::CullFace::Nothing,
        pipe::new()
    );

    match pso_result {
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(-1);
        },
        Ok(_) => {},
    }
    let pso =  pso_result.unwrap();

    let vertex_data = [
        // top (0, 0, 1)
        Vertex{pos: [-1.0, -1.0,  1.0]},
        Vertex{pos: [ 1.0, -1.0,  1.0]},
        Vertex{pos: [ 1.0,  1.0,  1.0]},
        Vertex{pos: [-1.0,  1.0,  1.0]},
        // bottom (0, 0, -1)
        Vertex{pos: [-1.0,  1.0, -1.0]},
        Vertex{pos: [ 1.0,  1.0, -1.0]},
        Vertex{pos: [ 1.0, -1.0, -1.0]},
        Vertex{pos: [-1.0, -1.0, -1.0]},
        // right (1, 0, 0)
        Vertex{pos: [ 1.0, -1.0, -1.0]},
        Vertex{pos: [ 1.0,  1.0, -1.0]},
        Vertex{pos: [ 1.0,  1.0,  1.0]},
        Vertex{pos: [ 1.0, -1.0,  1.0]},
        // left (-1, 0, 0)
        Vertex{pos: [-1.0, -1.0,  1.0]},
        Vertex{pos: [-1.0,  1.0,  1.0]},
        Vertex{pos: [-1.0,  1.0, -1.0]},
        Vertex{pos: [-1.0, -1.0, -1.0]},
        // front (0, 1, 0)
        Vertex{pos: [ 1.0,  1.0, -1.0]},
        Vertex{pos: [-1.0,  1.0, -1.0]},
        Vertex{pos: [-1.0,  1.0,  1.0]},
        Vertex{pos: [ 1.0,  1.0,  1.0]},
        // back (0, -1, 0)
        Vertex{pos: [ 1.0, -1.0,  1.0]},
        Vertex{pos: [-1.0, -1.0,  1.0]},
        Vertex{pos: [-1.0, -1.0, -1.0]},
        Vertex{pos: [ 1.0, -1.0, -1.0]},
    ];

    let index_data: &[u16] = &[
         0,  1,  2,  2,  3,  0, // top
         4,  5,  6,  6,  7,  4, // bottom
         8,  9, 10, 10, 11,  8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    let (vbuf, slice) = factory.create_vertex_buffer_indexed(&vertex_data, index_data);
    let data = pipe::Data {
        vbuf: vbuf,
        locals: factory.create_constant_buffer(1),
        model: Matrix4::identity().into(),
        view: Matrix4::identity().into(),
        proj: Matrix4::identity().into(),
        out: main_color,
    };

    window.set_inner_size(1024/2, 768/2);

    //let mut game = GameOfLife::new();
    'main: loop {
        // loop over events
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                _ => {},
            }
        }
        // draw a frame
        let locals = Locals {
            model: Matrix4::from_scale(0.75f32).into(),
            view: Matrix4::from_translation(Vector3::new(0.0f32, 0.0, -4.0)).into(),
            //proj: Matrix4::identity().into(),
            proj: cgmath::perspective(cgmath::deg(90.0f32), 1024.0/768.0f32, 0.01, 1000.0).into(),
        };
        //encoder.update_buffer(&data.locals, &[locals], 0).unwrap();
        encoder.update_constant_buffer(&data.locals, &locals);
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
