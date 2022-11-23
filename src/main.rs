use iced::{Application, application, Color, Command, Element, executor, Length, Point, Rectangle, Renderer, Settings, Size, window};
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
    MouseMoved(Point),
    MousePressed(Point),
    MouseReleased(Point),
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

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
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
}

#[derive(Debug)]
struct State {
    space_cache: canvas::Cache,
    system_cache: canvas::Cache,
    stars: Vec<(Point, f32)>,
}

impl State {
    fn new() -> Self {
        let (width, height) = window::Settings::default().size;

        Self {
            space_cache: Default::default(),
            system_cache: Default::default(),
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
            let mut stars = Path::new(|path| {
                for (p, size) in &self.stars {
                    path.rectangle(*p, Size::new(*size, *size));
                }
            });

            frame.translate(frame.center() - Point::ORIGIN);
            frame.fill(&mut stars, Color::WHITE);
        });

        vec![background]
    }
}