use hecs::World;

#[derive(Debug)]
pub struct GameLog {
    pub entries: Vec<String>,
}

impl GameLog {
    pub fn push_world<S: Into<String>>(world: &World, msg: S) {
        let msg: String = msg.into();

        for (_, log) in world.query::<&mut GameLog>().into_iter() {
            log.push(msg.clone());
        }
    }

    pub fn new() -> Self {
        GameLog {
            entries: Vec::new(),
        }
    }

    pub fn push<S: Into<String>>(&mut self, msg: S) {
        self.entries.push(msg.into());
    }
}
