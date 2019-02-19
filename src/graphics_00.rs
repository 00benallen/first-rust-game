use crate::input::KeyboardMotionControl;
use piston_window::UpdateArgs;
use piston::input::RenderArgs;
use opengl_graphics::{ GlGraphics };
use specs::{ Builder, Component, ReadStorage, WriteStorage, System, VecStorage, World, ReadExpect, WriteExpect };

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Rectangle {

    width: f64,
    height: f64

}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {

    pub x: f64,
    pub y: f64

}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Spin {
    rotation: f64,
    angular_velocity: f64
}

pub fn register_spin_rect(world: &mut World) {

    world.create_entity()
        .with(Position { x: 0.0, y: 0.0 })
        .with(Spin { rotation: 0.0, angular_velocity: 0.5 })
        .with(Rectangle { width: 50.0, height: 50.0 })
        .with(KeyboardMotionControl {})
        .build();

    world.create_entity()
        .with(Position { x: 150.0, y: 0.0 })
        .with(Spin { rotation: 0.0, angular_velocity: 0.1 })
        .with(Rectangle { width: 50.0, height: 50.0 })
        .build();

    world.create_entity()
        .with(Position { x: 0.0, y: 150.0 })
        .with(Spin { rotation: 0.0, angular_velocity: 0.1 })
        .with(Rectangle { width: 50.0, height: 50.0 })
        .build();

    world.create_entity()
        .with(Position { x: 100.0, y: 350.0 })
        .with(Spin { rotation: 0.0, angular_velocity: 0.1 })
        .with(Rectangle { width: 50.0, height: 25.0 })
        .build();

    world.create_entity()
        .with(Position { x: 0.0, y: 250.0 })
        .with(Spin { rotation: 0.0, angular_velocity: 1.0 })
        .with(Rectangle { width: 50.0, height: 150.0 })
        .build();

    world.create_entity()
        .with(Position { x: 50.0, y: 50.0 })
        .with(Spin { rotation: 0.0, angular_velocity: 0.1 })
        .with(Rectangle { width: 50.0, height: 50.0 })
        .build();

}

pub struct DrawClear;

impl<'a> System<'a> for DrawClear {
    type SystemData = (
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>);

    fn run(&mut self, data: Self::SystemData) {

        let (mut g_handle, args) = data;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        use graphics::*;

        g_handle.draw(args.viewport(), |_, gl| {
            // Clear the screen.
            clear(GREEN, gl);

        });
    }
}



//Create a struct for using as a system, can totally have internal data btw
pub struct DrawRectangles;

impl<'a> System<'a> for DrawRectangles {
    type SystemData = (
        ReadStorage<'a, Position>, 
        ReadStorage<'a, Rectangle>,
        ReadStorage<'a, Spin>,
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>);

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (pos_dat, rect_dat, spin_dat, mut g_handle, args) = data;

        for (pos, rect, spin) in (&pos_dat, &rect_dat, &spin_dat).join() {
            
            use graphics::*;

            const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

            let square = rectangle::square(0.0, 0.0, 1.0);

            g_handle.draw(args.viewport(), |c, gl| {

                let transform = c.transform
                .trans(pos.x + rect.width / 2.0, pos.y + rect.height / 2.0)
                .rot_rad(spin.rotation)
                .trans(- rect.width / 2.0, - rect.height / 2.0)
                .scale(rect.width, rect.height);
                
                // Draw a box rotating around the middle of the screen.
                rectangle(RED, square, transform, gl);

            });

        }
    }
}

pub struct ApplySpin;

impl<'a> System<'a> for ApplySpin {
    type SystemData = (
        WriteStorage<'a, Spin>,
        ReadExpect<'a, UpdateArgs>);

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (mut spin_dat, args) = data;

        for mut spin in (&mut spin_dat).join() {
            
            spin.rotation += spin.angular_velocity * args.dt;

        }
    }
}