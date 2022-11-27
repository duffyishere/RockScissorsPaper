use std::time::Instant;
use iced::{Application, application, Color, Command, Element, executor, Length, Point, Rectangle, Renderer, Settings, Size, Subscription, Vector, window};
use iced::time;
use iced::theme::{self, Theme};
use iced::widget::{canvas, Canvas, Container};
use iced::widget::canvas::{Cursor, Path};
use rand::Rng;

fn main() -> iced::Result {
    RockScissorsPaper::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct RockScissorsPaper {
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(std::time::Instant),
}

impl Application for RockScissorsPaper {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            RockScissorsPaper {
                state: State::new(),
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Rock Scissors Paper")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(instant) => {
                self.state.update(instant);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        canvas(&self.state)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn style(&self) -> theme::Application {
        fn dark_background(_theme: &Theme) -> application::Appearance {
            application::Appearance {
                background_color: Color::BLACK,
                text_color: Color::WHITE,
            }
        }

        theme::Application::from(dark_background as fn(&Theme) -> _)
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(time::Duration::from_millis(10)).map(Message::Tick)
    }
}

#[derive(Debug)]
struct State {
    space_cache: canvas::Cache,
    system_cache: canvas::Cache,
    start: Instant,
    now: Instant,
    stars: Vec<(Point, f32)>,
}

impl State {
    fn new() -> Self {
        let now = Instant::now();
        let (width, height) = window::Settings::default().size;

        Self {
            space_cache: Default::default(),
            system_cache: Default::default(),
            start: now,
            now,
            stars: Self::generate_stars(width, height),
        }
    }

    fn generate_stars(width: u32, height: u32) -> Vec<(Point, f32)> {
        use rand::Rng;

        let mut rng = rand::thread_rng();

        (0..100)
            .map(|_| {
                (
                    Point::new(
                        rng.gen_range(
                            (-(width as f32) / 2.0)..(width as f32 / 2.0),
                        ),
                        rng.gen_range(
                            (-(height as f32) / 2.0)..(height as f32 / 2.0),
                        ),
                    ),
                    rng.gen_range(0.5..1.0),
                )
            })
            .collect()
    }

    pub fn update(&mut self, now: Instant) {
        self.now = now;
        self.system_cache.clear();
    }
}

impl<Message> canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<canvas::Geometry> {

        let background = self.space_cache.draw(bounds.size(), |frame| {
        });

        let system = self.system_cache.draw(bounds.size(), |frame| {
            let center = frame.center();

            // TODO: Star 생성 후 각자 움직이게 하기
            let elapsed  = self.now - self.start;
            let rotation = (2.0 * 3.14 / 60.0) * elapsed.as_secs() as f32
                + (2.0 * 3.14 / 60_000.0) * elapsed.subsec_millis() as f32;

            for (point, size) in &self.stars {
                let mut star = Path::circle(*point, *size);

                frame.with_save(|frame| {
                    frame.translate(Vector::new(center.x, center.y));
                    frame.rotate(rotation);
                    // frame.translate(Vector::new(150 as f32, 0.0));

                    frame.fill(&mut star, Color::WHITE);
                })
            }
        });

        vec![background, system]
    }
}