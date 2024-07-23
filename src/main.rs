// pub mod ecs;
pub mod game_context;
pub mod renderer;
use game_context::{GameContext, DOT_SIZE_IN_PXS, GRID_X_SIZE, GRID_Y_SIZE};
use renderer::Renderer;
// use core::time::Duration;
use sdl2::keyboard::Keycode;

use sdl2::event::*;
use std::time::Duration;
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window(
            "Snake Game",
            GRID_X_SIZE * DOT_SIZE_IN_PXS,
            GRID_Y_SIZE * DOT_SIZE_IN_PXS,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    // .expect("Fucked Up window");
    let mut renderer = Renderer::new(window)?;

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut context = GameContext::new();
    let mut frame_counter: u32 = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W => context.move_up(),
                    Keycode::S => context.move_down(),
                    Keycode::A => context.move_left(),
                    Keycode::D => context.move_right(),
                    Keycode::R => context.reset(),
                    Keycode::Space => context.toggle_pause(),

                    _ => {}
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        if frame_counter == 10 {
            context.next_tick();
            frame_counter = 0;
        } else {
            frame_counter += 1;
        }

        renderer.draw(&context)?;
    }
    println!("Everything worked if you seee this");

    Ok(())
}
