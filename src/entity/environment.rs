pub struct EnvironmentState {
    pub dirs: super::movement::Direction
}

impl EnvironmentState {
    pub fn new() -> Self {
        Self {
            dirs: Default::default()
        }
    }
}