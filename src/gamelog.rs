#[derive(Debug)]
pub struct GameLog {
    pub entries: Vec<String>,
}

impl GameLog {
    pub fn new() -> Self {
        GameLog {
            entries: Vec::new(),
        }
    }

    pub fn push<S: Into<String>>(&mut self, msg: S) {
        self.entries.push(msg.into());
    }
}
