use bracket_lib::prelude::*;

// https://lospec.com/palette-list/fantasy-24

pub fn bg() -> RGB {
    hex("#1f240a")
}

pub fn player_fg() -> RGB {
    hex("#efd8a1")
}

pub fn wall_fg() -> RGB {
    hex("#3c9f9c")
}

pub fn wall_fog_fg() -> RGB {
    hex("#39571c")
}

pub fn floor_fg() -> RGB {
    hex("#efd8a1")
}

pub fn floor_fog_fg() -> RGB {
    hex("#927e6a")
}

fn hex(code: &str) -> RGB {
    RGB::from_hex(code).unwrap()
}
