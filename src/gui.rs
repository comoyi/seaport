use crate::data::{AppData, ScanStatus, ServerFileInfo, ServerStatus};
use iced::widget::{Button, Column, Container, Row};
use iced::{theme, window, Application, Command, Element, Renderer, Settings};
use log::info;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start(data: Arc<Mutex<AppData>>) {
    info!("start gui");

    let _ = Gui::run(Settings {
        window: window::Settings {
            size: (300, 200),
            resizable: true,
            decorations: true,
            ..window::Settings::default()
        },
        flags: data,
        default_font: Some(include_bytes!("../fonts/HarmonyOS_Sans_SC_Regular.ttf")),
        ..Settings::default()
    });
}

struct Gui {
    data: Arc<Mutex<AppData>>,
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
    type Flags = Arc<Mutex<AppData>>;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self { data: flags }, Command::none())
    }

    fn title(&self) -> String {
        String::from("")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let mut d_guard = self.data.lock().unwrap();
        match message {
            Message::StartServer => {
                match d_guard.server_status {
                    ServerStatus::Started => {
                        return Command::none();
                    }
                    _ => {}
                }
                info!("start server...");
                d_guard.server_status = ServerStatus::Starting;
                thread::sleep(Duration::from_millis(100));
                d_guard.server_status = ServerStatus::Started;
            }
            Message::StopServer => {
                match d_guard.server_status {
                    ServerStatus::Stopped => {
                        return Command::none();
                    }
                    _ => {}
                }
                info!("stop server...");
                d_guard.server_status = ServerStatus::Stopping;
                thread::sleep(Duration::from_millis(100));
                d_guard.server_status = ServerStatus::Stopped;
            }
            Message::Noop => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let mut light = Button::new("    ").on_press(Message::Noop);
        let mut scan_light = Button::new("    ").on_press(Message::Noop);
        let mut btn_start = Button::new("启动").on_press(Message::StartServer);
        let mut btn_stop = Button::new("停止").on_press(Message::StopServer);

        let d_guard = self.data.lock().unwrap();

        match d_guard.server_status {
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

        match d_guard.server_file_info.scan_status {
            ScanStatus::Wait => {
                scan_light = scan_light.style(theme::Button::Secondary);
            }
            ScanStatus::Scanning => {
                scan_light = scan_light.style(theme::Button::Primary);
            }
            ScanStatus::Failed => {
                scan_light = scan_light.style(theme::Button::Destructive);
            }
            ScanStatus::Completed => {
                scan_light = scan_light.style(theme::Button::Positive);
            }
        }
        drop(d_guard);

        let mut status_container = Row::new();
        status_container = status_container.padding(10).spacing(10);
        status_container = status_container.push(light);
        status_container = status_container.push(btn_start);
        status_container = status_container.push(btn_stop);

        let mut scan_status_container = Row::new();
        scan_status_container = scan_status_container.padding(10).spacing(10);
        scan_status_container = scan_status_container.push(scan_light);

        let mut mc = Column::new();
        mc = mc.push(status_container);
        mc = mc.push(scan_status_container);

        let c = Container::new(mc);

        let content = c.into();
        return content;
    }
}
