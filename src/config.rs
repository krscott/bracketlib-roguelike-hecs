use bracket_lib::prelude::{to_cp437, FontCharType, RGB};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

use crate::components::Renderable;

pub fn default_user_config() -> UserConfig {
    // Colors: https://lospec.com/palette-list/vinik24

    UserConfig {
        default_fg: "#c5ccb8".into(),
        default_fog_fg: "#9a9a97".into(),
        default_bg: "#0c0c0c".into(),
        default_fog_bg: "#0c0c0c".into(),
        ui: None,
        ui_hp: Some(TextUserConfig {
            fg: Some("#be955c".into()),
            bg: None,
        }),
        ui_hp_bar: Some(TextUserConfig {
            fg: Some("#9a4f50".into()),
            bg: None,
        }),
        ui_tooltip: Some(TextUserConfig {
            fg: Some("#c5ccb8".into()),
            bg: Some("#433455".into()),
        }),
        player: TileUserConfig {
            glyph: '@',
            fg: Some("#c28d75".into()),
            fog_fg: None,
            bg: None,
            fog_bg: None,
        },
        wall: TileUserConfig {
            glyph: '#',
            fg: Some("#387080".into()),
            fog_fg: Some("#5d6872".into()),
            bg: None,
            fog_bg: None,
        },
        floor: TileUserConfig {
            glyph: '.',
            fg: Some("#be955c".into()),
            fog_fg: Some("#6f6776".into()),
            bg: None,
            fog_bg: None,
        },
        orc: TileUserConfig {
            glyph: 'o',
            fg: Some("#9a4f50".into()),
            fog_fg: None,
            bg: None,
            fog_bg: None,
        },
        goblin: TileUserConfig {
            glyph: 'g',
            fg: Some("#9a4f50".into()),
            fog_fg: None,
            bg: None,
            fog_bg: None,
        },
    }
}

#[derive(Error, Debug)]
pub enum ConfigParseError {
    #[error("Could not convert '{0}' to CP437")]
    UnrecognizedGlyph(char),

    #[error("Error parsing color code (expected format: \"#123abc\", got: \"{0}\")")]
    BadColorCode(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileUserConfig {
    pub glyph: char,
    pub fg: Option<String>,
    pub fog_fg: Option<String>,
    pub bg: Option<String>,
    pub fog_bg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextUserConfig {
    pub fg: Option<String>,
    pub bg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    pub default_fg: String,
    pub default_fog_fg: String,
    pub default_bg: String,
    pub default_fog_bg: String,

    pub ui: Option<TextUserConfig>,
    pub ui_hp: Option<TextUserConfig>,
    pub ui_hp_bar: Option<TextUserConfig>,
    pub ui_tooltip: Option<TextUserConfig>,

    pub player: TileUserConfig,
    pub wall: TileUserConfig,
    pub floor: TileUserConfig,
    pub orc: TileUserConfig,
    pub goblin: TileUserConfig,
}

#[derive(Debug, Clone)]
pub struct TileConfig {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub fog_fg: RGB,
    pub bg: RGB,
    pub fog_bg: RGB,
}

impl TileConfig {
    fn try_from_user_config(
        value: TileUserConfig,
        defaults: &TileConfig,
    ) -> Result<Self, ConfigParseError> {
        let TileUserConfig {
            glyph,
            fg,
            fog_fg,
            bg,
            fog_bg,
        } = value;

        Ok(TileConfig {
            glyph: parse_glyph(glyph)?,
            fg: parse_color_code_option(fg, defaults.fg)?,
            fog_fg: parse_color_code_option(fog_fg, defaults.fog_fg)?,
            bg: parse_color_code_option(bg, defaults.bg)?,
            fog_bg: parse_color_code_option(fog_bg, defaults.fog_bg)?,
        })
    }
}

impl From<TileConfig> for Renderable {
    fn from(value: TileConfig) -> Self {
        Renderable {
            glyph: value.glyph,
            fg: value.fg,
            bg: value.bg,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextConfig {
    pub fg: RGB,
    pub bg: RGB,
}

impl TextConfig {
    fn try_from_option_user_config(
        value: Option<TextUserConfig>,
        defaults: &TextConfig,
    ) -> Result<Self, ConfigParseError> {
        match value {
            Some(value) => {
                let TextUserConfig { fg, bg } = value;

                Ok(TextConfig {
                    fg: parse_color_code_option(fg, defaults.fg)?,
                    bg: parse_color_code_option(bg, defaults.bg)?,
                })
            }
            None => Ok(defaults.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub bg: RGB,
    pub ui: TextConfig,
    pub ui_hp: TextConfig,
    pub ui_hp_bar: TextConfig,
    pub ui_tooltip: TextConfig,
    pub player: TileConfig,
    pub wall: TileConfig,
    pub floor: TileConfig,
    pub orc: TileConfig,
    pub goblin: TileConfig,
}

impl TryFrom<UserConfig> for Config {
    type Error = ConfigParseError;

    fn try_from(value: UserConfig) -> Result<Self, Self::Error> {
        let UserConfig {
            default_fg,
            default_fog_fg,
            default_bg,
            default_fog_bg,
            ui,
            ui_hp,
            ui_hp_bar,
            ui_tooltip,
            player,
            wall,
            floor,
            orc,
            goblin,
        } = value;

        let tile_defaults = TileConfig {
            glyph: 0,
            fg: parse_color_code(default_fg)?,
            fog_fg: parse_color_code(default_fog_fg)?,
            bg: parse_color_code(default_bg)?,
            fog_bg: parse_color_code(default_fog_bg)?,
        };

        let text_defaults = TextConfig {
            fg: tile_defaults.fg,
            bg: tile_defaults.bg,
        };

        Ok(Config {
            bg: tile_defaults.bg,
            ui: TextConfig::try_from_option_user_config(ui, &text_defaults)?,
            ui_hp: TextConfig::try_from_option_user_config(ui_hp, &text_defaults)?,
            ui_hp_bar: TextConfig::try_from_option_user_config(ui_hp_bar, &text_defaults)?,
            ui_tooltip: TextConfig::try_from_option_user_config(ui_tooltip, &text_defaults)?,
            player: TileConfig::try_from_user_config(player, &tile_defaults)?,
            wall: TileConfig::try_from_user_config(wall, &tile_defaults)?,
            floor: TileConfig::try_from_user_config(floor, &tile_defaults)?,
            orc: TileConfig::try_from_user_config(orc, &tile_defaults)?,
            goblin: TileConfig::try_from_user_config(goblin, &tile_defaults)?,
        })
    }
}

fn parse_glyph(glyph: char) -> Result<FontCharType, ConfigParseError> {
    match to_cp437(glyph) {
        0 => Err(ConfigParseError::UnrecognizedGlyph(glyph)),
        x => Ok(x),
    }
}

fn parse_color_code(code: String) -> Result<RGB, ConfigParseError> {
    match RGB::from_hex(&code) {
        Ok(rgb) => Ok(rgb),
        Err(_) => Err(ConfigParseError::BadColorCode(code)),
    }
}

fn parse_color_code_option(
    code_option: Option<String>,
    default: RGB,
) -> Result<RGB, ConfigParseError> {
    match code_option {
        Some(code) => parse_color_code(code),
        None => Ok(default),
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn test_default_config() {
        let res: Result<Config, _> = default_user_config().try_into();
        assert!(res.is_ok());
    }
}
