use bracket_lib::random::RandomNumberGenerator;

mod cliopt;
mod command;
mod components;
mod config;
mod damage_system;
mod despawn_entities_system;
mod gamelog;
mod gui;
mod inventory;
mod map;
mod map_indexing_system;
mod melee_combat_system;
mod monster_ai_system;
mod player;
mod prelude;
mod resource;
mod spawner;
mod visibility_system;

use damage_system::damage_system;
use despawn_entities_system::despawn_entities_system;
use inventory::{pickup_item_system, use_item_system};
use map_indexing_system::map_indexing_system;
use melee_combat_system::melee_combat_system;
use monster_ai_system::monster_ai_system;
use player::player_input;
use prelude::*;
use visibility_system::visibility_system;

const GAME_TITLE: &'static str = "Rusty-hecs Roguelike";

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    AiTurn,
    ShowInventory,
}

pub struct State {
    pub world: World,
    pub config: Config,
}

fn report_system_error<T>(res: anyhow::Result<T>) {
    match res {
        Ok(_) => {}
        Err(err) => {
            console::log(format!("System Error: {}", err));

            // Nightly-only
            console::log(format!("Backtrace: {}", err.backtrace()));
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let world = &mut self.world;

        // Actions
        report_system_error(use_item_system(world));
        visibility_system(world);
        monster_ai_system(world);
        report_system_error(melee_combat_system(world));
        report_system_error(damage_system(world));
        report_system_error(pickup_item_system(world));

        // Cleanup
        despawn_entities_system(world);
        map_indexing_system(world);
        world.clear_commands();
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        context.cls_bg(self.config.bg);

        let run_state = match self.world.resource_clone::<RunState>() {
            Ok(run_state) => run_state,
            Err(_err) => {
                console::log("Error: Missing RunState Entity");
                return;
            }
        };

        let next_run_state = match run_state {
            RunState::PreRun => {
                self.run_systems();
                RunState::AwaitingInput
            }
            RunState::AwaitingInput => match player_input(context, &mut self.world) {
                Ok(rs) => rs,
                res @ Err(_) => {
                    report_system_error(res);
                    RunState::AwaitingInput
                }
            },
            RunState::PlayerTurn => {
                self.run_systems();
                RunState::AiTurn
            }
            RunState::AiTurn => {
                self.run_systems();
                RunState::AwaitingInput
            }
            RunState::ShowInventory => {
                // TODO: Separate UI and input response
                match gui::ui_input(context, &mut self.world) {
                    gui::ItemMenuResult::Cancel => RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => RunState::ShowInventory,
                    gui::ItemMenuResult::Selected => RunState::PlayerTurn,
                }
            }
        };

        map::draw_map(context, &self.world, &self.config);
        gui::draw_ui(context, &self.world, &self.config);

        match run_state {
            RunState::ShowInventory => {
                gui::draw_inventory(context, &mut self.world, &self.config);
            }
            _ => {}
        }

        if let Some((_, run_state)) = self.world.query::<&mut RunState>().into_iter().next() {
            *run_state = next_run_state;
        }
    }
}

fn main() -> BError {
    let opts = cliopt::parse_opt_args()?;
    let config = opts.config;

    // Generate map
    let map = TileMap::rooms_and_cooridors(gui::MAP_VIEW_WIDTH as i32, gui::MAP_VIEW_HEIGHT as i32);

    // Create ECS World
    let mut world = World::new();

    // Add RNG
    world.spawn_resource(RandomNumberGenerator::new(), ())?;

    // Spawn Run State
    world.spawn_resource(RunState::PreRun, ())?;

    // Spawn Player
    let (player_x, player_y) = map.get_center_of_first_room();
    spawner::player(&mut world, &config, player_x, player_y)?;

    // Spawn Monsters and Items
    spawner::health_potion(&mut world, &config, player_x + 1, player_y);
    for room in map.get_rooms().iter().skip(1) {
        spawner::rng_room_entities(&mut world, &config, room)?;
    }

    // Spawn Map
    world.spawn_resource(map, ())?;

    // Spawn Game Log
    world.spawn_resource(GameLog::new(), ())?;
    GameLog::resource_push(&world, format!("Welcome to {}", GAME_TITLE))?;

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
