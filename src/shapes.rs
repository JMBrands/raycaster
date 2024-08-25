use std::{borrow::Borrow, f32::INFINITY};

use raylib::prelude::*;

pub enum Shape {
    Cube(Cube),
    Camera(Camera),
}

pub struct Camera {
    pos: Vector3,
    fov: i32,
    angle: Angle3D,
    dimensions: Vector2,
}

impl Camera {
    pub fn new(pos: Vector3, fov: i32, angle: Angle3D, dimensions: Vector2) -> Camera {
        Camera {
            pos: pos,
            fov: fov,
            angle: angle,
            dimensions: dimensions,
        }
    }

    pub fn draw(&self, mut d: RaylibDrawHandle, root: &Vec<Shape>) {
        let mut x = 0;
        while x < self.dimensions.x as i32 {
            let mut y = 0;
            while y < self.dimensions.y as i32 {
                let distance = fire_ray(
                    self.pos,
                    Angle3D {
                        roll: 0.0,
                        yaw: self.angle.yaw
                            + (((x as f32) - (self.dimensions.x / 2.0)) / self.dimensions.x
                                * (self.fov as f32)),
                        pitch: self.angle.pitch
                            + (((y as f32) - (self.dimensions.y / 2.0)) / self.dimensions.y
                                * (self.fov as f32)),
                    },
                    255.0,
                );
                d.draw_pixel(
                    x,
                    y,
                    Color {
                        r: distance as u8,
                        g: distance as u8,
                        b: distance as u8,
                        a: 255,
                    },
                );
                y += 1;
            }
            x += 1;
        }
    }
}

pub struct Angle3D {
    pub roll: f32,
    pub yaw: f32,
    pub pitch: f32,
}

pub fn distance_squared(vec1: Vector3, vec2: Vector3) -> f32 {
    (vec1.x - vec2.x) * (vec1.x - vec2.x)
        + (vec1.y - vec2.y) * (vec1.y - vec2.y)
        + (vec1.z - vec2.z) * (vec1.z - vec2.z)
}

pub fn distance(vec1: Vector3, vec2: Vector3) -> f32 {
    f32::sqrt(distance_squared(vec1, vec2))
}

pub fn fire_ray(start: Vector3, angle: Angle3D, limit: f32, root: Vec<Shape>) -> f32 {
    let mut smallest = &root[0];
    let mut smallest_dist = distance_squared(
        match smallest {
            Shape::Cube(cube) => cube.position,
            Shape::Camera(_) => Vector3 {
                x: INFINITY,
                y: INFINITY,
                z: INFINITY,
            },
        },
        start,
    );
    for shape in root {
        let dist = distance_squared(
            match shape {
                Shape::Cube(cube) => cube.position,
                Shape::Camera(_) => Vector3 {
                    x: INFINITY,
                    y: INFINITY,
                    z: INFINITY,
                },
            },
            start,
        );

        if dist >= 0.0 && dist < smallest_dist {
            match shape {
                Shape::Cube(cube) => {
                    smallest = &Cube::new(
                        cube.position.x,
                        cube.position.y,
                        cube.position.z,
                        cube.dimensions.x,
                        cube.dimensions.y,
                        cube.dimensions.z,
                        cube.color,
                    );
                }
                Shape::Camera(camera) => {
                    smallest = &Shape::Camera(Camera::new(
                        camera.pos,
                        camera.fov,
                        camera.angle,
                        camera.dimensions,
                    ));
                }
            };
            smallest_dist = dist;
        }
    }
    limit - smallest_dist
}

pub struct Cube {
    position: Vector3,
    dimensions: Vector3,
    color: Color,
}

impl Cube {
    pub fn new(x: f32, y: f32, z: f32, width: f32, height: f32, depth: f32, color: Color) -> Shape {
        Shape::Cube(Cube {
            position: Vector3 { x: x, y: y, z: z },
            dimensions: Vector3 {
                x: width,
                y: height,
                z: depth,
            },
            color: color,
        })
    }
}
