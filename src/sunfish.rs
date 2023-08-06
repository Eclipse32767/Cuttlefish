use iced::theme::Theme;
use iced::{Result, Settings, alignment, Alignment, Length, Application, Command, executor};
use iced::widget::{Button, Row, Column, Container, Text, Scrollable};
use iced::Color;
use libcfg::{getcfgdata, decodetheme, OurTheme};
mod libcfg;
use libstyle::{ButtonStyle, ThemeCustom, make_custom_theme, ThemeSet, ListStyle, MenuStyle};
mod libstyle;
use gettextrs::*;

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
pub fn prettypri(x: &str) -> &'static str {
    match x {
        "super" => "Command/Win",
        "alt" => "Alt",
        "control" => "Control",
        "shift" => "Shift",
        &_ => "Error"
    }
}
pub fn prettyheader(x: &str, pri: &str, sec: &str) -> String {
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
        let data = getcfgdata();
        let pri = prettypri(&data.primary);
        let sec = prettypri(&data.secondary);
        Manual {
            theme: decodetheme(&data.theme, OurTheme::Light),
            current_page: 0,
            primary_key: pri.to_string(),
            secondary_key: sec.to_string(),
            exit_header: prettyheader(&data.exith, pri, sec).to_string(),
            exit_key: data.exitk,
            launch_header: prettyheader(&data.launchh, pri, sec).to_string(),
            launch_key: data.launchk,
            kill_header: prettyheader(&data.killh, pri, sec).to_string(),
            kill_key: data.killk,
            minimize_header: prettyheader(&data.minih, pri, sec).to_string(),
            minimize_key: data.minik,
            scratch_header: prettyheader(&data.scratchh, pri, sec).to_string(),
            scratch_key: data.scratchk,
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
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
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
        
        let backtxt = Text::new(gettext("Back"));
        let forwardtxt = Text::new(gettext("Forwards"));
        let mut pageleft = Button::new(backtxt)
            .on_press(Message::PageDecr);
        let mut pageright = Button::new(forwardtxt)
            .on_press(Message::PageIncr);


        let mut settings = Column::new().spacing(10);
        let humanpg = self.current_page+1;
        let pgnum = Text::new(format!("{humanpg}"));
        let mut pgtitle = Text::new("Page Title").horizontal_alignment(alignment::Horizontal::Center);
        if self.current_page == 0 {
            pageleft = pageleft.style(style.secondary.mk_theme());
            let title = gettext("Basic Navigation");
            pgtitle = Text::new(format!("{title}"));
            let primary_key = self.primary_key.clone();
            let secondary_key = self.secondary_key.clone();
            let prefocus = gettext("To shift focus between applications, press:\n");
            let focus = gettext("+An Arrow Key.\n");
            let postfocus = gettext("This will shift the interface's focus in the direction you pressed.\n \n");
            let premove = gettext("To move applications around, press:\n");
            let movestr = gettext("+An Arrow Key.\n");
            let postmove = gettext("This should swap applications in that direction.\n\n");
            let immutable = gettext("As of now, these bindings are inferred and cannot be directly changed.");
            let text = Text::new(format!("{prefocus}{primary_key}{focus}{postfocus}{premove}{primary_key}+{secondary_key}{movestr}{postmove}{immutable}")).horizontal_alignment(alignment::Horizontal::Center);
            settings = settings.push(text);
        } else if self.current_page == 1 {
            let title = gettext("Basic Navigation, Continued");
            pgtitle = Text::new(format!("{title}"));
            let launchh = self.launch_header.clone();
            let launchk = self.launch_key.clone();
            let killh = self.kill_header.clone();
            let killk = self.kill_key.clone();
            let exith = self.exit_header.clone();
            let exitk = self.exit_key.clone();
            let presearch = gettext("To open the application search, press:\n");
            let search = gettext(".\n");
            let postsearch = gettext("This will open a search menu that you can use to run the apps you want.\n\n");
            let prekill = gettext("To close the currently focused application, press:\n");
            let kill = gettext(".\n");
            let postkill = gettext("This will close the currently focused application, potentially destroying unsaved work. \n\n");
            let preexit = gettext("To return to the login screen, press:\n");
            let exit = gettext(".\n");
            let postexit = gettext("This will close out the desktop entirely, destroying all unsaved work.");
            let text = Text::new(format!("{presearch}{launchh}+{launchk}{search}{postsearch}{prekill}{killh}+{killk}{kill}{postkill}{preexit}{exith}+{exitk}{exit}{postexit}")).horizontal_alignment(alignment::Horizontal::Center);
            settings = settings.push(text);
        } else if self.current_page == 2 {
            let title = gettext("Workspaces");
            pgtitle = Text::new(title);
            let primary_key = self.primary_key.clone();
            let secondary_key = self.secondary_key.clone();
            let head = gettext("There are 10 workspaces in this environment-\n In effect each one is its own desktop where you can move applications to or visit the applications located there.\n\n");
            let prefocus = gettext("To move yourself to a workspace, press:\n");
            let focus = gettext("+A Number Key.\n");
            let postfocus = gettext("This will move you to the workspace corresponding to the number you pressed.\n \n");
            let premove = gettext("To move the currently focused application to a workspace, press:\n");
            let movetxt = gettext("+A Number Key.\n");
            let postmove = gettext("This will banish the application to the corresponding workspace.\n\n");
            let immutable = gettext("As of now, these bindings are inferred and cannot be directly changed.");
            let text = Text::new(format!("{head}{prefocus}{primary_key}{focus}{postfocus}{premove}{primary_key}+{secondary_key}{movetxt}{postmove}{immutable}")).horizontal_alignment(alignment::Horizontal::Center);
            settings = settings.push(text);
        } else if self.current_page == 3 {
            pageright = pageright.style(style.secondary.mk_theme());
            let title = gettext("Minimization");
            pgtitle = Text::new(title);
            let minih = self.minimize_header.clone();
            let minik = self.minimize_key.clone();
            let scratchh = self.scratch_header.clone();
            let scratchk = self.scratch_key.clone();
            let premove = gettext("To minimize the focused application, press:\n");
            let movetxt = gettext(".\n");
            let postmove = gettext("This will minimize said application, temporarily removing it from the current workspace.\n\n");
            let prefocus = gettext("To show the currently minimized apps, press:\n");
            let focus = gettext(".\n");
            let postfocus = gettext("This show all of your minimized apps.");
            let text = Text::new(format!("{premove}{minih}+{minik}{movetxt}{postmove}{prefocus}{scratchh}+{scratchk}{focus}{postfocus}")).horizontal_alignment(alignment::Horizontal::Center);
            settings = settings.push(text);
        }
        let leftcol = Column::new().width(Length::FillPortion(2))
            .push(pageleft);
        let rightcol = Column::new().width(Length::FillPortion(2))
            .push(pageright).align_items(Alignment::End);
        let scroll = Scrollable::new(settings);
        let maincol = Column::new().spacing(30).push(pgtitle).push(scroll).push(pgnum).align_items(Alignment::Center).width(Length::FillPortion(8));
        let master = Row::new()
            .push(leftcol)
            .push(maincol)
            .push(rightcol)
            .spacing(30);
        Container::new(master)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
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
    fn theme(&self) -> Theme {
        let colors = match self.theme {
            OurTheme::Light => self.theme_set.light.application.clone(),
            OurTheme::Dark => self.theme_set.dark.application.clone(),
            OurTheme::Custom => self.theme_set.custom.application.clone()
        };
        let cust = Theme::Custom(std::boxed::Box::new(iced::theme::Custom::new(colors)));
        cust
    }
}
