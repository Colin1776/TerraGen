use cgmath::*;
use glow::*;
// use image::*;
use std::path::Path;

use sdl2::event::WindowEvent;

// type Vec3 = cgmath::Vector3<f32>;
type Mat4 = cgmath::Matrix4<f32>;

mod camera;
mod shader;
mod vao;

mod gen;
use gen::piece;

fn main() {
    /* Initialize rendering stuff like window, camera, shaders, textures, etc */
    let (gl, window, mut events_loop, _context, _sdl) = create_sdl2_context();

    let mut cam = camera::Camera::init();

    let basic = shader::Shader::build(&gl, "res/shaders/basic.vert", "res/shaders/basic.frag");
    basic.activate(&gl);
    basic.set_f32(&gl, "blue", 0.8);

    let texture = load_texture(&gl);
    basic.set_i32(&gl, "tex", 0);

    /* Initialize the terrain generator */

    // terragen will return a value of 0 to represent air in the terrain
    //                                 1 to represent stone in the terrain
    //                                 2 to represent grass in the terrain
    let mut pieces: Vec<piece::Piece> = Vec::new();
    pieces.push(piece::Piece {
        name: "air".to_string(),
        material: piece::Material::AIR,
    });
    pieces.push(piece::Piece {
        name: "stone".to_string(),
        material: piece::Material::ROCK,
    });
    pieces.push(piece::Piece {
        name: "grass".to_string(),
        material: piece::Material::SOIL,
    });

    let chunk0 = gen::get_chunk(0, 0);
    let chunk1 = gen::get_chunk(0, 1);

    let start = std::time::Instant::now();

    let base_chunk = vao::ChunkVAO::init(&gl, chunk0);

    let finish = std::time::Instant::now();

    let elps = finish - start;
    println!("{:?}", elps);

    let other_chunk = vao::ChunkVAO::init(&gl, chunk1);

    let mut prev_keys: std::collections::HashSet<sdl2::keyboard::Keycode> =
        std::collections::HashSet::new();

    let mut win_width: f32 = 800.0;
    let mut win_height: f32 = 600.0;

    _sdl.mouse().set_relative_mouse_mode(true);

    let mut old = std::time::Instant::now();
    let mut timer = 0.0;

    unsafe {
        gl.enable(glow::CULL_FACE);
        gl.enable(glow::DEPTH_TEST);
    }

    let _interval = window.subsystem().gl_set_swap_interval(0);

    'render: loop {
        let delta_time = old.elapsed().as_secs_f32();
        timer += delta_time;
        old = std::time::Instant::now();

        if timer >= 1.0 {
            let fps = 1.0 / delta_time;
            println!("{}", fps);
            timer = 0.0;
        }

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

            gl.bind_texture(glow::TEXTURE_2D, Some(texture));

            let mut model = Mat4::identity();
            basic.set_mat4(&gl, "model", model);

            gl.bind_vertex_array(Some(base_chunk.vao));
            gl.draw_arrays(glow::TRIANGLES, 0, base_chunk.num_verts as i32);

            model = Mat4::from_translation(vec3(0.0, 0.0, 16.0));
            basic.set_mat4(&gl, "model", model);

            gl.bind_vertex_array(Some(other_chunk.vao));
            gl.draw_arrays(glow::TRIANGLES, 0, other_chunk.num_verts as i32);
        }

        window.gl_swap_window();
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

fn load_texture(gl: &glow::Context) -> NativeTexture {
    let img = image::open(&Path::new("res/textures/texture_atlas.png"))
        .expect("No texture :(")
        .flipv()
        .fliph();
    // img.fliph();
    let data = img.as_bytes();
    unsafe {
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MIN_FILTER,
            glow::NEAREST as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MAG_FILTER,
            glow::NEAREST as i32,
        );
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            glow::RGB as u32,
            glow::UNSIGNED_BYTE,
            Some(data),
        );
        gl.generate_mipmap(glow::TEXTURE_2D);

        texture
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
