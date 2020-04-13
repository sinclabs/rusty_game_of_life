mod game;
mod renderers;

use game::universe;
use renderers::{Render, console, gui};
use std::{thread, time};

fn main() {
    let sleep_time = time::Duration::from_millis(150);

    let universe_size = 100;

    let mut universe = universe::new(universe_size, universe::StartState::RandomAlive);
 
    let mut console_renderer = console::ConsoleRenderer();

    let mut gui_renderer = gui::new();

    // loop {
    //     universe.evolve();
    //     console_renderer.render(&mut universe);
    //     thread::sleep(sleep_time);
    // }

    gui_renderer.render(&mut universe);
}
