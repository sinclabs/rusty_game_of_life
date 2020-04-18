use crate::renderers::render::Render;
use crate::universe::Universe;

pub struct ConsoleRenderer();

impl Render for ConsoleRenderer {
    fn render(&mut self, universe: &mut Universe) {
        let mut print_string = String::from("");
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
    }
}
