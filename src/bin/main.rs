use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use ben_celia_game::graphics_00::{ SpinRect, ScreenObject };
use ben_celia_game::specs_test;

fn main() {

    specs_test::specs_main();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [1440, 2800]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut rect = SpinRect {
        rotation: 0.0
    };

    let graphics_handle = &mut GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            rect.render(graphics_handle, &r);
        }

        if let Some(u) = e.update_args() {
            rect.update(&u);
        }
    }
}
