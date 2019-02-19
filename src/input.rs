use specs::NullStorage;
use specs::WriteStorage;
use crate::graphics_00::Position;
use piston_window::Key;
use specs::WriteExpect;
use piston_window::Button::Keyboard;
use specs::{ ReadStorage, ReadExpect, Component, VecStorage, System };
use piston_window::Event;
use piston_window::Button;


#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardMotionControl;

//Resource storing what buttons have been pressed
#[derive(Debug)]
pub struct ArrowKeysPressed {

    pub up: bool,
    pub left: bool,
    pub right: bool,
    pub down: bool

}

pub struct KeyboardSystem;

impl<'a> System<'a> for KeyboardSystem {

    type SystemData = (
        ReadExpect<'a, ArrowKeysPressed>,
        ReadStorage<'a, KeyboardMotionControl>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (buttons, _, mut pos_dat) = data;

        for pos in (&mut pos_dat).join() {
            // println!("{}", buttons.up);
            // println!("{}", buttons.left);
            // println!("{}", buttons.right);
            // println!("{}", buttons.down);
            if buttons.up {
                pos.y += 0.1; //TODO make dynamic
            }

            if buttons.left {
                pos.x -= 0.1; //TODO make dynamic
            }

            if buttons.right {
                pos.x += 0.1; //TODO make dynamic
            }

            if buttons.down {
                pos.y -= 0.1; //TODO make dynamic
            }

        }

    }

}