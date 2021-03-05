use hecs::{ComponentError, World};

use crate::resource::WorldResources;

#[derive(Debug)]
pub struct GameLog {
    pub entries: Vec<String>,
}

impl GameLog {
    pub fn resource_push<S: Into<String>>(world: &World, msg: S) -> Result<(), ComponentError> {
        world.resource::<GameLog>()?.map(|mut gl| gl.push(msg))?;

        Ok(())
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
