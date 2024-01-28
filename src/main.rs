use iced::widget::{button, column, text};
use iced::{executor, Alignment, Application, Command, Element, Sandbox, Settings, Theme};

use std::time::{self, Duration, Instant};

pub fn main() -> iced::Result {
    Stopwatch::run(Settings::default())
}

enum State {
    Idle,
    Ticking { last_tick: time::Instant }
}

// state of the app
struct Stopwatch {
    duration: time::Duration,
    state: State,
}

#[derive(Debug, Copy, Clone)]
enum Message {
    Toggle,
    Reset,
    Tick(time::Instant)
}

// logic
impl Application for Stopwatch {

    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Executor;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                duration: Duration::default(),
                state: State::Idle,
            },
            Command::none()
        )
    }
    //
    // react to the message -> change state (update logic)
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Toggle => match self.state {
                State::Idle => {
                    self.state = State::Ticking { last_tick: Instant::now() };
                }
                State::Ticking { .. } => self.state = State::Idle
            },
            Message::Tick(now) => {
                if let State::Ticking { last_tick } = &mut self.state {
                    self.duration += now - *last_tick;
                    *last_tick = now;
                }
            },
            Message::Reset => {
                self.duration = Duration::default();
            }
        }

        Command::none()
    }

    fn title(&self) -> String {
        String::from("Counter in Iced")
    }

    fn view(&self) -> Element<Message> {
        column![
            button("+").on_press(Message::IncrementPressed),    // incr. button -> sends message
            text(self.value).size(50),                          // Show value
            button("-").on_press(Message::DecrementPressed)     // decr. button -> sends message
        ].padding(20).align_items(Alignment::Center).into()
    }


}
