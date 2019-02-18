extern crate specs;

#[macro_use]
extern crate specs_derive;

extern crate piston_window;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

pub mod graphics_00;
pub mod specs_test;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use graphics_00::{ DrawClear, DrawRectangles, ApplySpin, register_spin_rect };
use specs::{ DispatcherBuilder, World, Dispatcher };


pub fn start() {

    let mut world = World::new();

    let ( mut render_dispatcher, mut window) = setup_graphics(&mut world);

    let mut update_dispatcher = DispatcherBuilder::new() //dispatches all given systems in order
        .with(ApplySpin, "apply_spin", &[])
        .build();
    update_dispatcher.setup(&mut world.res);

    // Only the second entity will get a position update,
    // because the first one does not have a velocity.
    register_spin_rect(&mut world);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        if let Some(r) = e.render_args() {
            world.add_resource(r);
            render_dispatcher.dispatch(&mut world.res); //dispatch all systems
            world.maintain(); //update entity registry with any changes that occured in the Systems
        }

        if let Some(u) = e.update_args() {
            update_dispatcher.dispatch(&mut world.res);
            world.maintain();
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

    let mut dispatcher = DispatcherBuilder::new() //dispatches all given systems in order
        .with_thread_local(DrawClear)
        .with_thread_local(DrawRectangles)
        // .with(UpdatePos, "update_pos", &["hello_world"]) //update position will run after hello world has run
        // .with(HelloWorld, "hello_updated", &["update_pos"]) //hello_updated will run after update_pos has run
        //.with_thread_local(RenderSys); this is here to remind how to render components properly
        .build();

    world.add_resource(graphics_handle);

    dispatcher.setup(&mut world.res); //register all Components, setup any Resources with Default implementations

    (dispatcher, window)
}