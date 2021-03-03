use bracket_lib::prelude::*;
use specs::prelude::*;

mod components;
mod map;
mod player;
mod rect;

use components::{Player, Position, Renderable};
use map::Map;

pub struct State {
    ecs: World,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::new();
        ecs.register::<Position>();
        ecs.register::<Renderable>();
        ecs.register::<Player>();

        Self { ecs }
    }

    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        player::player_input(self, context);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        context.cls();

        map.draw_to_context(context);

        for (pos, render) in (&positions, &renderables).join() {
            context.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

// ==================
// =   Functions    =
// ==================

// ==================
// =      Main      =
// ==================

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut state = State::new();

    let map = Map::rooms_and_cooridors(80, 50);
    let (player_x, player_y) = map.get_player_starting_position();
    state.ecs.insert(map);

    state
        .ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(bracket_lib::color::YELLOW),
            bg: RGB::named(bracket_lib::color::BLACK),
        })
        .with(Player)
        .build();

    main_loop(context, state)
}
