#![allow(non_snake_case)]
#![allow(unused_parens)]

use raylib::{prelude::*};
use raylib::ffi;

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
    
    let sunRadius = 4.0;
    let earthRadius = 0.6;
    let earthOrbitRadius = 8.0;
    let moonRadius = 0.16;
    let moonOrbitRadius = 1.5;
    
    let rotationSpeed = 0.2;         // General system rotation speed

    let mut earthRotation = 0.0;         // Rotation of earth around itself (days) in degrees
    let mut earthOrbitRotation = 0.0;    // Rotation of earth around the Sun (years) in degrees
    let mut moonRotation = 0.0;          // Rotation of moon around itself
    let mut moonOrbitRotation = 0.0;     // Rotation of moon around earth in degrees
    
    let mut vector_planet = Vector3::new(-16.0, 2.0, 0.0);

    while !rl.window_should_close() {
        if (rl.is_key_down(KeyboardKey::KEY_RIGHT))  { vector_planet.x += 0.1; }
        if (rl.is_key_down(KeyboardKey::KEY_LEFT))  { vector_planet.x -= 0.1; }
        if (rl.is_key_down(KeyboardKey::KEY_DOWN))  { vector_planet.y  -= 0.1; }
        if (rl.is_key_down(KeyboardKey::KEY_UP))  { vector_planet.y += 0.1; }
        rl.update_camera(&mut camera);
        
        earthRotation += (5.0*rotationSpeed);
        earthOrbitRotation += (365.0/360.0*(5.0*rotationSpeed)*rotationSpeed);
        moonRotation += (2.0*rotationSpeed);
        moonOrbitRotation += (8.0*rotationSpeed);
        
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::DARKGREEN);
        let mut d2 = d.begin_mode3D(camera);
        {
            
            // draw a fucking sun, I don't understand this, like at all, Like it's a fukin magic
            
            unsafe {
                ffi::rlPushMatrix();
                    ffi::rlScalef(sunRadius, sunRadius, sunRadius);          // Scale Sun
                    DrawSphereBasic(Color::GOLD);                              // Draw the Sun
                ffi::rlPopMatrix();
            
                ffi::rlPushMatrix();
                    ffi::rlRotatef(earthOrbitRotation, 0.0, 1.0, 0.0);    // Rotation for Earth orbit around Sun
                    ffi::rlTranslatef(earthOrbitRadius, 0.0, 0.0);         // Translation for Earth orbit
                    ffi::rlRotatef(-earthOrbitRotation, 0.0, 1.0, 0.0);   // Rotation for Earth orbit around Sun inverted

                    ffi::rlPushMatrix();
                        ffi::rlRotatef(earthRotation, 0.25, 1.0, 0.0);       // Rotation for Earth itself
                        ffi::rlScalef(earthRadius, earthRadius, earthRadius);// Scale Earth

                        DrawSphereBasic(Color::BLUE);                          // Draw the Earth
                    ffi::rlPopMatrix();
            
                ffi::rlRotatef(moonOrbitRotation, 0.0, 1.0, 0.0);     // Rotation for Moon orbit around Earth
                ffi::rlTranslatef(moonOrbitRadius, 0.0, 0.0);          // Translation for Moon orbit
                ffi::rlRotatef(-moonOrbitRotation, 0.0, 1.0, 0.0);    // Rotation for Moon orbit around Earth inverted
                ffi::rlRotatef(moonRotation, 0.0, 1.0, 0.0);          // Rotation for Moon itself
                ffi::rlScalef(moonRadius, moonRadius, moonRadius);       // Scale Moon

                DrawSphereBasic(Color::LIGHTGRAY);                         // Draw the Moon
            ffi::rlPopMatrix();
            }
            d2.draw_circle_3D(rvec3( 0.0, 0.0, 0.0 ), earthOrbitRadius, rvec3( 1, 0,0 ), 90.0, Color::RED.fade( 0.5));
            d2.draw_grid(10, 10.0)
        }
        
    }
}

fn DrawSphereBasic(color: Color)
{
    let rings = 16;
    let slices = 16;

    unsafe {
        ffi::rlBegin(ffi::RL_TRIANGLES as i32);
            ffi::rlColor4ub(color.r, color.g, color.b, color.a);
    
            for i in 0..(rings + 2)
            {
                for j in 0..slices
                {
                    let deg2rad: f32 = 0.017453292519943295;
                    ffi::rlVertex3f((deg2rad*(270+(180/(rings + 1))*i) as f32).cos()*(deg2rad*(j*360/slices) as f32).sin(),
                               (deg2rad*(270+(180/(rings + 1))*i) as f32).sin(),
                               (deg2rad*(270+(180/(rings + 1))*i) as f32).cos()*(deg2rad*((j*360/slices) as f32).cos()));
                    ffi::rlVertex3f((deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).cos()*(deg2rad*((j+1)*360/slices) as f32).sin(),
                               (deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).sin(),
                               (deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).cos()*(deg2rad*((j+1)*360/slices) as f32).cos());
                    ffi::rlVertex3f((deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).cos()*(deg2rad*(j*360/slices) as f32).sin(),
                               (deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).sin(),
                               (deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).cos()*(deg2rad*((j*360/slices) as f32).cos()));
    
                    ffi::rlVertex3f((deg2rad*(270+(180/(rings + 1))*i) as f32).cos()*(deg2rad*(j*360/slices) as f32).sin(),
                               (deg2rad*(270+(180/(rings + 1))*i) as f32).sin(),
                               (deg2rad*(270+(180/(rings + 1))*i) as f32).cos()*(deg2rad*(j*360/slices) as f32).cos());
                    ffi::rlVertex3f((deg2rad*(270+(180/(rings + 1))*(i)) as f32).cos()*(deg2rad*((j+1)*360/slices) as f32).sin(),
                               (deg2rad*(270+(180/(rings + 1))*(i)) as f32).sin(),
                               (deg2rad*(270+(180/(rings + 1))*(i)) as f32).cos()*(deg2rad*((j+1)*360/slices) as f32).cos());
                    ffi::rlVertex3f((deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).cos()*(deg2rad*((j+1)*360/slices) as f32).sin(),
                               (deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).sin(),
                               (deg2rad*(270+(180/(rings + 1))*(i+1)) as f32).cos()*(deg2rad*((j+1)*360/slices) as f32).cos());
                }
            }
        ffi::rlEnd();
    }

}
