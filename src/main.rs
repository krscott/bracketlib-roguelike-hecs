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
use hecs::World;
use map::Map;
use map_indexing_system::map_indexing_system;
use melee_combat_system::melee_combat_system;
use monster_ai_system::monster_ai_system;
use player::player_input;
use visibility_system::visibility_system;

const GAME_TITLE: &'static str = "Rusty-hecs Roguelike";

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    AiTurn,
}

impl RunState {
    pub fn from_world(world: &World) -> Option<Self> {
        let mut query = world.query::<&RunState>();
        query.into_iter().next().map(|(_ent, run_state)| *run_state)
    }
}

pub struct State {
    pub world: World,
    pub config: Config,
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
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        let next_run_state = match RunState::from_world(&self.world).unwrap() {
            RunState::PreRun => {
                self.run_systems();
                RunState::AwaitingInput
            }
            RunState::AwaitingInput => player_input(self, context),
            RunState::PlayerTurn => {
                self.run_systems();
                RunState::AiTurn
            }
            RunState::AiTurn => {
                self.run_systems();
                RunState::AwaitingInput
            }
        };

        if let Some((_, run_state)) = self.world.query::<&mut RunState>().into_iter().next() {
            *run_state = next_run_state;
        }

        context.cls_bg(self.config.bg);

        map::draw_map(context, &self.world, &self.config);
        gui::draw_ui(context, &self.world, &self.config);
    }
}

fn main() -> BError {
    let opts = cliopt::parse_opt_args()?;
    let config = opts.config;

    // Generate map
    let map = Map::rooms_and_cooridors(gui::MAP_VIEW_WIDTH as i32, gui::MAP_VIEW_HEIGHT as i32);
    let (player_x, player_y) = map.get_player_starting_position();

    // Create ECS World
    let mut world = World::new();

    // Spawn Run State
    world.spawn((RunState::PreRun,));

    // Spawn Player
    world.spawn((
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

    // Spawn Monsters
    {
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
    }

    // Spawn Map
    world.spawn((map,));

    // Spawn Game Log
    world.spawn((GameLog::new(),));
    GameLog::push_world(&world, format!("Welcome to {}", GAME_TITLE));

    // Create terminal context
    let mut context = BTermBuilder::simple80x50().with_title(GAME_TITLE).build()?;

    if config.post_scanlines {
        context.with_post_scanlines(config.post_burnin.is_some());
    }

    if let Some(color) = config.post_burnin {
        context.screen_burn_color(color);
    }

    // Create State
    let state = State { world, config };

    // Start main loop
    main_loop(context, state)
}
