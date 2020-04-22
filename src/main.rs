mod app;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use app::App;

fn main() {
    // OpenGL version
    let opengl = OpenGL::V3_2;

    // Glutin window
    let mut window : Window = WindowSettings::new("square-go-wee", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    // Run the game
    let mut app = App::new(GlGraphics::new(opengl));

    // The event handler
    let mut events = Events::new(EventSettings::new());

    // Main cycle
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }
}
