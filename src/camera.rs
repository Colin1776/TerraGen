use cgmath::vec3;
use cgmath::{self, InnerSpace};

type Point3 = cgmath::Point3<f32>;
type Vec3 = cgmath::Vector3<f32>;
type Mat4 = cgmath::Matrix4<f32>;

pub struct Camera {
    pos: Point3,
    front: Vec3,
    up: Vec3,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub fn init() -> Camera {
        let cam = Camera {
            pos: Point3::new(0.0, 0.0, 3.0),
            front: vec3(0.0, 0.0, -1.0),
            up: vec3(0.0, 1.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
        };
        cam
    }

    pub fn move_forward(&mut self, dist: f32) {
        self.pos += Vec3::normalize(vec3(self.front.x, 0.0, self.front.z)) * dist;
    }

    pub fn move_right(&mut self, dist: f32) {
        let temp = Vec3::normalize(Vec3::cross(self.front, self.up)) * dist;
        self.pos += vec3(temp.x, 0.0, temp.z);
    }

    pub fn move_up(&mut self, dist: f32) {
        self.pos += vec3(0.0, dist, 0.0);
    }

    pub fn rotate_right(&mut self, amt: f32) {
        self.yaw += amt;
        if self.yaw > 180.0 {
            self.yaw -= 360.0;
        }
        if self.yaw < -180.0 {
            self.yaw += 360.0
        }
        self.update_front();
    }

    pub fn rotate_up(&mut self, amt: f32) {
        self.pitch += amt;
        if self.pitch > 89.0 {
            self.pitch = 89.0
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0
        }
        self.update_front();
    }

    fn update_front(&mut self) {
        let front = Vec3 {
            x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            y: self.pitch.to_radians().sin(),
            z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        };
        self.front = front.normalize();
    }

    #[allow(dead_code)]
    pub fn get_view(&self) -> Mat4 {
        Mat4::look_at(self.pos, self.pos + self.front, self.up)
    }

    pub fn get_pos(&self) -> Point3 {
        let ret = self.pos.clone();
        ret
    }
}
