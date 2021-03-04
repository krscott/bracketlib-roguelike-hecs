use bracket_lib::prelude::{to_cp437, BTerm, RGB};
use hecs::World;

use crate::config::Config;

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

pub fn draw_ui(context: &mut BTerm, _world: &World, config: &Config) {
    // Demo bug
    context.draw_box(0, 43, 39, 6, config.ui_fg, config.ui_bg);

    // Demo bugfix
    draw_box_bugfix(context, 40, 43, 39, 6, config.ui_fg, config.ui_bg);
}
