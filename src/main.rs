extern crate nalgebra as na;
use std::f32::consts::PI;

use na::{Rotation3, Vector3};

#[derive(Debug)]
struct Shape {
    points: Vec<(Vector3<f32>, Vector3<f32>)>, // [(point, normal)]
}

impl Shape {
    fn new() -> Shape {
        Shape {
            points: Vec::with_capacity(0),
        }
    }

    fn initialize_donut(&mut self, r1: f32, r2: f32, precision: usize) -> () {
        self.points = Vec::with_capacity(precision * precision);
        let increment = 2. * PI / precision as f32;

        for theta_step in 0..precision {
            let theta = increment * theta_step as f32;
            for phi_step in 0..precision {
                let phi = increment * phi_step as f32;
                self.points.push((
                    Vector3::new(
                        (r2 + r1 * phi.cos()) * theta.cos(),
                        (r2 + r1 * phi.cos()) * theta.sin(),
                        r1 * phi.sin(),
                    ),
                    Vector3::new(phi.cos() * theta.cos(), phi.cos() * theta.sin(), phi.sin()),
                ));
            }
        }
    }

    fn initialize_cube(&mut self, length: f32, precision: usize) -> () {
        self.points = Vec::with_capacity(6 * precision * precision);
        let increment = length / precision as f32;
        let half_length = length / 2.;

        for d1 in 0..precision {
            let dd1 = -half_length + (d1 as f32) * increment;
            for d2 in 0..precision {
                let dd2 = -half_length + (d2 as f32) * increment;
                for normal_direction in vec![-1., 1.] {
                    let normal = Vector3::x_axis().scale(normal_direction);
                    self.points.push((
                        normal.scale(half_length)
                            + Vector3::y_axis().scale(dd1)
                            + Vector3::z_axis().scale(dd2),
                        normal,
                    ));
                    let normal = Vector3::y_axis().scale(normal_direction);
                    self.points.push((
                        normal.scale(half_length)
                            + Vector3::x_axis().scale(dd1)
                            + Vector3::z_axis().scale(dd2),
                        normal,
                    ));
                    let normal = Vector3::z_axis().scale(normal_direction);
                    self.points.push((
                        normal.scale(half_length)
                            + Vector3::x_axis().scale(dd1)
                            + Vector3::y_axis().scale(dd2),
                        normal,
                    ));
                }
            }
        }
    }
}

fn main() {
    let mut shape = Shape::new();
    shape.initialize_donut(10., 25., 80);
    //shape.initialize_cube(40., 40);

    let x_rot_speed = 0.00;
    let y_rot_speed = 0.06;
    let z_rot_speed = 0.02;
    let mut x_angle = 0.;
    let mut y_angle = 0.;
    let mut z_angle = 0.;

    let spectator_distance = 50.;
    let screen_distance = 20.;
    let light_source = Vector3::new(-100., 100., 100.);

    let width = 60;
    let height = 40;
    let mut screen_buffer = vec![0; width * height];
    let mut distance_buffer = vec![0.; width * height];

    let shadow_vector = vec![
        ' ', '.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@',
    ];

    loop {
        print!("\x1B[2J\x1B[1;1H");
        screen_buffer.fill(0);
        distance_buffer.fill(0.);
        let rot = Rotation3::from_euler_angles(x_angle, y_angle, z_angle);
        for (vi, ni) in &shape.points {
            let v = rot * vi;
            let n = rot * ni;
            let light_vector = light_source - v;
            let shadow = ((1. + n.dot(&light_vector) / (n.norm() + light_vector.norm())) * 6.)
                .clamp(0., 12.) as usize;
            if shadow == 0 {
                continue;
            }
            let inverse_z = 1. / (spectator_distance - v.z);

            let pixel_vector = Vector3::new(
                (v.x * screen_distance / (spectator_distance - v.z)) as i32,
                (v.y * screen_distance / (spectator_distance - v.z)) as i32,
                0,
            );
            let i = height as i32 / 2 - pixel_vector.y;
            let j = width as i32 / 2 + pixel_vector.x;
            let screen_idx = (i * (width as i32) + j) as usize;
            if 0 <= i
                && i < height as i32
                && 0 <= j
                && j < width as i32
                && distance_buffer[screen_idx] < inverse_z
                && screen_buffer[screen_idx] < shadow
            {
                screen_buffer[screen_idx] = shadow;
                distance_buffer[screen_idx] = inverse_z;
            }
        }

        for i in 0..height {
            for j in 0..width {
                print!("{}", shadow_vector[screen_buffer[i * width + j]]);
            }
            println!();
        }

        x_angle += x_rot_speed;
        y_angle += y_rot_speed;
        z_angle += z_rot_speed;

        if x_angle >= 2. * PI {
            x_angle = 0.;
        }
        if y_angle >= 2. * PI {
            y_angle = 0.;
        }
        if z_angle >= 2. * PI {
            z_angle = 0.;
        }
    }
}
