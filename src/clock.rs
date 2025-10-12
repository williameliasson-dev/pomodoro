use std::time::Duration;

use iced::{
    Element,
    widget::{button, column, row, text},
};

pub struct Timer {
    pub time_remaining: Duration,
    pub is_running: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum TimerMessage {
    TickSecond,
    Start,
    Pause,
    Reset,
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            time_remaining: Duration::from_secs(60 * 25),
            is_running: false,
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
            }
        }
    }

    pub fn view(&self) -> Element<'_, TimerMessage> {
        let button_column = row![
            button("Start").on_press(TimerMessage::Start),
            button("Reset").on_press(TimerMessage::Reset),
            button("Pause").on_press(TimerMessage::Pause),
        ]
        .spacing(10);

        let total_seconds = self.time_remaining.as_secs();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;

        let time_display = text(format!("{:02}:{:02}", minutes, seconds))
            .size(50)
            .width(iced::Length::Fill)
            .center();

        let timer_view = column![time_display, button_column]
            .width(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .padding(20);

        timer_view.into()
    }
}
