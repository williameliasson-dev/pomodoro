use crate::clock::Timer;
mod clock;

fn main() -> iced::Result {
    iced::application("test", Timer::update, Timer::view)
        .theme(|_s| iced::Theme::TokyoNight)
        .subscription(Timer::subscription)
        .run()
}
