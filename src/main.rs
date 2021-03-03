use bracket_lib::prelude::*;
use specs::prelude::*;

mod color;
mod components;
mod glyph;
mod map;
mod player;
mod rect;
mod visibility_system;

use components::{Player, Position, Renderable, Viewshed};
use map::Map;
use visibility_system::VisibilitySystem;

pub struct State {
    ecs: World,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::new();
        ecs.register::<Position>();
        ecs.register::<Renderable>();
        ecs.register::<Player>();
        ecs.register::<Viewshed>();

        Self { ecs }
    }

    fn run_systems(&mut self) {
        VisibilitySystem.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        player::player_input(self, context);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        // let map = self.ecs.fetch::<Map>();

        context.cls_bg(color::bg());

        map::draw_map(&self.ecs, context);

        for (pos, render) in (&positions, &renderables).join() {
            context.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

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
            glyph: glyph::player(),
            fg: color::player_fg(),
            bg: color::bg(),
        })
        .with(Player)
        .with(Viewshed::with_range(8))
        .build();

    main_loop(context, state)
}
