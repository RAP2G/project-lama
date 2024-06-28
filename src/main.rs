use core::time::Duration;
use sdl2::event::*;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Rust!", 800, 600)
        .position_centered()
        .build()
        .expect("Fucked Up window");

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    println!("Everything worked if you seee this");

    Ok(())
}
