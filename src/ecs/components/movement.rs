/// Movement component - tracks pending movement for an entity.
#[derive(Debug, Clone)]
pub struct Movement {
    /// Pending move direction (heading 0-7), or -1 if no move pending
    pub pending_heading: i32,
    /// Move speed in ticks (lower = faster)
    pub move_delay_ticks: i32,
    /// Ticks remaining until next move allowed
    pub cooldown_ticks: i32,
}

impl Movement {
    pub fn new() -> Self {
        Movement {
            pending_heading: -1,
            move_delay_ticks: 1,
            cooldown_ticks: 0,
        }
    }

    pub fn can_move(&self) -> bool {
        self.cooldown_ticks == 0
    }

    pub fn tick(&mut self) {
        if self.cooldown_ticks > 0 {
            self.cooldown_ticks -= 1;
        }
    }
}
