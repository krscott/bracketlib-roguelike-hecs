use bracket_lib::prelude::*;

pub fn bg() -> RGB {
    c("#1f240a")
}

pub fn player_fg() -> RGB {
    c("#efd8a1")
}

pub fn wall_fg() -> RGB {
    c("#276468")
}

pub fn floor_fg() -> RGB {
    c("#a58c27")
}

fn c(code: &str) -> RGB {
    RGB::from_hex(code).unwrap()
}
