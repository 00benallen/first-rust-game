use piston::input::*;
use opengl_graphics::{ GlGraphics };

pub trait ScreenObject {

    fn render(&self, graphics_handle: &mut GlGraphics, args: &RenderArgs);

    fn update(&mut self, args: &UpdateArgs);

}

pub struct SpinRect {
    pub rotation: f64
}

impl ScreenObject for SpinRect {

    fn render(&self, graphics_handle: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.width / 2.0,
                      args.height / 2.0);

        graphics_handle.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
            .rot_rad(rotation)
            .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }


}