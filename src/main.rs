mod shapes;

use raylib::prelude::*;

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 1024;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("raycaster")
        .build();

    let mut root: Vec<shapes::Shape> = Vec::new();

    root.push(shapes::Cube::new(
        10.0,
        0.0,
        -0.5,
        5.0,
        5.0,
        1.0,
        Color {
            r: 0,
            g: 255,
            b: 255,
            a: 255,
        },
    ));

    root.push(shapes::Cube::new(
        50.0,
        -5.0,
        -5.0,
        10.0,
        10.0,
        10.0,
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        },
    ));

    let camera = shapes::Camera::new(
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        180,
        shapes::Angle3D {
            roll: 0.0,
            yaw: 0.0,
            pitch: 0.0,
        },
        Vector2 {
            x: (WIDTH as f32),
            y: (HEIGHT as f32),
        },
    );

    while !rl.window_should_close() {
        let d = rl.begin_drawing(&thread);
        camera.draw(d, &root);
    }
}
