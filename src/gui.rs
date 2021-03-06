use crate::{inventory::get_inventory_list, prelude::*};

pub const MAP_VIEW_WIDTH: usize = 80;
pub const MAP_VIEW_HEIGHT: usize = 43;

const TOOLTIP_HORIZONTAL_PADDING: i32 = 1;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected,
}

pub fn ui_input(context: &mut BTerm, world: &mut World) -> ItemMenuResult {
    match context.key {
        Some(VirtualKeyCode::Escape) => ItemMenuResult::Cancel,
        Some(key) => {
            if let Ok(player) = world.resource_entity::<Player>() {
                let inventory = get_inventory_list(world, player);
                let selection = letter_to_option(key);
                if let Some((item, _)) = inventory.get(selection as usize) {
                    world.spawn_command(UseItemCommand {
                        user: player,
                        item: *item,
                    });
                    ItemMenuResult::Selected
                } else {
                    ItemMenuResult::NoResponse
                }
            } else {
                ItemMenuResult::NoResponse
            }
        }
        None => ItemMenuResult::NoResponse,
    }
}

/// Convert index to a letter, starting with 'a' -> 0
/// ```
/// assert_eq!(index_to_letter(0), 'a' as FontCharType);
/// assert_eq!(index_to_letter(25), 'z' as FontCharType);
/// ```
fn index_to_letter(i: usize) -> FontCharType {
    // 0 -> 'a'
    97 + i as FontCharType
}

fn draw_fill_box(context: &mut BTerm, x: i32, y: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
    let blank = to_cp437(' ');
    for x in x..=(x + width) {
        for y in y..=(y + height) {
            context.set(x, y, fg, bg, blank);
        }
    }
}

/// Fix draw_box bug which fills box with #000000 instead of bg.
/// See https://github.com/amethyst/bracket-lib/issues/174
fn draw_box_bugfix(context: &mut BTerm, x: i32, y: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
    // context.draw_box(x, y, width, height, fg, bg);

    context.draw_hollow_box(x, y, width, height, fg, bg);
    draw_fill_box(context, x + 1, y + 1, width - 2, height - 2, fg, bg);
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

    for (_, map) in world.query::<&mut TileMap>().into_iter() {
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

struct MenuBoxStyle {
    pad: i32,
    fg: RGB,
    bg: RGB,
    highlight_fg: RGB,
    highlight_bg: RGB,
}

fn draw_menu_box(
    context: &mut BTerm,
    style: &MenuBoxStyle,
    title: &str,
    footer: &str,
    x: i32,
    y: i32,
    min_width: i32,
    min_height: i32,
) {
    const TITLE_OFFSET_X: i32 = 3;

    let width = *[
        min_width,
        title.len() as i32 + TITLE_OFFSET_X * 2,
        footer.len() as i32 + TITLE_OFFSET_X * 2,
    ]
    .iter()
    .max()
    .unwrap();

    let height = i32::max(2, min_height);

    // Menu box
    draw_box_bugfix(context, x, y, width, height, style.fg, style.bg);

    // Title
    context.print_color(
        x + TITLE_OFFSET_X,
        y,
        style.highlight_fg,
        style.highlight_bg,
        title,
    );

    // Footer
    context.print_color(
        x + TITLE_OFFSET_X,
        y + height,
        style.highlight_fg,
        style.highlight_bg,
        footer,
    );
}

fn draw_select_menu<S: AsRef<str>>(
    context: &mut BTerm,
    style: &MenuBoxStyle,
    title: &str,
    footer: &str,
    x: i32,
    y: i32,
    options: &[S],
) {
    let inner_x = x + style.pad;
    let inner_y = y + style.pad;
    let inner_height = options.len() as i32;
    let inner_width = 4 + options
        .iter()
        .map(|s| s.as_ref().len() as i32)
        .max()
        .unwrap_or(0);

    draw_menu_box(
        context,
        style,
        title,
        footer,
        x,
        y,
        inner_width + style.pad * 2 - 1,
        inner_height + style.pad * 2 - 1,
    );

    for (i, s) in options.iter().enumerate() {
        let item_y = inner_y + i as i32;

        context.set(inner_x, item_y, style.fg, style.bg, to_cp437('('));
        context.set(
            inner_x + 1,
            item_y,
            style.highlight_fg,
            style.highlight_bg,
            index_to_letter(i),
        );
        context.set(inner_x + 2, item_y, style.fg, style.bg, to_cp437(')'));

        context.print(inner_x + 4, item_y, s.as_ref());
    }
}

pub fn draw_inventory(context: &mut BTerm, world: &World, config: &Config) {
    if let Ok(player) = world.resource_entity::<Player>() {
        let menu_options = get_inventory_list(world, player)
            .into_iter()
            .map(|(_, name)| name)
            .collect::<Vec<_>>();

        draw_select_menu(
            context,
            &MenuBoxStyle {
                pad: 2,
                fg: config.ui.fg,
                bg: config.ui.bg,
                highlight_fg: config.ui_title.fg,
                highlight_bg: config.ui_title.bg,
            },
            "Inventory",
            "ESCAPE to cancel",
            15,
            25 - menu_options.len() as i32 / 2,
            &menu_options,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_to_letter() {
        assert_eq!(index_to_letter(0), 'a' as FontCharType);
        assert_eq!(index_to_letter(25), 'z' as FontCharType);
    }
}
