extern crate piston_window;

use piston_window::*;

struct Game {
    circle: [f64; 4],
}

impl Game {
    fn new() -> Self {
        Game {
            circle: [320.0, 240.0, 100.0, 100.0], // position and size
        }
    }

    fn update(&mut self) {
        // Update game state here
    }

    fn draw(&self, context: Context, graphics: &mut G2d) {
        clear([1.0; 4], graphics);
        ellipse(
            [1.0, 0.0, 0.0, 1.0], // red color
            self.circle,
            context.transform,
            graphics,
        );
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();

    while let Some(event) = window.next() {
        if let Some(_) = event.update_args() {
            game.update();
        }

        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |context, graphics, _device| {
                game.draw(context, graphics);
            });
        }
    }
}
