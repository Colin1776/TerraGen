use glow::*;

mod camera;
mod shader;

fn main() {
    // Create a context from a sdl2 window
    let (gl, window, mut events_loop, _context) = create_sdl2_context();

    let basic = shader::Shader::build(&gl, "res/shaders/basic.vert", "res/shaders/basic.frag");
    basic.activate(&gl);
    basic.set_f32(&gl, "blue", 0.8);

    // Create a vertex buffer and vertex array object
    let (vbo, vao) = create_vertex_buffer(&gl);

    'render: loop {
        for event in events_loop.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event {
                break 'render;
            }
        }

        unsafe {
            gl.clear_color(0.53, 0.81, 0.92, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            gl.draw_arrays(glow::TRIANGLES, 0, 36);
        }

        window.gl_swap_window();
    }

    // Clean up
    unsafe {
        gl.delete_program(basic.prog);
        gl.delete_vertex_array(vao);
        gl.delete_buffer(vbo)
    }
}

fn create_sdl2_context() -> (
    glow::Context,
    sdl2::video::Window,
    sdl2::EventPump,
    sdl2::video::GLContext,
) {
    unsafe {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_flags().forward_compatible().set();
        let window = video
            .window("Hello triangle!", 1024, 769)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context().unwrap();
        let gl = glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
        let event_loop = sdl.event_pump().unwrap();

        (gl, window, event_loop, gl_context)
    }
}

fn create_vertex_buffer(gl: &glow::Context) -> (NativeBuffer, NativeVertexArray) {
    // let triangle_vertices = [0.5f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32];

    // cube (hope this is right) :D
    let cube_vertices: [f32; 288] = [
        // verts            // normals        // text UVs

        // face 0 = south
        -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0, -0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 1.0, 0.5,
        0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0, 0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0, 0.5, -0.5,
        -0.5, 0.0, 0.0, -1.0, 1.0, 0.0, -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0,
        // face 1 = north
        0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 1.0, -0.5, 0.5,
        0.5, 0.0, 0.0, 1.0, 1.0, 1.0, -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0, -0.5, -0.5, 0.5,
        0.0, 0.0, 1.0, 1.0, 0.0, 0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0,
        // face 2 = west
        -0.5, -0.5, 0.5, -1.0, -0.0, -0.0, 0.0, 0.0, -0.5, 0.5, 0.5, -1.0, -0.0, -0.0, 0.0, 1.0,
        -0.5, 0.5, -0.5, -1.0, -0.0, -0.0, 1.0, 1.0, -0.5, 0.5, -0.5, -1.0, -0.0, -0.0, 1.0, 1.0,
        -0.5, -0.5, -0.5, -1.0, -0.0, -0.0, 1.0, 0.0, -0.5, -0.5, 0.5, -1.0, -0.0, -0.0, 0.0, 0.0,
        // face 3 = east
        0.5, -0.5, -0.5, 1.0, 0.0, -0.0, 0.0, 0.0, 0.5, 0.5, -0.5, 1.0, 0.0, -0.0, 0.0, 1.0, 0.5,
        0.5, 0.5, 1.0, 0.0, -0.0, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.0, 1.0, 1.0, 0.5, -0.5,
        0.5, 1.0, 0.0, -0.0, 1.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, -0.0, 0.0, 0.0,
        // face 4 = top
        -0.5, 0.5, -0.5, -0.0, 1.0, 0.0, 0.0, 0.0, -0.5, 0.5, 0.5, -0.0, 1.0, 0.0, 0.0, 1.0, 0.5,
        0.5, 0.5, -0.0, 1.0, 0.0, 1.0, 1.0, 0.5, 0.5, 0.5, -0.0, 1.0, 0.0, 1.0, 1.0, 0.5, 0.5,
        -0.5, -0.0, 1.0, 0.0, 1.0, 0.0, -0.5, 0.5, -0.5, -0.0, 1.0, 0.0, 0.0, 0.0,
        // face 5 = bottom
        0.5, -0.5, -0.5, -0.0, -1.0, -0.0, 0.0, 0.0, 0.5, -0.5, 0.5, -0.0, -1.0, -0.0, 0.0, 1.0,
        -0.5, -0.5, 0.5, -0.0, -1.0, -0.0, 1.0, 1.0, -0.5, -0.5, 0.5, -0.0, -1.0, -0.0, 1.0, 1.0,
        -0.5, -0.5, -0.5, -0.0, -1.0, -0.0, 1.0, 0.0, 0.5, -0.5, -0.5, -0.0, -1.0, -0.0, 0.0, 0.0,
    ];

    unsafe {
        // let triangle_vertices_u8: &[u8] = core::slice::from_raw_parts(
        //     triangle_vertices.as_ptr() as *const u8,
        //     triangle_vertices.len() * core::mem::size_of::<f32>(),
        // );

        let cube_vertices_u8: &[u8] = core::slice::from_raw_parts(
            cube_vertices.as_ptr() as *const u8,
            cube_vertices.len() * core::mem::size_of::<f32>(),
        );

        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cube_vertices_u8, glow::STATIC_DRAW);

        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 8 * 4, 0);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 8 * 4, 3 * 4);
        gl.enable_vertex_attrib_array(2);
        gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 8 * 4, 6 * 4);

        (vbo, vao)
    }
}
