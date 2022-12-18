pub struct InningTracker {
    inning: u8,
    outs: u8,
    top: bool,
}

impl InningTracker {
    pub fn new() -> Self {
        InningTracker {
            inning: 1,
            outs: 0,
            top: true,
        }
    }

    pub fn end_inning_half(&mut self) {
        if self.top {
            self.top = !self.top;
            self.outs = 0;
        } else {
            self.top = !self.top;
            self.inning += 1;
            self.outs = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let inning_tracker = InningTracker::new();

        assert_eq!(
            inning_tracker.outs, 0,
            "New inning tracker doesen't start with 0 outs!"
        );
        assert_eq!(
            inning_tracker.inning, 1,
            "New inning tracker doesen't start with inning 1!"
        );
        assert_eq!(
            inning_tracker.top, true,
            "New inning tracker doesent start on top!"
        );
    }

    #[test]
    fn end_inning_top_half() {
        let mut inning_tracker = InningTracker::new();
        inning_tracker.end_inning_half();

        assert_eq!(
            inning_tracker.inning, 1,
            "InningTracker should be in first inning, after top half ends!"
        );
        assert_eq!(
            inning_tracker.outs, 0,
            "InningTracker should have 0 outs after inning half ends!"
        );
        assert_eq!(
            inning_tracker.top, false,
            "InningTracker should be in bottom half after top half ends!"
        );
    }

    #[test]
    fn end_inning_bottom_half() {
        let mut inning_tracker = InningTracker::new();
        inning_tracker.end_inning_half();
        inning_tracker.end_inning_half();

        assert_eq!(
            inning_tracker.inning, 2,
            "InningTracker should be in second inning, after bottom half ends!"
        );
        assert_eq!(
            inning_tracker.outs, 0,
            "InningTracker should have 0 outs after inning half ends!"
        );
        assert_eq!(
            inning_tracker.top, true,
            "InningTracker should be in top half after bottom half ends!"
        );
    }
}
