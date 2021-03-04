use bracket_lib::prelude::{to_cp437, BTerm, RGB};
use hecs::World;

use crate::{
    components::{CombatStats, Player},
    config::Config,
    gamelog::GameLog,
};

pub const MAP_VIEW_WIDTH: usize = 80;
pub const MAP_VIEW_HEIGHT: usize = 43;

/// Fix draw_box bug which fills box with #000000 instead of bg
fn draw_box_bugfix(context: &mut BTerm, x: i32, y: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
    context.draw_box(x, y, width, height, fg, bg);

    let blank = to_cp437(' ');
    for x in (x + 1)..(x + width) {
        for y in (y + 1)..(y + height) {
            context.set(x, y, fg, bg, blank);
        }
    }
}

pub fn draw_ui(context: &mut BTerm, world: &World, config: &Config) {
    draw_box_bugfix(context, 0, 43, 79, 6, config.ui.fg, config.ui.bg);

    if let Some((_, (_, stats))) = world.query::<(&Player, &CombatStats)>().into_iter().next() {
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        context.print_color(12, 43, config.ui_hp.fg, config.ui_hp.bg, &health);
        context.draw_bar_horizontal(
            28,
            43,
            51,
            stats.hp,
            stats.max_hp,
            config.ui_hp_bar.fg,
            config.ui_hp_bar.bg,
        );
    }

    if let Some((_, log)) = world.query::<&GameLog>().into_iter().next() {
        for (i, msg) in log.entries.iter().rev().enumerate() {
            let y = 44 + i;
            context.print(2, y, msg);
            if y >= 48 {
                break;
            }
        }
    }
}
