use bracket_lib::prelude::*;

pub fn player() -> FontCharType {
    to_cp437('@')
}

pub fn floor() -> FontCharType {
    to_cp437('.')
}

pub fn wall() -> FontCharType {
    to_cp437('#')
}

pub fn monster_goblin() -> FontCharType {
    to_cp437('g')
}

pub fn monster_orc() -> FontCharType {
    to_cp437('o')
}
