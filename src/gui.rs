use crate::config::CONFIG;
use crate::data::{AppData, ScanStatus, ServerStatus};
use crate::version;
use iced::widget::{Button, Column, Container, Row, Text, TextInput};
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
        let mut t = format!("App - v{}", version::VERSION_TEXT);
        let ct = &CONFIG.title;
        if ct.len() > 0 {
            t = format!("{}  {}", t, ct);
        }
        t
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
        let mut btn_start = Button::new("启动").on_press(Message::StartServer);
        let mut btn_stop = Button::new("停止").on_press(Message::StopServer);
        let mut scan_light = Button::new("    ").on_press(Message::Noop);
        let scan_text;
        let label_width = 60;
        let dir_label = Text::new("文件夹").width(label_width);
        let dir_input: TextInput<Message> = TextInput::new("", &CONFIG.dir, |_s| -> Message {
            return Message::Noop;
        })
        .width(calc_dir_input_width());
        let port_label = Text::new("端口").width(label_width);
        let port_input: TextInput<Message> =
            TextInput::new("", CONFIG.port.to_string().as_str(), |_s| -> Message {
                return Message::Noop;
            })
            .width(70);

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
                scan_text = Text::new("");
            }
            ScanStatus::Scanning => {
                scan_light = scan_light.style(theme::Button::Primary);
                scan_text = Text::new("刷新文件列表中");
            }
            ScanStatus::Failed => {
                scan_light = scan_light.style(theme::Button::Destructive);
                scan_text = Text::new("刷新文件列表表失败");
            }
            ScanStatus::Completed => {
                scan_light = scan_light.style(theme::Button::Positive);
                scan_text = Text::new("刷新文件列表成功");
            }
        }
        drop(d_guard);

        let status_container = Row::new()
            .padding(10)
            .spacing(10)
            .push(light)
            .push(btn_start)
            .push(btn_stop);

        let scan_status_container = Row::new()
            .padding(10)
            .spacing(10)
            .push(scan_light)
            .push(scan_text);

        let dir_container = Row::new().push(dir_label).push(dir_input);
        let port_container = Row::new().push(port_label).push(port_input);
        let config_container = Column::new()
            .padding(10)
            .spacing(10)
            .push(dir_container)
            .push(port_container);

        let mc = Column::new()
            .push(status_container)
            .push(scan_status_container)
            .push(config_container);

        let c = Container::new(mc);

        let content = c.into();
        return content;
    }
}

fn calc_dir_input_width() -> u16 {
    let min = 200;
    let max = 500;
    let mut width = (&CONFIG).dir.len() as u16 * 10;
    if width < min {
        width = min;
    } else if width > max {
        width = max;
    }
    width
}
