use core::engine::Engine;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::{Instant, Duration};
use std::thread::sleep;

mod core;

fn main() -> Result<(), String> {
    let mut engine: Engine = Engine::new("3D Engine", 1280, 720);

    // Create the SDL event pump to handle events
    let mut event_pump = engine
        .sdl_context
        .event_pump()
        .expect("Error creating event pump");

    // Set the target frame rate (FPS)
    let target_fps = 60;
    let target_frame_duration = Duration::from_secs_f32(1.0 / target_fps as f32);

    // Main loop
    let mut running = true;
    let mut last_frame_time = Instant::now();
    while running {
        let now = Instant::now();
        let frame_duration = now.duration_since(last_frame_time);
        last_frame_time = now;
        let realtime_fps = 1.0 / frame_duration.as_secs_f32();
        engine
            .set_title(format!("3D Engine - {:.2?} FPS", realtime_fps))
            .ok();

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => running = false, // Exit the loop on window close
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => running = false, // Exit on Escape key
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => engine.move_camera(keycode, 0.016),
                Event::Window {
                    win_event: WindowEvent::Resized(new_x, new_y),
                    ..
                } => engine.resize_window(new_x, new_y),
                _ => {}
            }
        }

        // Update the engine (call user-defined update logic)
        if !engine.on_user_update() {
            running = false;
        }

        // Frame delay to limit FPS
        let frame_time = Instant::now().duration_since(last_frame_time);
        if frame_time < target_frame_duration {
            let sleep_duration = target_frame_duration - frame_time;
            sleep(sleep_duration);
        }
    }

    Ok(())
}
