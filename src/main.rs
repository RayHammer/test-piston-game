mod app;

use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};

use app::App;

fn main() {
    // OpenGL version
    let opengl = OpenGL::V3_2;
    
    // Run the game
    let mut app = App::new(opengl);

    // The event handler
    let mut events = Events::new(EventSettings::new());

    // Main cycle
    while let Some(e) = events.next(&mut app.window) {
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }
}
