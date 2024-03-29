// use cgmath::vec3;
use glow::*;

// type Vec3 = cgmath::Vector3<f32>;

pub struct ChunkVAO {
    pub vao: NativeVertexArray,
    pub vbo: NativeBuffer,
    pub num_verts: usize,
    pub pos: (i32, i32),
}

impl ChunkVAO {
    pub fn init(gl: &glow::Context, smth: [i32; 262144], pos: (i32, i32)) -> ChunkVAO {
        let mut yea: Vec<f32> = Vec::new();

        let tex_size = 16;
        let atlas_size = 256;
        let tex_per_row = atlas_size / tex_size;

        // add each cube face by face.. can add culling manually or something idk
        for x in 0..16 {
            for z in 0..16 {
                for y in 0..1024 {
                    let val = smth[(x * 1024 * 16) + (z * 1024) + y];

                    if val != 0 {
                        let (tex_x_offset, tex_y_offset) = {
                            let temp_x: i32 = (val - 1) % tex_per_row;
                            let temp_y: i32 = (val - 1) / tex_per_row;

                            (temp_x as f32 / 16.0, temp_y as f32 / 16.0)
                        };

                        for face in 0..6 {
                            // gonna wanna add checks to make sure the face isn't covered yeah
                            let mut show_face = true;

                            if face == 0 {
                                if z != 0 {
                                    let south = smth[(x * 1024 * 16) + ((z - 1) * 1024) + y];

                                    if south != 0 {
                                        show_face = false;
                                    }
                                }
                            } else if face == 1 {
                                if z != 15 {
                                    let north = smth[(x * 1024 * 16) + ((z + 1) * 1024) + y];

                                    if north != 0 {
                                        show_face = false;
                                    }
                                }
                            } else if face == 2 {
                                if x != 0 {
                                    let west = smth[((x - 1) * 1024 * 16) + (z * 1024) + y];

                                    if west != 0 {
                                        show_face = false;
                                    }
                                }
                            } else if face == 3 {
                                if x != 15 {
                                    let east = smth[((x + 1) * 1024 * 16) + (z * 1024) + y];

                                    if east != 0 {
                                        show_face = false;
                                    }
                                }
                            } else if face == 4 {
                                if y != 1023 {
                                    let above = smth[(x * 1024 * 16) + (z * 1024) + y + 1];

                                    if above != 0 {
                                        show_face = false;
                                    }
                                }
                            } else if face == 5 {
                                if y != 0 {
                                    let below = smth[(x * 1024 * 16) + (z * 1024) + y - 1];

                                    if below != 0 {
                                        show_face = false;
                                    }
                                }
                            }

                            if show_face {
                                for vert in 0..6 {
                                    yea.push(CUBE_VERTICES[(vert * 8) + (face * 48)] + x as f32);
                                    yea.push(
                                        CUBE_VERTICES[(vert * 8) + (face * 48) + 1] + y as f32,
                                    );
                                    yea.push(
                                        CUBE_VERTICES[(vert * 8) + (face * 48) + 2] + z as f32,
                                    );
                                    yea.push(CUBE_VERTICES[(vert * 8) + (face * 48) + 3]);
                                    yea.push(CUBE_VERTICES[(vert * 8) + (face * 48) + 4]);
                                    yea.push(CUBE_VERTICES[(vert * 8) + (face * 48) + 5]);
                                    yea.push(
                                        CUBE_VERTICES[(vert * 8) + (face * 48) + 6] / 16.0
                                            + tex_x_offset,
                                    );
                                    yea.push(
                                        CUBE_VERTICES[(vert * 8) + (face * 48) + 7] / 16.0
                                            + tex_y_offset,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        unsafe {
            let vert_ptr: &[u8] = core::slice::from_raw_parts(
                yea.as_ptr() as *const u8,
                yea.len() * core::mem::size_of::<f32>(),
            );

            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vert_ptr, glow::STATIC_DRAW);

            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 8 * 4, 0);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 8 * 4, 3 * 4);
            gl.enable_vertex_attrib_array(2);
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 8 * 4, 6 * 4);

            let num_verts = yea.len() / 8;
            let ret = ChunkVAO {
                vao,
                vbo,
                num_verts,
                pos,
            };

            ret
        }
    }
}

const CUBE_VERTICES: [f32; 288] = [
    // verts            // normals        // text UVs

    // face 0 = south - faces towards -z
    0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, // vert 1
    0.0, 1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, // vert 2
    1.0, 1.0, 0.0, 0.0, 0.0, -1.0, 1.0, 1.0, // vert 3
    1.0, 1.0, 0.0, 0.0, 0.0, -1.0, 1.0, 1.0, // vert 4
    1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0, // vert 5
    0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, // vert 6
    // face 1 = north - faces towards +z
    1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // vert 1
    1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, // vert 2
    0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // vert 3
    0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // vert 4
    0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, // vert 5
    1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // vert 6
    // face 2 = west - faces towards -x
    0.0, 0.0, 1.0, -1.0, -0.0, -0.0, 0.0, 0.0, // vert 1
    0.0, 1.0, 1.0, -1.0, -0.0, -0.0, 0.0, 1.0, // vert 2
    0.0, 1.0, 0.0, -1.0, -0.0, -0.0, 1.0, 1.0, // vert 3
    0.0, 1.0, 0.0, -1.0, -0.0, -0.0, 1.0, 1.0, // vert 4
    0.0, 0.0, 0.0, -1.0, -0.0, -0.0, 1.0, 0.0, // vert 5
    0.0, 0.0, 1.0, -1.0, -0.0, -0.0, 0.0, 0.0, // vert 6
    // face 3 = east - faces towards +x
    1.0, 0.0, 0.0, 1.0, 0.0, -0.0, 0.0, 0.0, // vert 1
    1.0, 1.0, 0.0, 1.0, 0.0, -0.0, 0.0, 1.0, // vert 2
    1.0, 1.0, 1.0, 1.0, 0.0, -0.0, 1.0, 1.0, // vert 3
    1.0, 1.0, 1.0, 1.0, 0.0, -0.0, 1.0, 1.0, // vert 4
    1.0, 0.0, 1.0, 1.0, 0.0, -0.0, 1.0, 0.0, // vert 5
    1.0, 0.0, 0.0, 1.0, 0.0, -0.0, 0.0, 0.0, // vert 6
    // face 4 = top - faces towards +y
    0.0, 1.0, 0.0, -0.0, 1.0, 0.0, 0.0, 0.0, // vert 1
    0.0, 1.0, 1.0, -0.0, 1.0, 0.0, 0.0, 1.0, // vert 2
    1.0, 1.0, 1.0, -0.0, 1.0, 0.0, 1.0, 1.0, // vert 3
    1.0, 1.0, 1.0, -0.0, 1.0, 0.0, 1.0, 1.0, // vert 4
    1.0, 1.0, 0.0, -0.0, 1.0, 0.0, 1.0, 0.0, // vert 5
    0.0, 1.0, 0.0, -0.0, 1.0, 0.0, 0.0, 0.0, // vert 6
    // face 5 = bottom - faces towards -y
    1.0, 0.0, 0.0, -0.0, -1.0, -0.0, 0.0, 0.0, // vert 1
    1.0, 0.0, 1.0, -0.0, -1.0, -0.0, 0.0, 1.0, // vert 2
    0.0, 0.0, 1.0, -0.0, -1.0, -0.0, 1.0, 1.0, // vert 3
    0.0, 0.0, 1.0, -0.0, -1.0, -0.0, 1.0, 1.0, // vert 4
    0.0, 0.0, 0.0, -0.0, -1.0, -0.0, 1.0, 0.0, // vert 5
    1.0, 0.0, 0.0, -0.0, -1.0, -0.0, 0.0, 0.0, // vert 6
];
