use crate::data::ServerStatus;
use iced::widget::{Button, Column, Container, Row};
use iced::{theme, window, Application, Command, Element, Renderer, Settings};
use log::info;
use std::thread;
use std::time::Duration;

pub fn start() {
    info!("gui start");

    let _ = Gui::run(Settings {
        window: window::Settings {
            size: (300, 200),
            resizable: true,
            decorations: true,
            ..window::Settings::default()
        },
        default_font: Some(include_bytes!("../fonts/HarmonyOS_Sans_SC_Regular.ttf")),
        ..Settings::default()
    });
}

struct Gui {
    server_status: ServerStatus,
}

#[derive(Debug, Clone)]
enum Message {
    StartServer,
    StopServer,
    Noop,
}

impl Application for Gui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::theme::Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                server_status: ServerStatus::Stopped,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::StartServer => {
                match self.server_status {
                    ServerStatus::Started => {
                        return Command::none();
                    }
                    _ => {}
                }
                info!("start server...");
                self.server_status = ServerStatus::Starting;
                thread::sleep(Duration::from_secs(1));
                self.server_status = ServerStatus::Started;
            }
            Message::StopServer => {
                match self.server_status {
                    ServerStatus::Stopped => {
                        return Command::none();
                    }
                    _ => {}
                }
                info!("stop server...");
                self.server_status = ServerStatus::Stopping;
                thread::sleep(Duration::from_secs(1));
                self.server_status = ServerStatus::Stopped;
            }
            Message::Noop => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let mut light = Button::new("    ").on_press(Message::Noop);
        let mut btn_start = Button::new("启动").on_press(Message::StartServer);
        let mut btn_stop = Button::new("停止").on_press(Message::StopServer);
        match self.server_status {
            ServerStatus::Starting => {
                btn_start = btn_start.style(theme::Button::Secondary);
                btn_stop = btn_stop.style(theme::Button::Primary);
                light = light.style(theme::Button::Secondary);
            }
            ServerStatus::Started => {
                btn_start = btn_start.style(theme::Button::Secondary);
                btn_stop = btn_stop.style(theme::Button::Primary);
                light = light.style(theme::Button::Positive);
            }
            ServerStatus::Stopping => {
                btn_start = btn_start.style(theme::Button::Secondary);
                btn_stop = btn_stop.style(theme::Button::Secondary);
                light = light.style(theme::Button::Secondary);
            }
            ServerStatus::Stopped => {
                btn_start = btn_start.style(theme::Button::Primary);
                btn_stop = btn_stop.style(theme::Button::Secondary);
                light = light.style(theme::Button::Destructive);
            }
        }

        let mut status_container = Row::new();
        status_container = status_container.padding(10).spacing(10);
        status_container = status_container.push(light);
        status_container = status_container.push(btn_start);
        status_container = status_container.push(btn_stop);

        let mut mc = Column::new();
        mc = mc.push(status_container);

        let c = Container::new(mc);

        let content = c.into();
        return content;
    }
}
