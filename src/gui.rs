use bracket_lib::prelude::{to_cp437, BTerm, FontCharType, VirtualKeyCode, RGB};
use hecs::World;

use crate::{
    components::{CombatStats, Name},
    config::Config,
    gamelog::GameLog,
    inventory::InInventory,
    map::Map,
    player::Player,
};

pub const MAP_VIEW_WIDTH: usize = 80;
pub const MAP_VIEW_HEIGHT: usize = 43;

const TOOLTIP_HORIZONTAL_PADDING: i32 = 1;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected,
}

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

pub fn ui_input(context: &mut BTerm) -> ItemMenuResult {
    match context.key {
        Some(VirtualKeyCode::Escape) => ItemMenuResult::Cancel,
        _ => ItemMenuResult::NoResponse,
    }
}

pub fn draw_ui(context: &mut BTerm, world: &World, config: &Config) {
    draw_box_bugfix(context, 0, 43, 79, 6, config.ui.fg, config.ui.bg);

    if let Some((_, (_, stats))) = world.query::<(&Player, &CombatStats)>().into_iter().next() {
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        context.print_color(12, 43, config.ui_title.fg, config.ui_title.bg, &health);
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

    // let mouse_pos = context.mouse_pos();
    // context.set_bg(mouse_pos.0, mouse_pos.1, config.ui_tooltip.bg);

    draw_tooltips(context, world, config);
}

fn draw_tooltips(context: &mut BTerm, world: &World, config: &Config) {
    let (mx, my) = context.mouse_pos();

    for (_, map) in world.query::<&mut Map>().into_iter() {
        if !map.is_tile_visible(mx, my) {
            continue;
        }

        let tooltip = map
            .get_entities_on_tile(mx, my)
            .iter()
            .filter_map(|entity| {
                let mut query = world.query_one::<&Name>(*entity).ok()?;
                let Name(name) = query.get().expect("Unfiltered query");
                Some(name.clone())
            })
            .collect::<Vec<_>>();

        let tooltip_width = 2 * TOOLTIP_HORIZONTAL_PADDING
            + tooltip.iter().map(|s| s.len() as i32).max().unwrap_or(0);

        let tooltip_height = tooltip.len() as i32;

        let tooltip_x = i32::max(
            0,
            if mx >= map.get_width() - tooltip_width {
                mx - tooltip_width
            } else {
                mx + 1
            },
        );

        let tooltip_y = i32::max(0, i32::min(my, map.get_height() - tooltip_height));

        for (i, s) in tooltip.into_iter().enumerate() {
            context.print_color(
                tooltip_x,
                tooltip_y + i as i32,
                config.ui_tooltip.fg,
                config.ui_tooltip.bg,
                format!(" {:<pad$}", s, pad = (tooltip_width - 1) as usize),
            );
        }
    }
}

pub fn draw_inventory(context: &mut BTerm, world: &World, config: &Config) {
    for (player_entity, _) in world.query::<&Player>().into_iter() {
        let mut player_inventory = world.query::<(&InInventory, &Name)>();
        let player_inventory = player_inventory
            .into_iter()
            .filter(|(_, (in_inventory, _))| in_inventory.owner == player_entity)
            .collect::<Vec<_>>();

        let count = player_inventory.len() as i32;

        let mut y = 25 - (count / 2);
        draw_box_bugfix(
            context,
            15,
            y - 2,
            31,
            count + 3,
            config.ui.fg,
            config.ui.bg,
        );
        context.print_color(
            18,
            y - 2,
            config.ui_title.fg,
            config.ui_title.bg,
            "Inventory",
        );
        context.print_color(
            18,
            y + count + 1,
            config.ui_title.fg,
            config.ui_title.bg,
            "ESCAPE to cancel",
        );

        let mut j = 0;
        for (_env, (_inv, name)) in player_inventory {
            context.set(17, y, config.ui.fg, config.ui.bg, to_cp437('('));
            context.set(
                18,
                y,
                config.ui_title.fg,
                config.ui_title.bg,
                97 + j as FontCharType,
            );
            context.set(19, y, config.ui.fg, config.ui.bg, to_cp437(')'));

            context.print(21, y, name.as_str());

            y += 1;
            j += 1;
        }
    }
}
