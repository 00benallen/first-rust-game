use specs::WriteStorage;
use crate::graphics_00::Position;
use specs::{ ReadStorage, ReadExpect, Component, VecStorage, System };


#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct KeyboardMotionControl {
    pub velocity: f64
}

//Resource storing what buttons have been pressed
#[derive(Debug, Default)]
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

        let (buttons, motion_control_dat, mut pos_dat) = data;

        for (motion_control, pos) in (&motion_control_dat, &mut pos_dat).join() {
            if buttons.up {
                pos.y -= motion_control.velocity;
            }

            if buttons.left {
                pos.x -= motion_control.velocity;
            }

            if buttons.right {
                pos.x += motion_control.velocity;
            }

            if buttons.down {
                pos.y += motion_control.velocity;
            }

        }

    }

}