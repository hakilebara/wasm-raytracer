use std::ops::{Add, Mul, Sub};

const CANVAS_WIDTH: isize = 512;
const CANVAS_HEIGHT: isize = 512;
const VIEWPORT_WIDTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 1.0;

#[derive(Clone, Copy)]
struct Vec3(f64, f64, f64);

impl Vec3 {
    fn len(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    fn normalize(self) -> Self {
        let length = self.len();
        Vec3(self.0 / length, self.1 / length, self.2 / length)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

struct Sphere {
    center: Vec3,
    radius: f64,
    color: Vec3,
}

enum Light {
    Ambient { intensity: f64 },
    Point { intensity: f64, position: Vec3 },
    Directional { intensity: f64, direction: Vec3 },
}

const LIGHTS: [Light; 3] = [
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

const SPHERES: [Sphere; 4] = [
    Sphere {
        center: Vec3(0.0, -1.0, 3.0),
        radius: 1.0,
        color: Vec3(255.0, 0.0, 0.0),
    },
    Sphere {
        center: Vec3(2.0, 0.0, 4.0),
        radius: 1.0,
        color: Vec3(0.0, 0.0, 255.0),
    },
    Sphere {
        center: Vec3(-2.0, 0.0, 4.0),
        radius: 1.0,
        color: Vec3(0.0, 255.0, 0.0),
    },
    Sphere {
        center: Vec3(0.0, -5001.0, 0.0),
        radius: 5000.0,
        color: Vec3(255.0, 255.0, 0.0),
    },
];

fn canvas_to_viewport(x: f64, y: f64) -> Vec3 {
    Vec3(
        x * VIEWPORT_WIDTH / CANVAS_WIDTH as f64,
        y * VIEWPORT_HEIGHT / CANVAS_HEIGHT as f64,
        1.0,
    )
}

fn intersect_ray_sphere(origin: &Vec3, ray: &Vec3, sphere: &Sphere) -> (f64, f64) {
    let a = dot(&ray, &ray);
    let co = *origin - sphere.center;
    let b = 2.0 * dot(&ray, &co);
    let c = dot(&co, &co) - sphere.radius.powi(2);

    let discriminant: f64 = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        (f64::INFINITY, f64::INFINITY)
    } else {
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
        (t1, t2)
    }
}

fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}

fn compute_intensity(intensity: f64, l: Vec3, n: Vec3) -> f64 {
    let n_dot_l = dot(&l, &n);
    if n_dot_l > 0.0 {
        intensity * n_dot_l / (n.len() * l.len())
    } else {
        0.0
    }
}

fn compute_lighting(point: Vec3, n: Vec3) -> f64 {
    let mut i = 0.0;
    for light in &LIGHTS {
        i += match *light {
            Light::Ambient { intensity } => intensity,
            Light::Point {
                intensity,
                position,
            } => {
                let l = position - point;
                compute_intensity(intensity, l, n)
            }
            Light::Directional {
                intensity,
                direction,
            } => compute_intensity(intensity, direction, n),
        }
    }
    i
}

fn trace_ray(origin: &Vec3, ray: &Vec3, tmin: f64, tmax: f64) -> Vec3 {
    let mut closest_t = f64::INFINITY;
    let mut closest_sphere: Option<&Sphere> = None;
    for sphere in &SPHERES {
        let (t1, t2) = intersect_ray_sphere(origin, ray, sphere);
        if tmin < t1 && t1 < tmax && t1 < closest_t {
            closest_t = t1;
            closest_sphere = Some(sphere);
        }
        if tmin < t2 && t2 < tmax && t2 < closest_t {
            closest_t = t2;
            closest_sphere = Some(sphere);
        }
    }
    if let Some(closest_sphere) = closest_sphere {
        // closest_sphere.color
        let p = *origin + *ray * closest_t; // compute intersection point
        let n = p - closest_sphere.center; // compute sphere normal at intersection
        closest_sphere.color * compute_lighting(p, n.normalize())
    } else {
        Vec3(255.0, 255.0, 255.0)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn raytrace(ptr: *mut u8, len: usize) {
    let slice = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    let origin = Vec3(0.0, 0.0, 0.0);

    let mut pixels = slice.chunks_exact_mut(3);

    for y in (CANVAS_HEIGHT / -2..CANVAS_HEIGHT / 2).rev() {
        for x in CANVAS_WIDTH / -2..CANVAS_WIDTH / 2 {
            let ray = canvas_to_viewport(x as f64, y as f64);
            let color = trace_ray(&origin, &ray, 1.0, f64::INFINITY);
            let pixel = pixels.next().unwrap();
            pixel[0] = color.0.ceil() as u8;
            pixel[1] = color.1.ceil() as u8;
            pixel[2] = color.2.ceil() as u8;
        }
    }
}
