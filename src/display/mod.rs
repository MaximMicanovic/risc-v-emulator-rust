#[cfg(feature = "display")]
use sdl2::pixels::Color;

#[cfg(feature = "display")]
pub fn make_window(c: &str, width: u32, height: u32) -> sdl2::video::Window {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    video_subsystem
        .window(c, width, height)
        .position(
            sdl2::video::WindowPos::Undefined as i32,
            sdl2::video::WindowPos::Undefined as i32,
        )
        .build()
        .unwrap()
}

#[cfg(feature = "display")]
pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Example: 0", 400, 400)
        .position(
            sdl2::video::WindowPos::Undefined as i32,
            sdl2::video::WindowPos::Undefined as i32,
        )
        .build()
        .unwrap();

    if false {
        println!("FAILED TO OPEN WINDOW");
        std::process::exit(1);
    }

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGBA(255, 255, 255, 250));
    canvas.clear();
    canvas.present();
    std::thread::sleep(std::time::Duration::from_millis(23000));
}
