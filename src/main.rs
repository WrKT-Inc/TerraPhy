
extern crate lazy_static;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate serde;

mod constants;
mod load_json;
mod equations;

use crate::constants::{PHYSICAL_CONSTS};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{RenderArgs, UpdateArgs};

const WINDOW_SIZE: [u32; 2] = [1366, 768];
const APP_NAME: &str = "TerraPhy";

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    square: Square,
    ground: Ground
}

struct Ground {
    width: f64,
    height: f64,
    color: [f32; 4],
}

struct Square {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    mass: f64,
    color: [f32; 4],
}


impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let ground_color: [f32; 4] = self.ground.color;
        let ground_y = args.window_size[1] as f64 - self.ground.height;
        let ground = rectangle::rectangle_by_corners(0.0, ground_y, self.ground.width, args.window_size[1] as f64);
        

        let square_color: [f32; 4] = self.square.color;
        let square = rectangle::rectangle_by_corners(self.square.x, self.square.y, self.square.x + self.square.width, self.square.y + self.square.height);
        
        //  Draw all objects
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            rectangle(ground_color, ground, c.transform, gl);
            rectangle(square_color, square, c.transform, gl);            
        });

    }
    fn update(&mut self, args: &UpdateArgs) {
        // Apply horizontal velocity to square object
        // self.square.x += 2.0 * args.dt;
        // println!("square_x: {}", self.square.x);

        // check if the square_y is on the ground, if not the apply gravity
        println!("square_y: {}", self.square.y);

        // apply gravity to square object
        let center_y = self.square.y + self.square.width / 2.0;
        if center_y < WINDOW_SIZE[1] as f64 - self.ground.height {
            let acc = 0.5 * 9.8 * self.square.mass * args.dt * args.dt;
            self.square.y += acc;
        }
    }
}

fn main() {
    // Set up OpenGL Engine
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(APP_NAME, WINDOW_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        square: Square {
            x: 0.0,
            y: 0.0,
            width: 50.0,
            height: 50.0,
            mass: 1000.0,
            color: [0.0, 0.0, 0.0, 1.0],
        },
        ground: Ground {
            width: WINDOW_SIZE[0] as f64 + 100.0,
            height: 100.0,
            color: [0.5, 0.3, 0.0, 1.0],
        },
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
            // get window size 
            // println!("window size: {:?}", args.window_size);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
