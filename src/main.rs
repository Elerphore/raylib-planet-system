#![allow(non_snake_case)]
#![allow(unused_parens)]

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1200, 900)
        .title("Hello, World")
        .build();
        
    let mut camera = Camera3D::perspective(
        Vector3::new(-5.0, 15.5, 0.0),
        Vector3::new(10.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        90.0,
    );
    
    rl.set_camera_mode(&camera, CameraMode::CAMERA_FREE);
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        rl.update_camera(&mut camera);
        
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::DARKGREEN);
        let mut d2 = d.begin_mode3D(camera);
        {
            d2.draw_sphere(Vector3::new(-16.0, 2.5, 0.0), 10.0, Color::RED);
            d2.draw_grid(10, 10.0)
        }
        
    }
}
