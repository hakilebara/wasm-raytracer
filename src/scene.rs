use crate::vec::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub color: Vec3,
    pub specular: i32,
    pub reflective: f64,
}

pub enum Light {
    Ambient { intensity: f64 },
    Point { intensity: f64, position: Vec3 },
    Directional { intensity: f64, direction: Vec3 },
}

pub const LIGHTS: [Light; 3] = [
    Light::Ambient { intensity: 0.2 },
    Light::Point {
        intensity: 0.6,
        position: Vec3(2.0, 1.0, 0.0),
    },
    Light::Directional {
        intensity: 0.2,
        direction: Vec3(1.0, 4.0, 4.0),
    },
];

pub const SPHERES: [Sphere; 5] = [
    Sphere {
        center: Vec3(0.0, -1.0, 3.0),
        radius: 1.0,
        color: Vec3(255.0, 0.0, 0.0),
        specular: 500,
        reflective: 0.2,
    },
    Sphere {
        center: Vec3(2.0, 0.0, 4.0),
        radius: 1.0,
        color: Vec3(0.0, 0.0, 255.0),
        specular: 500,
        reflective: 0.3,
    },
    Sphere {
        center: Vec3(-2.0, 0.0, 4.0),
        radius: 1.0,
        color: Vec3(0.0, 255.0, 0.0),
        specular: 10,
        reflective: 0.4,
    },
    Sphere {
        center: Vec3(0.0, 0.0, -3.0),
        radius: 1.0,
        color: Vec3(255.0, 0.0, 0.0),
        specular: 500,
        reflective: 0.2,
    },
    Sphere {
        center: Vec3(0.0, -5001.0, 0.0),
        radius: 5000.0,
        color: Vec3(255.0, 255.0, 0.0),
        specular: 1000,
        reflective: 0.5,
    },
];
