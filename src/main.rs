use iced::alignment;
use iced::executor;
use iced::theme::{self, Theme};
use iced::time;
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Application, Command, Element, Length, Settings, Subscription};
use notify_rust::Notification;

use iced::time::{Duration, Instant};

const MINUTE: u64 = 60;
const HOUR: u64 = 60 * MINUTE;

pub fn main() -> iced::Result {
    Stopwatch::run(Settings::default())
}

enum State {
    Idle,
    Ticking { last_tick: time::Instant },
}

// state of the app
struct Stopwatch {
    duration: time::Duration,
    state: State,
    notification_shown: bool,
}

#[derive(Debug, Copy, Clone)]
enum Message {
    Toggle,
    Reset,
    Tick(time::Instant),
}

impl Stopwatch {
    fn show_notification(&mut self) {
        if self.duration.as_secs() == MINUTE * 25 && self.notification_shown == false {
            // TODO: Fix the notification system.
            let handler = Notification::new()
                .summary("test summary")
                .body("test body")
                .show();
            match handler {
                Ok(_) => println!("Message sent"),
                Err(some_err) => println!("Error sending the message: {:?}", some_err)
            }
            self.notification_shown = true;
        }
    }
}

// logic
impl Application for Stopwatch {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                duration: Duration::default(),
                state: State::Idle,
                notification_shown: false,
            },
            Command::none(),
        )
    }
    //
    // react to the message -> change state (update logic)
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Toggle => match self.state {
                State::Idle => {
                    self.state = State::Ticking {
                        last_tick: Instant::now(),
                    };
                }
                State::Ticking { .. } => self.state = State::Idle,
            },
            Message::Tick(now) => {
                if let State::Ticking { last_tick } = &mut self.state {
                    self.duration += now - *last_tick;
                    *last_tick = now;
                }
                self.show_notification();
            }
            Message::Reset => self.duration = Duration::default(),
        }

        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let tick = match self.state {
            State::Idle => Subscription::none(),
            State::Ticking { .. } => time::every(Duration::from_millis(10)).map(Message::Tick),
        };
        Subscription::batch(vec![tick])
    }

    fn title(&self) -> String {
        String::from("Counter in Iced")
    }

    fn view(&self) -> Element<Message> {
        let seconds = self.duration.as_secs();

        let duration = text(format!(
            "{:0>2}:{:0>2}:{:0>2}.{:0>2}",
            seconds / HOUR,
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
            self.duration.subsec_millis() / 10
        ));

        let button = |label| {
            button(text(label).horizontal_alignment(alignment::Horizontal::Center))
                .padding(10)
                .width(80)
        };

        let toggle_button = {
            let label = match self.state {
                State::Idle => "Start",
                State::Ticking { .. } => "Stop",
            };

            button(label).on_press(Message::Toggle)
        };

        let reset_button = button("Reset")
            .style(theme::Button::Destructive)
            .on_press(Message::Reset);

        let controls = row![toggle_button, reset_button].spacing(20);

        let content = column![duration, controls]
            .align_items(Alignment::Center)
            .spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
