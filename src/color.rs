use bracket_lib::prelude::*;

pub fn bg() -> RGB {
    hex("#1f240a")
}

pub fn player_fg() -> RGB {
    hex("#efd8a1")
}

pub fn wall_fg() -> RGB {
    hex("#276468")
}

pub fn floor_fg() -> RGB {
    hex("#a58c27")
}

fn hex(code: &str) -> RGB {
    RGB::from_hex(code).unwrap()
}
