use iced::alignment;
use iced::executor;
use iced::theme::{self, Theme};
use iced::time;
use iced::time::{Duration, Instant};
use iced::widget::Svg;
use iced::widget::{button, column, container, row, svg, text, Row};
use iced::Renderer;
use iced::{Alignment, Application, Command, Element, Length, Settings, Subscription};

use notify_rust::Notification;

const MILLISECOND: u64 = 1000;
const MINUTE: u64 = 60;
const HOUR: u64 = 60 * MINUTE;
const POMODORO_CYCLE_MILLIS: u128 = (MINUTE * 1 * MILLISECOND) as u128;

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
    tomatoes: u16,
}

#[derive(Debug, Copy, Clone)]
enum Message {
    Toggle,
    Reset,
    Tick(time::Instant),
}

impl Stopwatch {
    fn show_notification(&self) {
        if let Err(some_err) = Notification::new()
            .summary("Tomatentimer")
            .body("\nTomaten für Sie\nYou've reached a milestone 🐈!\n")
            .show()
        {
            eprintln!("Error while sending the message:\n{:?}", some_err);
        }
    }
    fn set_tomato_on_milestone(&mut self, show_notificacion: bool) {
        if self.duration.as_secs() != 0 && self.duration.as_millis() % POMODORO_CYCLE_MILLIS == 0 {
            self.tomatoes += 1;
            if show_notificacion {
                self.show_notification();
            }
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
                tomatoes: 0,
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
                self.set_tomato_on_milestone(true);
            }
            Message::Reset => {
                self.duration = Duration::default();
            }
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
        String::from("Tomatentimer in Iced")
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

        let tomatos = text(format!(
            "{} Tomato{}",
            self.tomatoes,
            if self.tomatoes == 1 { "" } else { "s" }
        ));

        let handle = svg::Handle::from_path(format!(
            "{}/resources/rottentomatoes.svg",
            env!("CARGO_MANIFEST_DIR")
        ));

        // let mut svg_tomatos = Vec::new();

        let mut svg_tomatos = Vec::new();

        for _ in 0..=self.tomatoes {
            let handle = handle.clone();
            svg_tomatos.push(
                svg(handle)
                    .width(25)
                    .height(25)
                    .style(theme::Svg::default()),
            );
        }

        let el: Vec<Element<'_, Message, Renderer>> = svg_tomatos
            .iter()
            .flatmap(|v| {
                let mut t: Vec<Element<'_, Message, Renderer>> = Vec::new();
                t.push(Text::new(format!("{}", v.name)).into());
                t.push(Text::new(format!("{:?}", v.state)).into());
                t
            })
            .collect();
        let r: Row<Message, Renderer> = Row::with_children(el);
        r.into();

        let content = column![duration, controls, tomatos]
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
