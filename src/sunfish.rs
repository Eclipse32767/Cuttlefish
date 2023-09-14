use iced::theme::Theme;
use iced::{Result, Settings, alignment, Alignment, Length, Application, Command, executor};
use iced::widget::{Button, Row, Column, Container, Text, Scrollable};
use iced::Color;
use lib_cfg::{get_cfg_data, decode_theme, OurTheme};
mod lib_cfg;
use lib_style::{ButtonStyle, ThemeCustom, make_custom_theme, ThemeSet, ListStyle, MenuStyle};
mod lib_style;
use gettextrs::*;
use gettextrs::gettext as tr;

fn main() -> Result {
    let _ = textdomain("SunfishMan");
    let _ = bind_textdomain_codeset("SunfishMan", "UTF-8");
    Manual::run(Settings::default())
}

struct Manual {
    theme:OurTheme,
    current_page:u8,
    primary_key: String,
    secondary_key: String,
    exit_header: String,
    exit_key: String,
    launch_header: String,
    launch_key: String,
    kill_header: String,
    kill_key: String,
    minimize_header: String,
    minimize_key: String,
    scratch_header: String,
    scratch_key: String,
    theme_set: ThemeSet,
}
pub fn pretty_pri(x: &str) -> &'static str {
    match x {
        "super" => "Command/Win",
        "alt" => "Alt",
        "control" => "Control",
        "shift" => "Shift",
        &_ => "Error"
    }
}
pub fn pretty_header(x: &str, pri: &str, sec: &str) -> String {
    let primary = pri.to_string();
    let secondary = sec.to_string();
    match x {
        "pri" => primary,
        "sec" => secondary,
        "both" => format!("{primary}+{secondary}"),
        &_ => "Error".to_string()
    }
}

impl Default for Manual {
    fn default() -> Self {
        let data = get_cfg_data();
        let pri = pretty_pri(&data.primary);
        let sec = pretty_pri(&data.secondary);
        Manual {
            theme: decode_theme(&data.theme, OurTheme::Light),
            current_page: 0,
            primary_key: pri.to_string(),
            secondary_key: sec.to_string(),
            exit_header: pretty_header(&data.exit_h, pri, sec).to_string(),
            exit_key: data.exit_k,
            launch_header: pretty_header(&data.launch_h, pri, sec).to_string(),
            launch_key: data.launch_k,
            kill_header: pretty_header(&data.kill_h, pri, sec).to_string(),
            kill_key: data.kill_k,
            minimize_header: pretty_header(&data.mini_h, pri, sec).to_string(),
            minimize_key: data.mini_k,
            scratch_header: pretty_header(&data.scratch_h, pri, sec).to_string(),
            scratch_key: data.scratch_k,
            theme_set: ThemeSet {
                light: ThemeCustom {
                    application: iced::theme::Palette {
                        background: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        text: Color::from_rgb8(0x00, 0x19, 0x36),
                        primary: Color::from_rgb8(0x00, 0xF1, 0xD6),
                        success: Color::from_rgb8(0xFF, 0x4C, 0x00),
                        danger: Color::from_rgb8(0xFF, 0x4C, 0x00),
                    },
                    sidebar: ButtonStyle { 
                        border_radius: 2.0,
                        txt_color: Color::from_rgb8( 0x00, 0x19, 0x36),
                        bg_color: Color::from_rgb8(0xD2, 0xF0, 0xFF),
                        border_color: Color::from_rgb8(0, 0, 0),
                        border_width: 0.0,
                        shadow_offset: iced::Vector {x: 0.0, y: 0.0}
                    },
                    secondary: ButtonStyle {
                        border_radius: 2.0,
                        txt_color: Color::from_rgb8(0x00, 0x20, 0x46),
                        bg_color: Color::from_rgb8(0xC6, 0xEC, 0xFF),
                        border_color: Color::from_rgb8(0, 0, 0),
                        border_width: 0.0,
                        shadow_offset: iced::Vector {x: 0.0, y: 0.0}
                    },
                    list: ListStyle {
                        txt_color: Color::from_rgb8( 0x00, 0x19, 0x36),
                        bg_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        handle_color: Color::from_rgb8( 0x00, 0x19, 0x36),
                        border_radius: 10.0,
                        border_width: 5.0,
                        border_color: Color::from_rgb8( 0x00, 0x19, 0x36),
                        menu: MenuStyle {
                            txt_color: Color::from_rgb8( 0x00, 0x19, 0x36),
                            bg_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                            border_radius: 10.0,
                            border_width: 5.0,
                            border_color: Color::from_rgb8( 0x00, 0x19, 0x36),
                            sel_txt_color: Color::from_rgb8( 0x00, 0x19, 0x36),
                            sel_bg_color: Color::from_rgb8(0x00, 0xF1, 0xD6),
                        }
                    }
                },
                dark: ThemeCustom { // TODO: set dark theme properly
                    application: iced::theme::Palette {
                        background: Color::from_rgb8(0x00, 0x19, 0x36),
                        text: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        primary: Color::from_rgb8(0x00, 0xCD, 0xB6),
                        success: Color::from_rgb8(1, 1, 1),
                        danger: Color::from_rgb8(0xC5, 0x3A, 0x00),
                    },
                    sidebar: ButtonStyle { 
                        border_radius: 2.0,
                        txt_color: Color::from_rgb8( 0xE0, 0xF5, 0xFF),
                        bg_color: Color::from_rgb8(0x00, 0x20, 0x46),
                        border_color: Color::from_rgb8(0, 0, 0),
                        border_width: 0.0,
                        shadow_offset: iced::Vector {x: 0.0, y: 0.0}
                    },
                    secondary: ButtonStyle {
                        border_radius: 2.0,
                        txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        bg_color: Color::from_rgb8(0x00, 0x29, 0x58),
                        border_color: Color::from_rgb8(0, 0, 0),
                        border_width: 0.0,
                        shadow_offset: iced::Vector {x: 0.0, y: 0.0}
                    },
                    list: ListStyle {
                        txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        bg_color: Color::from_rgb8(0x00, 0x29, 0x58),
                        handle_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        border_radius: 5.0,
                        border_width: 2.0,
                        border_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        menu: MenuStyle {
                            txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                            bg_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                            border_radius: 5.0,
                            border_width: 2.0,
                            border_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                            sel_txt_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                            sel_bg_color: Color::from_rgb8(0x00, 0xCD, 0xB6),
                        }
                    }
                },
                custom: make_custom_theme()
            },
        }
    }
}
#[derive(Debug, Clone)]
enum Message {
    PageIncr,
    PageDecr,
    KeyboardUpdate(iced::keyboard::Event)
}
impl Application for Manual {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::default(),
            Command::none()
        )
    }
    fn title(&self) -> String {
        format!("Sway Manual")
    }
    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::PageIncr => {
                if self.current_page < 3 {self.current_page = self.current_page + 1;}
                Command::none()
            }
            Message::PageDecr => {
                if self.current_page > 0 {self.current_page = self.current_page -1;}
                Command::none()
            }
            Message::KeyboardUpdate(x) => {
                match x {
                    iced::keyboard::Event::KeyPressed { key_code, modifiers: _ } => {
                        if key_code == iced::keyboard::KeyCode::Right {
                            if self.current_page < 3 {self.current_page = self.current_page + 1;}
                        } else if key_code == iced::keyboard::KeyCode::Left {
                            if self.current_page > 0 {self.current_page = self.current_page -1;}
                        }
                    }
                    iced::keyboard::Event::KeyReleased {..} => {

                    }
                    iced::keyboard::Event::CharacterReceived(..) => {

                    }
                    iced::keyboard::Event::ModifiersChanged(..) => {

                    }
                }
                Command::none()
            }
        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        let style = match self.theme {
            OurTheme::Light => self.theme_set.light.clone(),
            OurTheme::Dark => self.theme_set.dark.clone(),
            OurTheme::Custom => self.theme_set.custom.clone(),
        };
        
        let back_txt = Text::new(tr("Back"));
        let forward_txt = Text::new(tr("Forwards"));
        let mut page_left = Button::new(back_txt)
            .on_press(Message::PageDecr);
        let mut page_right = Button::new(forward_txt)
            .on_press(Message::PageIncr);


        let mut settings = Column::new().spacing(10);
        let human_pg = self.current_page+1;
        let pg_num = Text::new(format!("{human_pg}"));
        let mut pg_title = Text::new("Page Title").horizontal_alignment(alignment::Horizontal::Center);
        if self.current_page == 0 {
            page_left = page_left.style(style.secondary.mk_theme());
            let title = tr("Basic Navigation");
            pg_title = Text::new(format!("{title}"));
            let primary_key = self.primary_key.clone();
            let secondary_key = self.secondary_key.clone();
            let pre_focus = tr("To shift focus between applications, press:\n");
            let focus = tr("+An Arrow Key.\n");
            let post_focus = tr("This will shift the interface's focus in the direction you pressed.\n \n");
            let pre_move = tr("To move applications around, press:\n");
            let move_str = tr("+An Arrow Key.\n");
            let post_move = tr("This should swap applications in that direction.\n\n");
            let immutable = tr("As of now, these bindings are inferred and cannot be directly changed.");
            let text = Text::new(format!("{pre_focus}{primary_key}{focus}{post_focus}{pre_move}{primary_key}+{secondary_key}{move_str}{post_move}{immutable}")).horizontal_alignment(alignment::Horizontal::Center);
            settings = settings.push(text);
        } else if self.current_page == 1 {
            let title = tr("Basic Navigation, Continued");
            pg_title = Text::new(format!("{title}"));
            let launch_h = self.launch_header.clone();
            let launch_k = self.launch_key.clone();
            let kill_h = self.kill_header.clone();
            let kill_k = self.kill_key.clone();
            let exit_h = self.exit_header.clone();
            let exit_k = self.exit_key.clone();
            let pre_search = tr("To open the application search, press:\n");
            let search = tr(".\n");
            let post_search = tr("This will open a search menu that you can use to run the apps you want.\n\n");
            let pre_kill = tr("To close the currently focused application, press:\n");
            let kill = tr(".\n");
            let post_kill = tr("This will close the currently focused application, potentially destroying unsaved work. \n\n");
            let pre_exit = tr("To return to the login screen, press:\n");
            let exit = tr(".\n");
            let post_exit = tr("This will close out the desktop entirely, destroying all unsaved work.");
            let text = Text::new(format!("{pre_search}{launch_h}+{launch_k}{search}{post_search}{pre_kill}{kill_h}+{kill_k}{kill}{post_kill}{pre_exit}{exit_h}+{exit_k}{exit}{post_exit}")).horizontal_alignment(alignment::Horizontal::Center);
            settings = settings.push(text);
        } else if self.current_page == 2 {
            let title = tr("Workspaces");
            pg_title = Text::new(title);
            let primary_key = self.primary_key.clone();
            let secondary_key = self.secondary_key.clone();
            let head = tr("There are 10 workspaces in this environment-\n In effect each one is its own desktop where you can move applications to or visit the applications located there.\n\n");
            let pre_focus = tr("To move yourself to a workspace, press:\n");
            let focus = tr("+A Number Key.\n");
            let post_focus = tr("This will move you to the workspace corresponding to the number you pressed.\n \n");
            let pre_move = tr("To move the currently focused application to a workspace, press:\n");
            let move_txt = tr("+A Number Key.\n");
            let post_move = tr("This will banish the application to the corresponding workspace.\n\n");
            let immutable = tr("As of now, these bindings are inferred and cannot be directly changed.");
            let text = Text::new(format!("{head}{pre_focus}{primary_key}{focus}{post_focus}{pre_move}{primary_key}+{secondary_key}{move_txt}{post_move}{immutable}")).horizontal_alignment(alignment::Horizontal::Center);
            settings = settings.push(text);
        } else if self.current_page == 3 {
            page_right = page_right.style(style.secondary.mk_theme());
            let title = tr("Minimization");
            pg_title = Text::new(title);
            let mini_h = self.minimize_header.clone();
            let mini_k = self.minimize_key.clone();
            let scratch_h = self.scratch_header.clone();
            let scratch_k = self.scratch_key.clone();
            let pre_move = tr("To minimize the focused application, press:\n");
            let move_txt = tr(".\n");
            let post_move = tr("This will minimize said application, temporarily removing it from the current workspace.\n\n");
            let pre_focus = tr("To show the currently minimized apps, press:\n");
            let focus = tr(".\n");
            let post_focus = tr("This show all of your minimized apps.");
            let text = Text::new(format!("{pre_move}{mini_h}+{mini_k}{move_txt}{post_move}{pre_focus}{scratch_h}+{scratch_k}{focus}{post_focus}")).horizontal_alignment(alignment::Horizontal::Center);
            settings = settings.push(text);
        }
        let left_col = Column::new().width(Length::FillPortion(2))
            .push(page_left);
        let right_col = Column::new().width(Length::FillPortion(2))
            .push(page_right).align_items(Alignment::End);
        let scroll = Scrollable::new(settings);
        let main_col = Column::new().spacing(30).push(pg_title).push(scroll).push(pg_num).align_items(Alignment::Center).width(Length::FillPortion(8));
        let master = Row::new()
            .push(left_col)
            .push(main_col)
            .push(right_col)
            .spacing(30);
        Container::new(master)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
    fn theme(&self) -> Theme {
        let colors = match self.theme {
            OurTheme::Light => self.theme_set.light.application.clone(),
            OurTheme::Dark => self.theme_set.dark.application.clone(),
            OurTheme::Custom => self.theme_set.custom.application.clone()
        };
        let custom = Theme::Custom(Box::new(iced::theme::Custom::new(colors)));
        custom
    }
    fn subscription(&self) -> iced::Subscription<Message> {
        iced::subscription::events_with(
            |event, _| {
                if let iced::Event::Keyboard(keyboard_event) = event {
                    Some(Message::KeyboardUpdate(keyboard_event))
                } else {
                    None
                }
            }
        )
    }
}
