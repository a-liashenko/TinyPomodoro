use self::timer::Timer;
use chrono::Duration;
use serde::{Deserialize, Serialize};

mod timer;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    Short,
    Long,
    Focus,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PomodoroCfg {
    #[serde(with = "crate::serde_helpers::duration")]
    pub short: Duration,
    #[serde(with = "crate::serde_helpers::duration")]
    pub long: Duration,
    #[serde(with = "crate::serde_helpers::duration")]
    pub focus: Duration,
    pub rounds: u16,
}

impl PomodoroCfg {
    pub fn get_duration(&self, status: &Status) -> Duration {
        match status {
            Status::Focus => self.focus,
            Status::Long => self.long,
            Status::Short => self.short,
        }
    }
}

pub struct Pomodoro {
    timer: Timer,
    config: PomodoroCfg,
    status: Status,
    current_focus: u16,
}

impl Pomodoro {
    pub fn new(config: PomodoroCfg) -> Self {
        let timer = Timer::new_paused(config.focus);
        Self {
            timer,
            config,
            status: Status::Focus,
            current_focus: 1,
        }
    }

    pub fn update_config(&mut self, new: PomodoroCfg) {
        if new.rounds < self.config.rounds {
            self.current_focus = 1;
        }
        self.config = new;
        self.reset();
    }

    pub fn pause(&mut self) {
        self.timer.pause()
    }

    pub fn resume(&mut self) {
        self.timer.resume()
    }

    pub fn reset(&mut self) {
        let duration = self.config.get_duration(&self.status);
        self.timer = Timer::new_paused(duration);
    }

    pub fn toggle(&mut self) {
        if self.is_running() {
            self.pause();
        } else {
            self.resume();
        }
    }

    pub fn next(&mut self) -> Status {
        let status = match self.status {
            Status::Long => {
                self.current_focus = 1;
                Status::Focus
            }
            Status::Short => {
                self.current_focus += 1;
                Status::Focus
            }
            Status::Focus => {
                if self.current_focus >= self.config.rounds {
                    Status::Long
                } else {
                    Status::Short
                }
            }
        };

        let duration = self.config.get_duration(&status);
        self.timer = Timer::new_paused(duration);
        self.status = status;

        status
    }

    pub fn try_next(&mut self) -> Option<Status> {
        if self.timer.time_left() == Duration::zero() {
            self.next();
            Some(self.status)
        } else {
            None
        }
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn config(&self) -> &PomodoroCfg {
        &self.config
    }

    pub fn time_left(&self) -> Duration {
        self.timer.time_left()
    }

    pub fn is_running(&self) -> bool {
        self.timer.is_running()
    }

    pub fn current_focus(&self) -> u16 {
        self.current_focus
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pomodoro() {
        let cfg = PomodoroCfg {
            short: Duration::milliseconds(100),
            long: Duration::milliseconds(200),
            focus: Duration::milliseconds(500),
            rounds: 2,
        };

        let mut pomodoro = Pomodoro::new(cfg);
        pomodoro.resume();
        assert_eq!(pomodoro.status(), Status::Focus);
        std::thread::sleep(cfg.focus.to_std().unwrap());

        pomodoro.try_next();
        pomodoro.resume();
        assert_eq!(pomodoro.status(), Status::Short);
        std::thread::sleep(cfg.short.to_std().unwrap());

        pomodoro.try_next();
        pomodoro.resume();
        assert_eq!(pomodoro.status(), Status::Focus);
        std::thread::sleep(cfg.focus.to_std().unwrap());

        pomodoro.try_next();
        pomodoro.resume();
        assert_eq!(pomodoro.status(), Status::Long);
        std::thread::sleep(cfg.long.to_std().unwrap());

        pomodoro.try_next();
        pomodoro.resume();
        assert_eq!(pomodoro.status(), Status::Focus);
        std::thread::sleep(cfg.focus.to_std().unwrap());
    }
}
