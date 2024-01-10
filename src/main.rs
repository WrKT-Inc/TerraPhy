extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{RenderArgs, UpdateArgs};

const WINDOW_SIZE: [u32; 2] = [1024, 768];
const APP_NAME: &str = "TerraPhy";

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    square_mass: f64,
    square_x: f64,  // Keeps track of where the square is
    square_y: f64,
    square_width: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const SQUARE_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        // create static brown ground
        const GROUND_COLOR: [f32; 4] = [0.5, 0.3, 0.0, 1.0];
        let ground_width = 1024.0;
        let ground_height = 100.0;
        let ground = rectangle::rectangle_by_corners(0.0, 0.0, ground_width, ground_height);
        let ground_y = args.window_size[1] as f64 - ground_height;

        let square = rectangle::square(0.0, 0.0, self.square_width);
        let square_x = self.square_x;
        let square_y = self.square_y;

        //  Draw the square to screen.
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            rectangle(SQUARE_COLOR, square, c.transform.trans(square_x, square_y), gl);
            rectangle(GROUND_COLOR, ground, c.transform.trans(0.0, ground_y), gl);
        });

    }
    fn update(&mut self, args: &UpdateArgs) {
        self.square_x += 2.0 * args.dt;

    //     check if the square_y is on the ground, if not the apply gravity
        println!("square_x: {}", self.square_x);
        println!("square_y: {}", self.square_y);

        // apply based on square mass
        if self.square_y < 668.0 {
            let acc = 0.5 * 9.8 * self.square_mass * args.dt * args.dt;
            self.square_y += acc;
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(APP_NAME, WINDOW_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        square_mass: 1000.0,
        square_x: 0.0,
        square_y: 0.0,
        square_width: 50.0,
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
