use cgmath;
use cgmath::vec3;

type Vec3 = cgmath::Vector3<f32>;

pub struct Camera {
    pos: Vec3,
    front: Vec3,
    up: Vec3,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub fn init() -> Camera {
        let cam = Camera {
            pos: vec3(0.0, 0.0, 3.0),
            front: vec3(0.0, 0.0, -1.0),
            up: vec3(0.0, 1.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
        };
        cam
    }
}
