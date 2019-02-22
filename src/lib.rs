extern crate specs;

#[macro_use]
extern crate specs_derive;

extern crate piston_window;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

pub mod graphics_00;
pub mod input;

use crate::input::ArrowKeysPressed;
use piston_window::Button::Keyboard;
use crate::input::KeyboardSystem;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use graphics_00::{ DrawClear, DrawRectangles, ApplySpin, register_spin_rect };
use specs::{ DispatcherBuilder, World, Dispatcher };

use std::env;


pub fn start() {

    let mut world = World::new();

    let ( mut render_dispatcher, mut window) = setup_graphics(&mut world);

    let mut update_dispatcher = DispatcherBuilder::new()
        .with(ApplySpin, "apply_spin", &[])
        .with(KeyboardSystem, "keyboard_system", &[])
        .build();
    update_dispatcher.setup(&mut world.res);

    world.add_resource(ArrowKeysPressed { up: false, left: false, right: false, down: false });

    register_spin_rect(&mut world);

    let mut events = Events::new(EventSettings::new());

    let debug = env::var("DEBUG").is_ok();
    while let Some(e) = events.next(&mut window) {

        if let Some(r) = e.render_args() {
            world.add_resource(r);
            render_dispatcher.dispatch(&mut world.res);
            world.maintain();
        }

        if let Some(Keyboard(k)) = e.press_args() {
            println!("Keyboard key pressed");
            if k == Key::Up {
                println!("up");
                world.write_resource::<ArrowKeysPressed>().up = true;
            }

            if k == Key::Left {
                println!("left");
                world.write_resource::<ArrowKeysPressed>().left = true;
            }

            if k == Key::Right {
                println!("right");
                world.write_resource::<ArrowKeysPressed>().right = true;
            }

            if k == Key::Down {
                println!("down");
                world.write_resource::<ArrowKeysPressed>().down = true;
            }

        }

        if let Some(Keyboard(k)) = e.release_args() {
            println!("Keyboard key pressed");
            if k == Key::Up {
                println!("up");
                world.write_resource::<ArrowKeysPressed>().up = false;
            }

            if k == Key::Left {
                println!("left");
                world.write_resource::<ArrowKeysPressed>().left = false;
            }

            if k == Key::Right {
                println!("right");
                world.write_resource::<ArrowKeysPressed>().right = false;
            }

            if k == Key::Down {
                println!("down");
                world.write_resource::<ArrowKeysPressed>().down = false;
            }

        }

        if let Some(u) = e.update_args() {
            world.add_resource(u);
            update_dispatcher.dispatch(&mut world.res);
            world.maintain();
        }

        if debug {
            debug_input_event(e);
        }
    }
}

fn setup_graphics(world: &mut World) -> (Dispatcher<'static, 'static>, Window) {

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let window: Window = WindowSettings::new(
            "spinning-square",
            [1440, 2800]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let graphics_handle = GlGraphics::new(opengl);

    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(DrawClear)
        .with_thread_local(DrawRectangles)
        .build();

    world.add_resource(graphics_handle);

    dispatcher.setup(&mut world.res);

    (dispatcher, window)
}

fn debug_input_event(e: Event) {

    if let Some(Button::Mouse(button)) = e.press_args() {
            println!("Pressed mouse button '{:?}'", button);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            println!("Pressed keyboard key '{:?}'", key);
        }

        if let Some(button) = e.release_args() {
            match button {
                Button::Keyboard(key) => println!("Released keyboard key '{:?}'", key),
                Button::Mouse(button) => println!("Released mouse button '{:?}'", button),
                Button::Controller(button) => println!("Released controller button '{:?}'", button),
                Button::Hat(hat) => println!("Released controller hat `{:?}`", hat),
            }
        }
       
        e.mouse_cursor(|x, y| {
            println!("Mouse moved '{} {}'", x, y);
        });
        
        e.mouse_scroll(|dx, dy| println!("Scrolled mouse '{}, {}'", dx, dy));
        e.mouse_relative(|dx, dy| println!("Relative mouse moved '{} {}'", dx, dy));
        e.text(|text| println!("Typed '{}'", text));
        e.resize(|w, h| println!("Resized '{}, {}'", w, h));
        if let Some(cursor) = e.cursor_args() {
            if cursor { println!("Mouse entered"); }
            else { println!("Mouse left"); }
        };

}