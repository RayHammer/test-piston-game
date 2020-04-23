extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};
use piston::window::WindowSettings;

pub struct App {
    pub window: Window,
    gl: GlGraphics,
    rotation: f64,
}

impl App {
    // Constructor
    pub fn new(opengl : OpenGL) -> App {
        App {
            window: WindowSettings::new("square-go-wee", [200, 200])
                .graphics_api(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap(),
            gl: GlGraphics::new(opengl),
            rotation: 0.0
        }
    }

    // Game logic goes here
    pub fn update(&mut self, args: &UpdateArgs) {
        // 2 degrees per second
        self.rotation += 2.0 * args.dt;
    }

    // Drawing logic goes here
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0,
            args.window_size[1] / 2.0);
        
        self.gl.draw(args.viewport(),  |c, gl| {
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);
            
            rectangle(RED, square, transform, gl)
        });
    }
}