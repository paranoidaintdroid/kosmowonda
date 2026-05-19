use std::thread::sleep;
use std::time::{Duration, Instant};

use kosmo_math::Vec2;
use kosmo_math::integrator::symplectic_euler_step;
use kosmo_physics::{Body, accumulate_forces_adaptive};
use kosmo_render::Renderer;

fn main() {
    let mut renderer = Renderer::new("Kosmo", 800, 600);

    let mut bodies = vec![
        Body::new(50000.0, Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)),
        Body::new(0.01, Vec2::new(8.0, 0.0), Vec2::new(0.0, 17.5)),
        Body::new(0.02, Vec2::new(12.0, 0.0), Vec2::new(0.0, 14.4)),
        Body::new(0.025, Vec2::new(16.0, 0.0), Vec2::new(0.0, 12.5)),
        Body::new(0.015, Vec2::new(22.0, 0.0), Vec2::new(0.0, 10.7)),
        Body::new(5.0, Vec2::new(34.0, 0.0), Vec2::new(0.0, 8.3)),
        Body::new(3.0, Vec2::new(46.0, 0.0), Vec2::new(0.0, 7.1)),
        Body::new(1.5, Vec2::new(58.0, 0.0), Vec2::new(0.0, 6.2)),
        Body::new(1.7, Vec2::new(70.0, 0.0), Vec2::new(0.0, 5.6)),
    ];

    let dt = 0.0005;
    let world_scale = 12.0;

    let target_frame_time = Duration::from_millis(16); // use 16 for 60fps

    while renderer.is_open() {
        let frame_start = Instant::now();

        accumulate_forces_adaptive(&mut bodies);

        for body in bodies.iter_mut() {
            let (new_pos, new_vel) =
                symplectic_euler_step(body.position, body.velocity, body.acceleration, dt);

            body.position = new_pos;
            body.velocity = new_vel;
        }

        renderer.clear(0x000000);

        for (i, body) in bodies.iter().enumerate() {
            let (x, y) = renderer.world_to_screen(body.position, world_scale);

            let radius = match i {
                0 => 8,
                5 => 5,
                6 => 4,
                _ => 2,
            };

            let color = match i {
                0 => 0xFFFF00,
                1 => 0xAAAAAA,
                2 => 0xFFD700,
                3 => 0x00AAFF,
                4 => 0xFF3300,
                5 => 0xFFA500,
                6 => 0xCCCC88,
                7 => 0x66FFFF,
                _ => 0x3333FF,
            };

            renderer.draw_circle(x as isize, y as isize, radius, color);
        }

        renderer.present();

        let frame_time = frame_start.elapsed();

        if frame_time < target_frame_time {
            sleep(target_frame_time - frame_time);
        }
    }
}
