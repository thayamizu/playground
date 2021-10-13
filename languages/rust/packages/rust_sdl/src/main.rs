use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::thread;
use std::time::Duration;
use sdl2::image::{LoadTexture};

const TITLE: &'static str = "My SDL Window";
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;


fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init()?;
    let video_subsys: sdl2::VideoSubsystem = sdl_ctx.video()?;
    let window: sdl2::video::Window = video_subsys
        .window(TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .resizable()
        .opengl()
        .allow_highdpi()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_ctx.event_pump()?;
    let pngFileName: String = String::from("test.png");
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(pngFileName)?;

    'main: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        canvas.copy(&texture, None, None).expect("Render failed");
        canvas.present();
        for ev in event_pump.poll_iter() {
           match ev {
                Event::Quit{..}
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'main
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    let position = canvas.window().position();
                    let x = sdl2::video::WindowPos::from(position.0 );
                    let y = sdl2::video::WindowPos::from(position.1- 1);
                    canvas.window_mut().set_position(x, y)
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                }=> {
                    let position = canvas.window().position();
                    let x = sdl2::video::WindowPos::from(position.0);
                    let y = sdl2::video::WindowPos::from(position.1 + 1 );
                    canvas.window_mut().set_position(x, y)
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let position = canvas.window().position();
                    let x = sdl2::video::WindowPos::from(position.0 - 1);
                    let y = sdl2::video::WindowPos::from(position.1);
                    canvas.window_mut().set_position(x, y)
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                }=> {
                    let position = canvas.window().position();
                    let x = sdl2::video::WindowPos::from(position.0 + 1);
                    let y = sdl2::video::WindowPos::from(position.1);
                    canvas.window_mut().set_position(x, y)
                },
                Event::KeyDown {
                    keycode: Some(Keycode::F11),
                    ..
                }=> {
                    let current_mode = canvas.window().fullscreen_state();
                    match current_mode {
                        sdl2::video::FullscreenType::True=>{
                            canvas.window_mut().set_fullscreen(sdl2::video::FullscreenType::Off);
                        },
                        _=>{
                            canvas.window_mut().set_fullscreen(sdl2::video::FullscreenType::True);
                        }
                    };
                },
                _=> {}
            };

            

            
        }
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    return Ok(());
}
