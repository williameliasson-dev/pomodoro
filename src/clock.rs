use std::time::Duration;

use iced::{
    Element,
    widget::{button, column, row, text},
};
use notify_rust::Notification;

pub struct Timer {
    pub time_remaining: Duration,
    pub is_running: bool,
    pub cycle: PomodoroCycle,
    pub work_sessions_completed: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum TimerMessage {
    TickSecond,
    Start,
    Pause,
    Reset,
}

#[derive(Debug, Clone, Copy)]
pub enum PomodoroCycle {
    Work,
    ShortBreak,
    LongBreak,
}

fn get_cycle_duration(cycle: PomodoroCycle) -> Duration {
    match cycle {
        PomodoroCycle::Work => Duration::from_secs(25 * 60),
        PomodoroCycle::ShortBreak => Duration::from_secs(5 * 60),
        PomodoroCycle::LongBreak => Duration::from_secs(15 * 60),
    }
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            time_remaining: Duration::from_secs(get_cycle_duration(PomodoroCycle::Work).as_secs()),
            is_running: false,
            cycle: PomodoroCycle::Work,
            work_sessions_completed: 0,
        }
    }
}

impl Timer {
    pub fn subscription(&self) -> iced::Subscription<TimerMessage> {
        if self.is_running {
            iced::time::every(Duration::from_secs(1)).map(|_| TimerMessage::TickSecond)
        } else {
            iced::Subscription::none()
        }
    }

    pub fn update(&mut self, message: TimerMessage) {
        match message {
            TimerMessage::TickSecond => {
                if self.is_running && !self.time_remaining.is_zero() {
                    self.time_remaining = self.time_remaining - Duration::from_secs(1);
                }

                if self.is_running && self.time_remaining.is_zero() {
                    self.is_running = false;
                    self.switch_cycle();

                    Notification::new()
                        .summary("Pomodoro Timer")
                        .body(self.get_cycle_message())
                        .show()
                        .ok();
                }
            }
            TimerMessage::Start => {
                self.is_running = true;
            }
            TimerMessage::Pause => {
                self.is_running = false;
            }
            TimerMessage::Reset => {
                self.time_remaining = Duration::from_secs(60 * 25);
                self.is_running = false;
                self.cycle = PomodoroCycle::Work;
                self.work_sessions_completed = 0;
            }
        }
    }

    pub fn view(&self) -> Element<'_, TimerMessage> {
        let start_pause_button = match self.is_running {
            true => button(text("Pause").center())
                .on_press(TimerMessage::Pause)
                .width(100),
            false => button(text("Start").center())
                .on_press(TimerMessage::Start)
                .width(100),
        };

        let button_column = row![
            start_pause_button,
            button(text("Reset").center())
                .on_press(TimerMessage::Reset)
                .width(100)
        ]
        .spacing(10);

        let total_seconds = self.time_remaining.as_secs();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;

        let time_display = text(format!("{:02}:{:02}", minutes, seconds))
            .size(50)
            .width(iced::Length::Fill)
            .center();

        let cycle_display = text(self.get_cycle_message())
            .size(25)
            .width(iced::Length::Fill)
            .center();

        let timer_view = column![time_display, button_column, cycle_display]
            .width(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .padding(20);

        timer_view.into()
    }

    fn switch_cycle(&mut self) {
        match self.cycle {
            PomodoroCycle::Work => {
                self.work_sessions_completed += 1;

                if self.work_sessions_completed % 4 == 0 {
                    self.cycle = PomodoroCycle::LongBreak;
                } else {
                    self.cycle = PomodoroCycle::ShortBreak;
                }
            }
            PomodoroCycle::ShortBreak | PomodoroCycle::LongBreak => {
                self.cycle = PomodoroCycle::Work;
            }
        }

        self.time_remaining = get_cycle_duration(self.cycle);
    }

    fn get_cycle_message(&self) -> &'static str {
        match self.cycle {
            PomodoroCycle::Work => "Time to focus!",
            PomodoroCycle::ShortBreak => "Take a short break!",
            PomodoroCycle::LongBreak => "Take a long break!",
        }
    }
}
