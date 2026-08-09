const CANVAS_WIDTH: isize = 512;
const CANVAS_HEIGHT: isize = 512;
const VIEWPORT_WIDTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 1.0;

mod vec;
use vec::Vec3;

mod scene;
use scene::{LIGHTS, Light, SPHERES, Sphere};

fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}

fn canvas_to_viewport(x: f64, y: f64) -> Vec3 {
    Vec3(
        x * VIEWPORT_WIDTH / CANVAS_WIDTH as f64,
        y * VIEWPORT_HEIGHT / CANVAS_HEIGHT as f64,
        1.0,
    )
}

fn trace_ray(origin: &Vec3, ray: &Vec3, tmin: f64, tmax: f64) -> Vec3 {
    let (closest_sphere, closest_t) = closest_intersection(origin, ray, tmin, tmax);
    if let Some(closest_sphere) = closest_sphere {
        // closest_sphere.color
        let p = *origin + *ray * closest_t; // compute intersection point
        let n = p - closest_sphere.center; // compute sphere normal at intersection
        closest_sphere.color * compute_lighting(p, n.normalize(), *ray * (-1.0), &closest_sphere)
    } else {
        Vec3(0.0, 0.0, 0.0)
    }
}

fn closest_intersection<'a>(
    origin: &'a Vec3,
    ray: &'a Vec3,
    tmin: f64,
    tmax: f64,
) -> (Option<&'a Sphere>, f64) {
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
    (closest_sphere, closest_t)
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

// * `point` - The intersection point of an object surface and a ray
fn compute_lighting(point: Vec3, n: Vec3, v: Vec3, sphere: &Sphere) -> f64 {
    let mut i = 0.0;
    for light in &LIGHTS {
        i += match *light {
            Light::Ambient { intensity } => intensity,
            Light::Point {
                intensity,
                position,
            } => {
                let l = position - point;
                compute_light_intensity(point, intensity, l, n, v, sphere.specular)
            }
            Light::Directional {
                intensity,
                direction,
            } => compute_light_intensity(point, intensity, direction, n, v, sphere.specular),
        }
    }
    i
}

fn compute_light_intensity(point: Vec3, intensity: f64, l: Vec3, n: Vec3, v: Vec3, s: i32) -> f64 {
    let mut i = 0.0;

    // Shadow check
    let (shadow_sphere, _) = closest_intersection(&point, &l, 0.001, f64::INFINITY);
    if let Some(_) = shadow_sphere {
        return 0.0;
    }

    // Diffuse
    let n_dot_l = dot(&l, &n);
    if n_dot_l > 0.0 {
        i += intensity * n_dot_l / (n.len() * l.len());
    }

    // Specular
    if s != -1 {
        let r = n * dot(&n, &l) * 2.0 - l;
        let r_dot_v = dot(&r, &v);
        if r_dot_v > 0.0 {
            i += intensity * (r_dot_v / (r.len() * v.len())).powi(s);
        }
    }
    i
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
