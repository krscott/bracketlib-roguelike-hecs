use bracket_lib::prelude::*;
use monster_ai_system::MonsterAi;
use specs::prelude::*;

mod color;
mod components;
mod glyph;
mod map;
mod monster_ai_system;
mod player;
mod rect;
mod visibility_system;

use components::{Monster, Name, Player, Position, Renderable, Viewshed};
use map::Map;
use visibility_system::VisibilitySystem;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::new();
        ecs.register::<Position>();
        ecs.register::<Renderable>();
        ecs.register::<Player>();
        ecs.register::<Viewshed>();
        ecs.register::<Monster>();
        ecs.register::<Name>();

        let runstate = RunState::Running;

        Self { ecs, runstate }
    }

    fn run_systems(&mut self) {
        VisibilitySystem.run_now(&self.ecs);
        MonsterAi.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        match self.runstate {
            RunState::Paused => {
                self.runstate = player::player_input(self, context);
            }
            RunState::Running => {
                self.run_systems();
                self.runstate = RunState::Paused;
            }
        }

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        context.cls_bg(color::bg());

        map::draw_map(&self.ecs, context);

        for (pos, render) in (&positions, &renderables).join() {
            if map.is_tile_visible(pos.x, pos.y) {
                context.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
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
        .with(Name("Player".into()))
        .build();

    let mut rng = RandomNumberGenerator::new();
    for (i, room) in map.get_rooms().iter().enumerate() {
        let (x, y) = room.center();
        if (x, y) != (player_x, player_y) {
            let (glyph, name) = match rng.roll_dice(1, 2) {
                1 => (glyph::monster_goblin(), "Goblin"),
                _ => (glyph::monster_orc(), "Orc"),
            };

            state
                .ecs
                .create_entity()
                .with(Position { x, y })
                .with(Renderable {
                    glyph,
                    fg: color::monster_fg(),
                    bg: color::bg(),
                })
                .with(Viewshed::with_range(8))
                .with(Monster)
                .with(Name(format!("{} #{}", name, i)))
                .build();
        }
    }

    state.ecs.insert(map);

    main_loop(context, state)
}
