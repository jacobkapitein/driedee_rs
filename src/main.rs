use core::engine::Engine;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod core;

fn main() -> Result<(), String> {
    let mut engine: Engine = Engine::new("3D Engine", 1280, 720);

    // Create the SDL event pump to handle events
    let mut event_pump = engine
        .sdl_context
        .event_pump()
        .expect("Error creating event pump");

    // Main loop
    let mut running = true;
    while running {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => running = false, // Exit the loop on window close
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => running = false, // Exit on Escape key
                _ => {}
            }
        }

        // Update the engine (call user-defined update logic)
        if !engine.on_user_update(0.016) {
            running = false;
        }

        // Simulate a frame delay (60 FPS)
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
