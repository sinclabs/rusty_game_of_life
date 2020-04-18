mod game;
mod renderers;

use game::universe;
use renderers::{console, gui, Render};
use std::env;
use std::{thread, time};

const FPS: u64 = 60;

enum GameRenderer {
    ConsoleRenderer(console::ConsoleRenderer),
    GUIRenderer(gui::GuiRenderer),
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut renderer: GameRenderer;

    if args.len() > 1 && args[1] == "console" {
        renderer = GameRenderer::ConsoleRenderer(console::ConsoleRenderer());
    } else {
        renderer = GameRenderer::GUIRenderer(gui::new());
    }

    let universe_size = 100;
    let mut universe = universe::new(universe_size, universe::StartState::RandomAlive);

    match renderer {
        GameRenderer::GUIRenderer(ref mut gui_renderer) => {
            gui_renderer.render(&mut universe);
        }
        GameRenderer::ConsoleRenderer(ref mut console_renderer) => {
            let sleep_time = time::Duration::from_millis(1000 / FPS);
            loop {
                universe.evolve();
                console_renderer.render(&mut universe);
                thread::sleep(sleep_time);
            }
        }
    }
}
