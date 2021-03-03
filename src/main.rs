use bracket_lib::prelude::*;

mod color;
mod components;
mod glyph;
mod map;
mod monster_ai_system;
mod player;
mod rect;
mod visibility_system;

use components::{Monster, Name, Player, Position, Renderable, Viewshed};
use hecs::World;
use map::Map;
use monster_ai_system::monster_ai_system;
use visibility_system::visibility_system;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    pub world: World,
    pub runstate: RunState,
}

impl State {
    fn new() -> Self {
        let world = World::new();
        let runstate = RunState::Running;

        Self { world, runstate }
    }

    fn run_systems(&mut self) {
        monster_ai_system(&mut self.world);
        visibility_system(&mut self.world);
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

        context.cls_bg(color::bg());

        map::draw_map(&self.world, context);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut state = State::new();

    let map = Map::rooms_and_cooridors(80, 50);
    let (player_x, player_y) = map.get_player_starting_position();

    state.world.spawn((
        Player,
        Name("Player".into()),
        Position {
            x: player_x,
            y: player_y,
        },
        Renderable {
            glyph: glyph::player(),
            fg: color::player_fg(),
            bg: color::bg(),
        },
        Viewshed::with_range(8),
    ));

    let mut rng = RandomNumberGenerator::new();
    let mut to_spawn = Vec::new();
    for (i, room) in map.get_rooms().iter().enumerate() {
        let (x, y) = room.center();
        if (x, y) != (player_x, player_y) {
            let (glyph, name) = match rng.roll_dice(1, 2) {
                1 => (glyph::monster_goblin(), "Goblin"),
                _ => (glyph::monster_orc(), "Orc"),
            };

            to_spawn.push((
                Monster,
                Name(format!("{} #{}", name, i)),
                Position { x, y },
                Renderable {
                    glyph,
                    fg: color::monster_fg(),
                    bg: color::bg(),
                },
                Viewshed::with_range(8),
            ));
        }
    }
    state.world.spawn_batch(to_spawn);

    state.world.spawn((map,));

    main_loop(context, state)
}
