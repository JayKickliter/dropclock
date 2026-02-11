#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

use std::time::Instant;

/// Runs a closure on drop, passing the [`Instant`] captured at creation.
pub struct DropClock<F: FnOnce(Instant)> {
    start: Instant,
    on_drop: Option<F>,
}

impl<F: FnOnce(Instant)> DropClock<F> {
    /// Captures [`Instant::now`] and stores `on_drop` for later.
    pub fn new(on_drop: F) -> DropClock<F> {
        let start = Instant::now();
        let on_drop = Some(on_drop);
        DropClock { start, on_drop }
    }

    /// Disarms the timer without calling the closure.
    pub fn cancel(mut self) {
        self.on_drop.take();
    }
}

impl<F: FnOnce(Instant)> Drop for DropClock<F> {
    fn drop(&mut self) {
        if let Some(on_drop) = self.on_drop.take() {
            (on_drop)(self.start);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DropClock;
    use std::time::Duration;

    #[test]
    fn elapsed_reflects_sleep() {
        let sleep_duration = Duration::from_millis(77);
        let mut elapsed = Duration::from_millis(0);
        {
            let _timer = DropClock::new(|start| {
                elapsed = dbg!(start.elapsed());
            });
            std::thread::sleep(sleep_duration);
        }
        assert!(elapsed >= sleep_duration);
    }

    #[test]
    fn canceled_timer_does_not_fire() {
        let mut fired = false;
        {
            let timer = DropClock::new(|_start| {
                fired = true;
            });
            timer.cancel();
        }
        assert!(!fired);
    }
}
