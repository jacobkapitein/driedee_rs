use clap::{Arg, ArgMatches, Command};
use core::engine::Engine;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod core;

fn main() -> Result<(), String> {
    let matches = set_commands();

    let object_to_load = matches
        .get_one::<String>("object")
        .cloned()
        .unwrap_or(String::from("./teapot.obj"));

    let mut engine: Engine = Engine::new("3D Engine", 1280, 720, object_to_load.as_str());

    // Create the SDL event pump to handle events
    let mut event_pump = engine
        .sdl_context
        .event_pump()
        .expect("Error creating event pump");

    // Set the target frame rate (FPS)
    let target_fps = matches.get_one::<f32>("max-fps").cloned().unwrap_or(60.0);
    let target_frame_duration = Duration::from_secs_f32(1.0 / target_fps);

    let mut pressed_keys: HashSet<Keycode> = HashSet::new();

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
                } => {
                    pressed_keys.insert(keycode);
                } //engine.move_camera(keycode, 0.016),
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    pressed_keys.remove(&keycode);
                }
                Event::Window {
                    win_event: WindowEvent::Resized(new_x, new_y),
                    ..
                } => engine.resize_window(new_x, new_y),
                _ => {}
            }
        }

        if pressed_keys.contains(&Keycode::W) {
            engine.move_camera(Keycode::W, 0.016);
        }
        if pressed_keys.contains(&Keycode::S) {
            engine.move_camera(Keycode::S, 0.016);
        }
        if pressed_keys.contains(&Keycode::A) {
            engine.move_camera(Keycode::A, 0.016);
        }
        if pressed_keys.contains(&Keycode::D) {
            engine.move_camera(Keycode::D, 0.016);
        }
        if pressed_keys.contains(&Keycode::UP) {
            engine.move_camera(Keycode::UP, 0.016);
        }
        if pressed_keys.contains(&Keycode::DOWN) {
            engine.move_camera(Keycode::DOWN, 0.016);
        }
        if pressed_keys.contains(&Keycode::LEFT) {
            engine.move_camera(Keycode::LEFT, 0.016);
        }
        if pressed_keys.contains(&Keycode::RIGHT) {
            engine.move_camera(Keycode::RIGHT, 0.016);
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

fn set_commands() -> ArgMatches {
    Command::new("driedee_rs")
        .about("A basic 3D engine application")
        .arg(
            Arg::new("max-fps")
                .long("max-fps")
                .value_parser(clap::value_parser!(f32))
                .default_value("60")
                .help("Sets the maximum frames per second"),
        )
        .arg(
            Arg::new("object")
                .long("object")
                .value_parser(clap::value_parser!(String))
                .default_value("./teapot.obj")
                .help("Load in a different obj file"),
        )
        .get_matches()
}
