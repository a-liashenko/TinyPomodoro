use chrono::{DateTime, Duration, Utc};

#[derive(PartialEq, Eq)]
enum Status {
    Active,
    Paused,
}

pub struct Timer {
    finish_at: DateTime<Utc>,
    paused_at: DateTime<Utc>,
    status: Status,
}

impl Timer {
    pub fn new_paused(duration: Duration) -> Self {
        let now = Utc::now();
        Self {
            finish_at: now + duration,
            paused_at: now,
            status: Status::Paused,
        }
    }

    pub fn time_left(&self) -> Duration {
        match self.status {
            Status::Active => {
                let now = Utc::now();
                if now > self.finish_at {
                    Duration::zero()
                } else {
                    self.finish_at - now
                }
            }
            Status::Paused => self.finish_at - self.paused_at,
        }
    }

    pub fn pause(&mut self) {
        if self.status == Status::Active {
            self.paused_at = Utc::now();
            self.status = Status::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.status == Status::Paused {
            let diff = self.finish_at - self.paused_at;
            self.finish_at = Utc::now() + diff;
            self.status = Status::Active;
        }
    }

    pub fn is_running(&self) -> bool {
        self.status == Status::Active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer() {
        let seconds = 80;
        let mut timer = Timer::new_paused(Duration::seconds(seconds));
        std::thread::sleep(Duration::milliseconds(200).to_std().unwrap());
        assert_eq!(timer.time_left().num_seconds(), 80);

        timer.resume();
        std::thread::sleep(Duration::milliseconds(200).to_std().unwrap());
        assert_eq!(timer.time_left().num_seconds(), 79);
    }
}
