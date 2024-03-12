use cgmath::*;
use glow::*;

use sdl2::event::WindowEvent;

type Vec3 = cgmath::Vector3<f32>;
type Mat4 = cgmath::Matrix4<f32>;

mod camera;
mod shader;
mod vao;

mod gen;

fn main() {
    let (gl, window, mut events_loop, _context, _sdl) = create_sdl2_context();

    let mut cam = camera::Camera::init();

    let basic = shader::Shader::build(&gl, "res/shaders/basic.vert", "res/shaders/basic.frag");
    basic.activate(&gl);
    basic.set_f32(&gl, "blue", 0.8);

    let (vbo, vao) = create_vertex_buffer(&gl);

    let cube_positions: [Vec3; 8] = [
        vec3(1.0, 1.0, 1.0),
        vec3(1.0, 1.0, 2.0),
        vec3(1.0, 1.0, 3.0),
        vec3(1.0, 2.0, 3.0),
        vec3(1.0, 3.0, 3.0),
        vec3(1.0, 3.0, 4.0),
        vec3(1.0, 3.0, 5.0),
        vec3(1.0, 4.0, 5.0),
    ];

    let mut prev_keys: std::collections::HashSet<sdl2::keyboard::Keycode> =
        std::collections::HashSet::new();

    let mut win_width: f32 = 800.0;
    let mut win_height: f32 = 600.0;

    _sdl.mouse().set_relative_mouse_mode(true);

    let mut old = std::time::Instant::now();

    unsafe {
        gl.enable(glow::CULL_FACE);
    }

    'render: loop {
        let delta_time = old.elapsed().as_secs_f32();
        old = std::time::Instant::now();

        let x = handle_events(
            &_sdl,
            &gl,
            &mut events_loop,
            &mut cam,
            &mut prev_keys,
            &mut win_width,
            &mut win_height,
            delta_time,
        );

        if x == true {
            break 'render;
        }

        let projection = cgmath::perspective(Deg(90.0), win_width / win_height, 0.1, 100.0);
        let view = cam.get_view();
        // let model = Mat4::identity();

        basic.set_mat4(&gl, "projection", projection);
        basic.set_mat4(&gl, "view", view);
        // basic.set_mat4(&gl, "model", model);

        unsafe {
            gl.clear_color(0.53, 0.81, 0.92, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            gl.bind_vertex_array(Some(vao));

            for pos in cube_positions {
                let model = Mat4::from_translation(pos);
                basic.set_mat4(&gl, "model", model);
                gl.draw_arrays(glow::TRIANGLES, 0, 36);
            }

            // gl.draw_arrays(glow::TRIANGLES, 0, 36);
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
    sdl2::Sdl,
) {
    unsafe {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_flags().forward_compatible().set();
        let window = video
            .window("Fortnite", 800, 600)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context().unwrap();
        let gl = glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
        let event_loop = sdl.event_pump().unwrap();

        (gl, window, event_loop, gl_context, sdl)
    }
}

fn create_vertex_buffer(gl: &glow::Context) -> (NativeBuffer, NativeVertexArray) {
    // let triangle_vertices = [0.5f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32];

    // cube (hope this is right) :D
    let cube_vertices: [f32; 288] = [
        // verts            // normals        // text UVs

        // face 0 = south
        0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, // vert 1
        0.0, 1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, // vert 2
        1.0, 1.0, 0.0, 0.0, 0.0, -1.0, 1.0, 1.0, // vert 3
        1.0, 1.0, 0.0, 0.0, 0.0, -1.0, 1.0, 1.0, // vert 4
        1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0, // vert 5
        0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, // vert 6
        // face 1 = north
        1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // vert 1
        1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, // vert 2
        0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // vert 3
        0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // vert 4
        0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, // vert 5
        1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // vert 6
        // face 2 = west
        0.0, 0.0, 1.0, -1.0, -0.0, -0.0, 0.0, 0.0, // vert 1
        0.0, 1.0, 1.0, -1.0, -0.0, -0.0, 0.0, 1.0, // vert 2
        0.0, 1.0, 0.0, -1.0, -0.0, -0.0, 1.0, 1.0, // vert 3
        0.0, 1.0, 0.0, -1.0, -0.0, -0.0, 1.0, 1.0, // vert 4
        0.0, 0.0, 0.0, -1.0, -0.0, -0.0, 1.0, 0.0, // vert 5
        0.0, 0.0, 1.0, -1.0, -0.0, -0.0, 0.0, 0.0, // vert 6
        // face 3 = east
        1.0, 0.0, 0.0, 1.0, 0.0, -0.0, 0.0, 0.0, // vert 1
        1.0, 1.0, 0.0, 1.0, 0.0, -0.0, 0.0, 1.0, // vert 2
        1.0, 1.0, 1.0, 1.0, 0.0, -0.0, 1.0, 1.0, // vert 3
        1.0, 1.0, 1.0, 1.0, 0.0, -0.0, 1.0, 1.0, // vert 4
        1.0, 0.0, 1.0, 1.0, 0.0, -0.0, 1.0, 0.0, // vert 5
        1.0, 0.0, 0.0, 1.0, 0.0, -0.0, 0.0, 0.0, // vert 6
        // face 4 = top
        0.0, 1.0, 0.0, -0.0, 1.0, 0.0, 0.0, 0.0, // vert 1
        0.0, 1.0, 1.0, -0.0, 1.0, 0.0, 0.0, 1.0, // vert 2
        1.0, 1.0, 1.0, -0.0, 1.0, 0.0, 1.0, 1.0, // vert 3
        1.0, 1.0, 1.0, -0.0, 1.0, 0.0, 1.0, 1.0, // vert 4
        1.0, 1.0, 0.0, -0.0, 1.0, 0.0, 1.0, 0.0, // vert 5
        0.0, 1.0, 0.0, -0.0, 1.0, 0.0, 0.0, 0.0, // vert 6
        // face 5 = bottom
        1.0, 0.0, 0.0, -0.0, -1.0, -0.0, 0.0, 0.0, // vert 1
        1.0, 0.0, 1.0, -0.0, -1.0, -0.0, 0.0, 1.0, // vert 2
        0.0, 0.0, 1.0, -0.0, -1.0, -0.0, 1.0, 1.0, // vert 3
        0.0, 0.0, 1.0, -0.0, -1.0, -0.0, 1.0, 1.0, // vert 4
        0.0, 0.0, 0.0, -0.0, -1.0, -0.0, 1.0, 0.0, // vert 5
        1.0, 0.0, 0.0, -0.0, -1.0, -0.0, 0.0, 0.0, // vert 6
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

        let vao: NativeVertexArray = gl.create_vertex_array().unwrap();
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

fn handle_events(
    sdl: &sdl2::Sdl,
    gl: &glow::Context,
    events: &mut sdl2::EventPump,
    cam: &mut camera::Camera,
    prev_keys: &mut std::collections::HashSet<sdl2::keyboard::Keycode>,
    win_width: &mut f32,
    win_height: &mut f32,
    delta_time: f32,
) -> bool {
    for event in events.poll_iter() {
        if let sdl2::event::Event::Quit { .. } = event {
            return true;
        }

        if let sdl2::event::Event::MouseMotion {
            timestamp: _,
            window_id: _,
            which: _,
            mousestate: _,
            x: _,
            y: _,
            xrel,
            yrel,
        } = event
        {
            let sens = 0.1;

            let x_amnt = sens * xrel as f32;
            let y_amnt = sens * yrel as f32;

            cam.rotate_right(x_amnt);
            cam.rotate_up(-y_amnt);
        }

        if let sdl2::event::Event::Window { win_event, .. } = event {
            match win_event {
                WindowEvent::Resized(x, y) => unsafe {
                    *win_width = x as f32;
                    *win_height = y as f32;
                    gl.viewport(0, 0, x, y);
                },

                _ => {}
            }
        }
    }

    // keyboard input
    let keys: std::collections::HashSet<sdl2::keyboard::Keycode> = events
        .keyboard_state()
        .pressed_scancodes()
        .filter_map(sdl2::keyboard::Keycode::from_scancode)
        .collect();

    let new_keys = &keys - &prev_keys;
    let current_keys = keys.clone();

    *prev_keys = keys;

    let speed: f32 = 2.5 * delta_time;

    for key in current_keys {
        match key {
            sdl2::keyboard::Keycode::W => cam.move_forward(speed as f32),
            sdl2::keyboard::Keycode::A => cam.move_right(-speed),
            sdl2::keyboard::Keycode::S => cam.move_forward(-speed),
            sdl2::keyboard::Keycode::D => cam.move_right(speed),
            sdl2::keyboard::Keycode::Space => cam.move_up(speed),
            sdl2::keyboard::Keycode::LShift => cam.move_up(-speed),

            _ => {}
        }
    }

    for key in new_keys {
        match key {
            sdl2::keyboard::Keycode::C => println!(
                "{}, {}, {}",
                cam.get_pos().x,
                cam.get_pos().y,
                cam.get_pos().z
            ),
            sdl2::keyboard::Keycode::Escape => {
                let x = sdl.mouse().relative_mouse_mode();
                sdl.mouse().set_relative_mouse_mode(!x);
            }

            _ => {}
        }
    }

    false
}
