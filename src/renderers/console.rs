use crate::renderers::render::Render;
use crate::universe::Universe;
use std::{thread, time};

const FPS: u64 = 60;

pub struct ConsoleRenderer();

impl Render for ConsoleRenderer {
    fn render(&mut self, universe: &mut Universe) {
        let sleep_time = time::Duration::from_millis(1000 / FPS);
        let mut print_string = String::from("");

        loop {
            universe.evolve();

            let get_display_string = |row: usize, col: usize| {
                if universe.cells[row][col] {
                    "■ "
                } else {
                    "□ "
                }
            };

            print!("\x1B[2J"); // Clears the console

            for row in 0..universe.cells.len() {
                for col in 0..universe.cells[row].len() {
                    print_string += get_display_string(row, col);
                }
                print_string += "\n";
            }

            println!("{}", print_string);
            
            thread::sleep(sleep_time);
        }
    }
}
