use bracket_lib::prelude::*;

mod color;
mod components;
mod damage_system;
mod glyph;
mod map;
mod map_indexing_system;
mod melee_combat_system;
mod monster_ai_system;
mod player;
mod rect;
mod visibility_system;

use components::{BlocksTile, CombatStats, Monster, Name, Player, Position, Renderable, Viewshed};
use damage_system::{damage_system, delete_the_dead};
use hecs::{Entity, World};
use map::Map;
use map_indexing_system::map_indexing_system;
use melee_combat_system::melee_combat_system;
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
    pub player_entity: Entity,
    pub map_entity: Entity,
}

impl State {
    fn new(world: World, player_entity: Entity, map_entity: Entity) -> Self {
        let runstate = RunState::Running;

        Self {
            world,
            runstate,
            player_entity,
            map_entity,
        }
    }

    fn run_systems(&mut self) {
        visibility_system(&mut self.world, self.player_entity, self.map_entity);
        monster_ai_system(&mut self.world, self.player_entity, self.map_entity);
        melee_combat_system(&mut self.world);
        damage_system(&mut self.world);

        delete_the_dead(&mut self.world);
        map_indexing_system(&mut self.world, self.map_entity);
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        match self.runstate {
            RunState::Paused => {
                self.runstate =
                    player::player_input(self, context, self.player_entity, self.map_entity);
            }
            RunState::Running => {
                self.run_systems();
                self.runstate = RunState::Paused;
            }
        }

        context.cls_bg(color::bg());

        map::draw_map(context, &self.world, self.map_entity);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut world = World::new();

    let map = Map::rooms_and_cooridors(80, 50);
    let (player_x, player_y) = map.get_player_starting_position();

    let player_entity = world.spawn((
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
        CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        },
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
                BlocksTile,
                CombatStats {
                    max_hp: 16,
                    hp: 16,
                    defense: 1,
                    power: 4,
                },
            ));
        }
    }
    world.spawn_batch(to_spawn);

    let map_entity = world.spawn((map,));

    let state = State::new(world, player_entity, map_entity);

    main_loop(context, state)
}
