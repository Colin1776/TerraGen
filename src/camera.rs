use cgmath::vec3;
use cgmath::{self, InnerSpace};

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

    fn move_forward(&mut self, dist: f32) {
        self.pos += Vec3::normalize(vec3(self.front.x, 0.0, self.front.z) * dist);
    }

    fn move_right(&mut self, dist: f32) {
        let temp = Vec3::normalize(Vec3::cross(self.front, self.up) * dist);
        self.pos += vec3(temp.x, 0.0, temp.z);
    }

    fn move_up(&mut self, dist: f32) {
        self.pos += vec3(0.0, dist, 0.0);
    }
}
