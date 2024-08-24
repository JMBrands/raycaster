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
                    127,
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

pub fn fire_ray(start: Vector3, angle: Angle3D, limit: i32) -> i32 {
    limit
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
