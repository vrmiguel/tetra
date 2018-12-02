extern crate tetra;

use tetra::error::Result;
use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};

struct GameState;

impl State for GameState {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context, _dt: f64) {
        // Cornflour blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
    }
}

fn main() -> Result {
    let ctx = &mut ContextBuilder::new()
        .title("Hello, world!")
        .quit_on_escape(true)
        .build()?;

    let state = &mut GameState;

    tetra::run(ctx, state)
}
