use crate::config::CONFIG;
use crate::data::{AppData, ScanStatus, ServerStatus};
use crate::{app, util, version};
use iced::widget::{Button, Column, Container, Row, Text, TextInput};
use iced::{
    subscription, theme, window, Application, Command, Element, Renderer, Settings, Subscription,
};
use iced_aw::menu::{MenuBar, MenuTree};
use iced_aw::{menu, Card, Modal};
use log::{info, trace};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start(data: Arc<Mutex<AppData>>) {
    info!("start gui");

    let _ = Gui::run(Settings {
        window: window::Settings {
            size: (680, 280),
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
    is_show_modal: bool,
}

#[derive(Debug, Clone)]
enum Message {
    StartServer,
    StopServer,
    Exit,
    OpenModal,
    CloseModal,
    Noop,
}

impl Application for Gui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::theme::Theme;
    type Flags = Arc<Mutex<AppData>>;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                data: flags,
                is_show_modal: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let mut t = format!("{} - v{}", app::APP_NAME, version::VERSION_TEXT);
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
            Message::Exit => {
                exit(0);
            }
            Message::CloseModal => {
                self.is_show_modal = false;
            }
            Message::OpenModal => {
                self.is_show_modal = true;
            }
            Message::Noop => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let m_btn_help = Button::new("帮助")
            .style(theme::Button::Secondary)
            .on_press(Message::Noop);
        let m_btn_about = Button::new("关于")
            .style(theme::Button::Secondary)
            .on_press(Message::OpenModal);
        let mt_help = MenuTree::new(m_btn_about);
        let mr_help = MenuTree::with_children(m_btn_help, vec![mt_help]);

        let m_btn_exit = Button::new("退出")
            .style(theme::Button::Secondary)
            .on_press(Message::Exit);
        let mt_exit = MenuTree::new(m_btn_exit);
        let m_btn_opt = Button::new("操作")
            .style(theme::Button::Secondary)
            .on_press(Message::Noop);
        let mr_opt = MenuTree::with_children(m_btn_opt, vec![mt_exit]);
        let mb = MenuBar::new(vec![mr_opt, mr_help])
            .padding(10)
            .spacing(10.0)
            .item_width(menu::ItemWidth::Static(50));

        let modal_about = Modal::new(self.is_show_modal, "", || {
            Card::new(
                Text::new("关于"),
                Text::new(format!(
                    "{}\n\nVersion {}\n\nCopyright © 2023 清新池塘",
                    app::APP_NAME,
                    version::VERSION_TEXT
                )),
            )
            .max_width(300.0)
            .into()
        })
        .backdrop(Message::CloseModal)
        .on_esc(Message::CloseModal);

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
        let last_scan_finish_time_text = Text::new(format_last_scan_finish_time(
            d_guard.server_file_info.last_scan_finish_time,
        ));

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

        let default_padding = 10;
        let default_spacing = 10;
        let status_container = Row::new()
            .padding(default_padding)
            .spacing(default_spacing)
            .push(light)
            .push(btn_start)
            .push(btn_stop);

        let scan_status_container = Row::new()
            .padding(default_padding)
            .spacing(default_spacing)
            .push(scan_light)
            .push(scan_text);

        let dir_container = Row::new().push(dir_label).push(dir_input);
        let port_container = Row::new().push(port_label).push(port_input);
        let config_container = Column::new()
            .padding(default_padding)
            .spacing(default_spacing)
            .push(dir_container)
            .push(port_container);

        let st_container = Column::new()
            .padding(default_padding)
            .push(last_scan_finish_time_text);

        let ann_card = Card::new("公告", CONFIG.announcement.as_str()).max_width(350.0);

        let opt_c = Column::new()
            .push(modal_about)
            .push(mb)
            .push(status_container)
            .push(scan_status_container)
            .push(st_container)
            .push(config_container);
        let ann_c = Column::new().padding(default_padding).push(ann_card);
        let mc = Row::new().push(opt_c).push(ann_c);

        let c = Container::new(mc);

        let content = c.into();
        return content;
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        trace!("s0");
        let es = vec![SubscribeEvent::RefreshUi, SubscribeEvent::RefreshUi];
        Subscription::batch(es.iter().map(SubscribeEvent::s))
    }
}

fn calc_dir_input_width() -> u16 {
    let min = 250;
    let max = 380;
    let mut width = (&CONFIG).dir.len() as u16 * 10;
    if width < min {
        width = min;
    } else if width > max {
        width = max;
    }
    width
}

fn format_last_scan_finish_time(timestamp: i64) -> String {
    if timestamp == 0 {
        return "等待刷新".to_string();
    }
    let dt = util::format_timestamp_to_datetime(timestamp);
    format!("上次刷新时间：{}", dt)
}

enum SubscribeEvent {
    RefreshUi,
}

impl SubscribeEvent {
    fn s(&self) -> Subscription<Message> {
        trace!("s1");
        subscription::unfold("1", "InitData".to_string(), |d| {
            trace!("s2,{:?}", d);

            async {
                thread::sleep(Duration::from_secs(1));
                (Some(Message::Noop), "NewData".to_string())
            }
        })
        .map(|_| Message::Noop)
    }
}
