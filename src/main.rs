use crate::clock::Timer;
mod clock;

fn main() -> iced::Result {
    iced::application("pomodoro", Timer::update, Timer::view)
        .window(iced::window::Settings {
            size: (iced::Size::new(300.0, 400.0)),
            resizable: false,
            ..iced::window::Settings::default()
        })
        .theme(|_s| iced::Theme::GruvboxDark)
        .subscription(Timer::subscription)
        .run()
}
