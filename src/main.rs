use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

// ==================
// =   Constants    =
// ==================

const MAP_WIDTH: usize = 80;
const MAP_HEIGHT: usize = 50;

const PLAYER_START_X: i32 = 40;
const PLAYER_START_Y: i32 = 25;

// ==================
// =     Types      =
// ==================

#[derive(Debug, PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

impl TileType {
    fn fg(&self) -> RGB {
        match self {
            TileType::Wall => RGB::from_f32(0.0, 1.0, 0.0),
            TileType::Floor => RGB::from_f32(0.5, 0.5, 0.5),
        }
    }

    fn bg(&self) -> RGB {
        RGB::from_f32(0.0, 0.0, 0.0)
    }

    fn glyph(&self) -> u16 {
        let char = match self {
            TileType::Wall => '#',
            TileType::Floor => '.',
        };

        to_cp437(char)
    }
}

// ==================
// =   Components   =
// ==================

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component, Debug)]
struct Player;

// ==================
// =    Systems     =
// ==================

// ==================
// =     State      =
// ==================

struct State {
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
        player_input(self, context);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Vec<TileType>>();

        context.cls();

        draw_map(&map, context);

        for (pos, render) in (&positions, &renderables).join() {
            context.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

// ==================
// =   Functions    =
// ==================

fn get_map_index(x: i32, y: i32) -> usize {
    let x = min(MAP_WIDTH, max(0, x as usize));
    let y = min(MAP_HEIGHT, max(0, y as usize));

    (y * MAP_WIDTH) + x
}

fn get_map_coords(index: usize) -> (i32, i32) {
    let x = (index % MAP_WIDTH) as i32;
    let y = (index / MAP_WIDTH) as i32;
    (x, min(y, MAP_HEIGHT as i32))
}

fn create_random_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; MAP_WIDTH * MAP_HEIGHT];

    // Map edge walls
    for x in 0..(MAP_WIDTH as i32) {
        map[get_map_index(x, 0)] = TileType::Wall;
        map[get_map_index(x, MAP_HEIGHT as i32 - 1)] = TileType::Wall;
    }
    for y in 0..(MAP_HEIGHT as i32) {
        map[get_map_index(0, y)] = TileType::Wall;
        map[get_map_index(MAP_WIDTH as i32 - 1, y)] = TileType::Wall;
    }

    // Random walls
    let mut rng = RandomNumberGenerator::new();

    let player_index = get_map_index(PLAYER_START_X, PLAYER_START_Y);
    for _ in 0..400 {
        let x = rng.roll_dice(1, MAP_WIDTH as i32 - 1);
        let y = rng.roll_dice(1, MAP_HEIGHT as i32 - 1);
        let i = get_map_index(x, y);

        if i != player_index {
            map[i] = TileType::Wall;
        }
    }

    map
}

fn draw_map(map: &[TileType], context: &mut BTerm) {
    for (i, tile) in map.iter().enumerate() {
        let (x, y) = get_map_coords(i);
        context.set(x, y, tile.fg(), tile.bg(), tile.glyph())
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let new_pos_index = get_map_index(pos.x + delta_x, pos.y + delta_y);
        match map[new_pos_index] {
            TileType::Wall => {
                // Do nothing
            }
            TileType::Floor => {
                let (x, y) = get_map_coords(new_pos_index);
                pos.x = x;
                pos.y = y;
            }
        }
    }
}

/// Check for player input and try to move Player entity
fn player_input(state: &mut State, context: &mut BTerm) {
    let delta_xy = match context.key {
        Some(VirtualKeyCode::Left) => Some((-1, 0)),
        Some(VirtualKeyCode::Right) => Some((1, 0)),
        Some(VirtualKeyCode::Up) => Some((0, -1)),
        Some(VirtualKeyCode::Down) => Some((0, 1)),
        _ => None,
    };

    if let Some((dx, dy)) = delta_xy {
        try_move_player(dx, dy, &mut state.ecs);
    }
}

// ==================
// =      Main      =
// ==================

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut state = State::new();

    state
        .ecs
        .create_entity()
        .with(Position {
            x: PLAYER_START_X,
            y: PLAYER_START_Y,
        })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(bracket_lib::color::YELLOW),
            bg: RGB::named(bracket_lib::color::BLACK),
        })
        .with(Player)
        .build();

    state.ecs.insert(create_random_map());

    main_loop(context, state)
}
