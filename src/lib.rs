mod bindings {
    wit_bindgen::generate!({
        path: "wit/raytraycer.wit",
    });
}

const CANVAS_WIDTH: isize = 512;
const CANVAS_HEIGHT: isize = 512;
const VIEWPORT_WIDTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 1.0;

mod vec;

use vec::Vec3;

mod scene;
use scene::{Light, Sphere, LIGHTS, SPHERES};

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

fn trace_ray(origin: &Vec3, ray: &Vec3, tmin: f64, tmax: f64, depth: u32) -> Vec3 {
    let (closest_sphere, closest_t) = closest_intersection(origin, ray, tmin, tmax);
    if let Some(closest_sphere) = closest_sphere {
        // closest_sphere.color
        let p = *origin + *ray * closest_t; // compute intersection point
        let n = p - closest_sphere.center; // compute sphere normal at intersection
        let local_color = closest_sphere.color
            * compute_lighting(p, n.normalize(), *ray * (-1.0), &closest_sphere);

        // If we hit the recursion limit or the object is not reflective, we're done
        if depth <= 0 || closest_sphere.reflective <= 0.0 {
            return local_color;
        }

        // Compute the reflected color
        let r = reflect_ray(*ray * (-1.0), n);
        let reflected_color = trace_ray(&p, &r, 0.001, f64::INFINITY, depth - 1);

        return local_color * (1.0 - closest_sphere.reflective)
            + reflected_color * closest_sphere.reflective;
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

fn reflect_ray(r: Vec3, n: Vec3) -> Vec3 {
    n * dot(&n, &r) * 2.0 - r
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
        let r = reflect_ray(l, n);
        let r_dot_v = dot(&r, &v);
        if r_dot_v > 0.0 {
            i += intensity * (r_dot_v / (r.len() * v.len())).powi(s);
        }
    }
    i
}

struct Component;

impl bindings::Guest for Component {
    fn raytrace(rotation: f64) -> Vec<u8> {
        let origin = Vec3(0.0, 0.0, 0.0);
        let mut ans = vec![];

        for y in (CANVAS_HEIGHT / -2..CANVAS_HEIGHT / 2).rev() {
            for x in CANVAS_WIDTH / -2..CANVAS_WIDTH / 2 {
                let ray = canvas_to_viewport(x as f64, y as f64);
                let color = trace_ray(&origin, &ray.rotate_y(rotation), 1.0, f64::INFINITY, 2);
                ans.push(color.0.ceil() as u8);
                ans.push(color.1.ceil() as u8);
                ans.push(color.2.ceil() as u8);
            }
        }
        ans
    }
}

bindings::export!(Component with_types_in bindings);
