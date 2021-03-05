use bracket_lib::prelude::*;

mod cliopt;
mod command;
mod components;
mod config;
mod damage_system;
mod despawn_entities_system;
mod gamelog;
mod gui;
mod map;
mod map_indexing_system;
mod melee_combat_system;
mod monster_ai_system;
mod player;
mod rect;
mod visibility_system;

use command::clear_commands_system;
use components::{BlocksTile, CombatStats, Monster, Name, Player, Position, Renderable, Viewshed};
use config::Config;
use damage_system::damage_system;
use despawn_entities_system::despawn_entities_system;
use gamelog::GameLog;
use hecs::{Entity, Fetch, Query, World};
use map::Map;
use map_indexing_system::map_indexing_system;
use melee_combat_system::melee_combat_system;
use monster_ai_system::monster_ai_system;
use player::player_input;
use visibility_system::visibility_system;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    AiTurn,
}

impl RunState {
    pub fn from_world(world: &mut World) -> Option<Self> {
        let mut query = world.query::<&RunState>();
        query.into_iter().next().map(|(_ent, run_state)| *run_state)
    }
}

pub struct State {
    pub world: World,
    pub config: Config,
    pub run_state_entity: Entity,
    pub player_entity: Entity,
    pub map_entity: Entity,
}

impl State {
    fn run_systems(&mut self) {
        let world = &mut self.world;

        // Actions
        visibility_system(world);
        monster_ai_system(world);
        melee_combat_system(world);
        damage_system(world);

        // Cleanup
        despawn_entities_system(world);
        map_indexing_system(world);
        clear_commands_system(world);
    }

    fn get_run_state(&mut self) -> RunState {
        let mut query = self
            .world
            .query_one::<&RunState>(self.run_state_entity)
            .unwrap();
        let run_state = query.get().unwrap();

        *run_state
    }

    fn set_run_state(&mut self, new_run_state: RunState) {
        let mut query = self
            .world
            .query_one::<&mut RunState>(self.run_state_entity)
            .unwrap();
        let run_state = query.get().unwrap();

        *run_state = new_run_state;
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        let next_run_state = match self.get_run_state() {
            RunState::PreRun => {
                self.run_systems();
                RunState::AwaitingInput
            }
            RunState::AwaitingInput => {
                player_input(self, context, self.player_entity, self.map_entity)
            }
            RunState::PlayerTurn => {
                self.run_systems();
                RunState::AiTurn
            }
            RunState::AiTurn => {
                self.run_systems();
                RunState::AwaitingInput
            }
        };

        self.set_run_state(next_run_state);

        context.cls_bg(self.config.bg);

        map::draw_map(context, &self.world, &self.config, self.map_entity);
        gui::draw_ui(context, &self.world, &self.config);
    }
}

fn main() -> BError {
    let opts = cliopt::parse_opt_args()?;
    let config = opts.config;

    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut world = World::new();

    let map = Map::rooms_and_cooridors(gui::MAP_VIEW_WIDTH as i32, gui::MAP_VIEW_HEIGHT as i32);
    let (player_x, player_y) = map.get_player_starting_position();

    let player_entity = world.spawn((
        Player,
        Name("Player".into()),
        Position {
            x: player_x,
            y: player_y,
        },
        Renderable {
            glyph: config.player.glyph,
            fg: config.player.fg,
            bg: config.player.bg,
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
            let (renderable, name): (Renderable, &str) = match rng.roll_dice(1, 2) {
                1 => (config.goblin.clone().into(), "Goblin"),
                _ => (config.orc.clone().into(), "Orc"),
            };

            to_spawn.push((
                Monster,
                Name(format!("{} #{}", name, i)),
                Position { x, y },
                renderable,
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

    let run_state_entity = world.spawn((RunState::PreRun,));

    {
        let mut gamelog = GameLog::new();
        gamelog.push("Welcome to the Rusty-hecs Roguelike");
        world.spawn((gamelog,));
    }

    let state = State {
        world,
        config,
        run_state_entity,
        player_entity,
        map_entity,
    };

    main_loop(context, state)
}

pub fn with_query<Q: Query, F>(world: &World, mut callback: F)
where
    F: FnMut(Entity, <<Q as Query>::Fetch as Fetch>::Item),
{
    for (entity, components) in world.query::<Q>().into_iter() {
        callback(entity, components);
    }
}

// pub fn with_one<Q: Query, F>(world: &mut World, mut callback: F)
// where
//     F: FnMut(Entity, <<Q as Query>::Fetch as Fetch>::Item),
// {
//     if let Some((entity, components)) = world.query::<Q>().into_iter().next() {
//         callback(entity, components);
//     }
// }
