use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

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

#[derive(Component)]
struct LeftMover;

struct LeftWalker;

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

struct State {
    ecs: World,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::new();
        ecs.register::<Position>();
        ecs.register::<Renderable>();
        ecs.register::<LeftMover>();
        ecs.register::<Player>();

        Self { ecs }
    }

    fn run_systems(&mut self) {
        LeftWalker.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        player_input(self, context);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        context.cls();

        for (pos, render) in (&positions, &renderables).join() {
            context.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
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

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut state = State::new();

    state
        .ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(bracket_lib::color::YELLOW),
            bg: RGB::named(bracket_lib::color::BLACK),
        })
        .with(Player)
        .build();

    for i in 0..10 {
        state
            .ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: to_cp437('☺'),
                fg: RGB::named(bracket_lib::color::RED),
                bg: RGB::named(bracket_lib::color::BLACK),
            })
            .with(LeftMover)
            .build();
    }

    main_loop(context, state)
}
