use ::core::f32;
use std::f32::INFINITY;

use raylib::prelude::*;

pub enum Shape {
    Cube(Cube),
    Camera(Camera),
}

#[derive(Clone, Copy)]
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
                    127.0,
                    root,
                );
                d.draw_pixel(
                    x,
                    y,
                    Color {
                        r: (distance*2.0 % 255.0) as u8,
                        g: (distance*2.0 % 255.0) as u8,
                        b: (distance*2.0 % 255.0) as u8,
                        a: 255,
                    },
                );
                y += 1;
            }
            x += 1;
        }
    }
}

#[derive(Clone, Copy)]
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

pub fn fire_ray(start: Vector3, angle: Angle3D, limit: f32, root: &Vec<Shape>) -> f32 {
    let mut total = 0.0;
    let mut position = start;
    let mut last_dist = 1.0;
    while last_dist > 0.01 {
        let mut smallest = &root[0];
        let mut smallest_dist = distance_squared(
            match smallest {
                Shape::Cube(cube) => cube.closest_point_on_cube(position),
                Shape::Camera(_) => Vector3 {
                    x: INFINITY,
                    y: INFINITY,
                    z: INFINITY,
                },
            },
            position,
        );
        for shape in root {
            let dist = distance_squared(
                match shape {
                    Shape::Cube(ref cube) => cube.closest_point_on_cube(position),
                    Shape::Camera(_) => Vector3 {
                        x: INFINITY,
                        y: INFINITY,
                        z: INFINITY,
                    },
                },
                position,
            );

            if dist >= 0.0 && dist < smallest_dist {
                match shape {
                    Shape::Cube(ref cube) => {
                        smallest = &Shape::Cube(*cube);
                    }
                    Shape::Camera(camera) => {
                        smallest = &Shape::Camera(*camera);
                    }
                };
                smallest_dist = dist;
            }
        }
        smallest_dist = f32::sqrt(smallest_dist);
        total += smallest_dist;
        // Vector((cos(yaw) cos(pitch), cos(yaw) sin(pitch), sin(yaw)))
        position = Vector3 {
            x: position.x
                + smallest_dist
                    * f32::cos(-angle.yaw / 360.0 * 2.0 * f32::consts::PI)
                    * f32::cos(-angle.pitch / 360.0 * 2.0 * f32::consts::PI),
            y: position.x
                + smallest_dist
                    * f32::cos(-angle.yaw / 360.0 * 2.0 * f32::consts::PI)
                    * f32::sin(-angle.pitch / 360.0 * 2.0 * f32::consts::PI),
            z: position.z + smallest_dist * f32::sin(-angle.yaw / 360.0 * 2.0 * f32::consts::PI),
        };
        if total >= limit {
            return limit;
        }
        last_dist = smallest_dist;
    }
    total
}

#[derive(Clone, Copy)]
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

    pub fn closest_point_on_cube(&self, to: Vector3) -> Vector3 {
        let mut final_x = 0.0;
        let mut final_y = 0.0;
        let mut final_z = 0.0;
        if to.x >= self.position.x && to.x <= self.position.x + self.dimensions.x {
            final_x = to.x;
        } else if to.x < self.position.x {
            final_x = self.position.x;
        } else {
            final_x = self.position.x + self.dimensions.x;
        }

        if to.y >= self.position.y && to.y <= self.position.y + self.dimensions.y {
            final_y = to.y;
        } else if to.y < self.position.y {
            final_y = self.position.y;
        } else {
            final_y = self.position.y + self.dimensions.y;
        }

        if to.z >= self.position.z && to.z <= self.position.z + self.dimensions.z {
            final_z = to.z;
        } else if to.z < self.position.z {
            final_z = self.position.z;
        } else {
            final_z = self.position.z + self.dimensions.z;
        }

        Vector3 { x: final_x, y: final_y, z: final_z }
    }
}
