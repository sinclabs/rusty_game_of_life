extern crate piston_window;

use crate::renderers::render::Render;
use crate::universe::Universe;
use piston_window::*;

const CELL_SIZE: f64 = 5.0;

pub fn new() -> GuiRenderer {
    GuiRenderer {
        window: WindowSettings::new("Game of Life", (500, 500))
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|_e| panic!("Couldn't open Window!")),
    }
}

pub struct GuiRenderer {
    window: PistonWindow,
}

impl Render for GuiRenderer {
    fn render(&mut self, universe: &mut Universe) {
        let get_rectangle_color = |is_cell_alive: bool| {
            if is_cell_alive {
                [1.0, 1.0, 1.0, 1.0] // White
            } else {
                [0.0, 0.0, 0.0, 1.0] // Black
            }
        };

        let mut current_x = 0.0;
        let mut current_y = 0.0;

        while let Some(event) = self.window.next() {
            self.window.draw_2d(&event, |context, graphics, _device| {
                current_x = 0.0;
                current_y = 0.0;

                universe.evolve();
                clear([1.0; 4], graphics);
                for row in 0..universe.cells.len() {
                    for col in 0..universe.cells[row].len() {
                        rectangle(
                            get_rectangle_color(universe.cells[row][col]),
                            [current_x, current_y, CELL_SIZE, CELL_SIZE],
                            context.transform,
                            graphics,
                        );
                        current_x += CELL_SIZE;
                    }
                    current_x = 0.0;
                    current_y += CELL_SIZE;
                }
            });
        }
    }
}
