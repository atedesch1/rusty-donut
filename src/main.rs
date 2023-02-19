extern crate nalgebra as na;
use std::f32::consts::PI;

#[derive(Debug)]
struct Shape {
    points: Vec<(na::Vector3<f32>, na::Vector3<f32>)>,
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
                    na::Vector3::new(
                        (r2 + r1 * phi.cos()) * theta.cos(),
                        (r2 + r1 * phi.cos()) * theta.sin(),
                        r1 * phi.sin(),
                    ),
                    na::Vector3::new(phi.cos() * theta.cos(), phi.cos() * theta.sin(), phi.sin()),
                ));
            }
        }
    }

    fn rotate(&mut self, rot: &na::Rotation<f32, 3>) -> () {
        for i in 0..self.points.len() {
            self.points[i] = (rot * self.points[i].0, rot * self.points[i].1);
        }
    }
}

fn main() {
    let mut donut = Shape::new();
    donut.initialize_donut(10., 20., 100);

    let rot_yaxis = na::Rotation3::from_axis_angle(&na::Vector3::y_axis(), 1.);
    let rot_xaxis = na::Rotation3::from_axis_angle(&na::Vector3::x_axis(), 2.);
    let rot = rot_xaxis * rot_yaxis;

    loop {
        donut.rotate(&rot);
    }
}
