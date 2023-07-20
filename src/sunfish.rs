use iced::theme::{self, Theme};
use iced::{Result, Settings, alignment, Alignment, Length, Application, Command, executor};
use iced::widget::{Button, Row, Column, Container, Text, Scrollable};
use iced_style::Color;
use libcfg::{getcfgdata, decodetheme, OurTheme};
mod libcfg;
use langman::{get_lang, Translation};
mod langman;
mod langcfg;
mod liblocale;
use libstyle::{ButtonStyle, ThemeCustom, make_custom_theme};
mod libstyle;


fn main() -> Result {
    Manual::run(Settings::default())
}

struct Manual {
    theme:OurTheme,
    locale: Translation,
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
    cust_theme: ThemeCustom
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
            locale: get_lang(),
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
            cust_theme: make_custom_theme()
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

        let active_btn = match self.theme {
            OurTheme::Custom => self.cust_theme.body.active.clone(),
            OurTheme::Light => ButtonStyle{
                border_radius: 10.0,
                txt_color: Color::from_rgb8(0x00, 0x20, 0x46),
                bg_color: Color::from_rgb8(0x00, 0xF1, 0xD6),
                border_color: Color::from_rgb8(0, 0, 0),
                border_width: 0.0,
                shadow_offset: iced::Vector {x: 0.0, y: 0.0}
            },
            OurTheme::Dark => ButtonStyle{
                border_radius: 10.0,
                txt_color: Color::from_rgb8(0x00, 0x20, 0x46),
                bg_color: Color::from_rgb8(0x00, 0xCD, 0xB6),
                border_color: Color::from_rgb8(0, 0, 0),
                border_width: 0.0,
                shadow_offset: iced::Vector {x: 0.0, y: 0.0}
            }
        };
        let inactive_btn = match self.theme {
            OurTheme::Custom => self.cust_theme.body.inactive.clone(),
            OurTheme::Light => ButtonStyle{
                border_radius: 10.0,
                txt_color: Color::from_rgb8(0x00, 0x20, 0x46),
                bg_color: Color::from_rgb8(0xC6, 0xEC, 0xFF),
                border_color: Color::from_rgb8(0, 0, 0),
                border_width: 0.0,
                shadow_offset: iced::Vector {x: 0.0, y: 0.0}
            },
            OurTheme::Dark => ButtonStyle{
                border_radius: 10.0,
                txt_color: Color::from_rgb8(0xD2, 0xF0, 0xFF),
                bg_color: Color::from_rgb8(0x00, 0x29, 0x58),
                border_color: Color::from_rgb8(0, 0, 0),
                border_width: 0.0,
                shadow_offset: iced::Vector {x: 0.0, y: 0.0}
            }
        };
        let backtxt = String::as_str(&self.locale.globals.backtxt);
        let forwardtxt = String::as_str(&self.locale.globals.forwardtxt);
        let mut pageleft = Button::new(backtxt)
            .on_press(Message::PageDecr)
            .style(theme::Button::Custom(std::boxed::Box::new(active_btn.clone())));
        let mut pageright = Button::new(forwardtxt)
            .on_press(Message::PageIncr)
            .style(theme::Button::Custom(std::boxed::Box::new(active_btn.clone())));


        let mut settings = Column::new().spacing(10);
        let humanpg = self.current_page+1;
        let pgnum = Text::new(format!("{humanpg}"));
        let mut pgtitle = Text::new("Page Title").horizontal_alignment(alignment::Horizontal::Center);
        if self.current_page == 0 {
            pageleft = pageleft.style(theme::Button::Custom(std::boxed::Box::new(inactive_btn.clone())));
            let title = self.locale.navigation.title.clone();
            pgtitle = Text::new(format!("{title}"));
            let primary_key = self.primary_key.clone();
            let secondary_key = self.secondary_key.clone();
            let prefocus = self.locale.navigation.prefocus.clone();
            let focus = self.locale.navigation.focus.clone();
            let postfocus = self.locale.navigation.postfocus.clone();
            let premove = self.locale.navigation.premove.clone();
            let movestr = self.locale.navigation.movetxt.clone();
            let postmove = self.locale.navigation.postmove.clone();
            let immutable = self.locale.navigation.immutable.clone();
            let text = Text::new(format!("{prefocus}{primary_key}{focus}{postfocus}{premove}{primary_key}+{secondary_key}{movestr}{postmove}{immutable}")).horizontal_alignment(alignment::Horizontal::Center);
            settings = settings.push(text);
        } else if self.current_page == 1 {
            let title = self.locale.advanced.title.clone();
            pgtitle = Text::new(format!("{title}"));
            let launchh = self.launch_header.clone();
            let launchk = self.launch_key.clone();
            let killh = self.kill_header.clone();
            let killk = self.kill_key.clone();
            let exith = self.exit_header.clone();
            let exitk = self.exit_key.clone();
            let presearch = self.locale.advanced.presearch.clone();
            let search = self.locale.advanced.search.clone();
            let postsearch = self.locale.advanced.postsearch.clone();
            let prekill = self.locale.advanced.prekill.clone();
            let kill = self.locale.advanced.kill.clone();
            let postkill = self.locale.advanced.postkill.clone();
            let preexit = self.locale.advanced.preexit.clone();
            let exit = self.locale.advanced.exit.clone();
            let postexit = self.locale.advanced.postexit.clone();
            let text = Text::new(format!("{presearch}{launchh}+{launchk}{search}{postsearch}{prekill}{killh}+{killk}{kill}{postkill}{preexit}{exith}+{exitk}{exit}{postexit}")).horizontal_alignment(alignment::Horizontal::Center);
            settings = settings.push(text);
        } else if self.current_page == 2 {
            let title = self.locale.workspaces.title.clone();
            pgtitle = Text::new(title);
            let primary_key = self.primary_key.clone();
            let secondary_key = self.secondary_key.clone();
            let head = self.locale.workspaces.head.clone();
            let prefocus = self.locale.workspaces.prefocus.clone();
            let focus = self.locale.workspaces.focus.clone();
            let postfocus = self.locale.workspaces.postfocus.clone();
            let premove = self.locale.workspaces.premove.clone();
            let movetxt = self.locale.workspaces.movetxt.clone();
            let postmove = self.locale.workspaces.postmove.clone();
            let immutable = self.locale.workspaces.immutable.clone();
            let text = Text::new(format!("{head}{prefocus}{primary_key}{focus}{postfocus}{premove}{primary_key}+{secondary_key}{movetxt}{postmove}{immutable}")).horizontal_alignment(alignment::Horizontal::Center);
            settings = settings.push(text);
        } else if self.current_page == 3 {
            pageright = pageright.style(theme::Button::Custom(std::boxed::Box::new(inactive_btn.clone())));
            let title = self.locale.minimization.title.clone();
            pgtitle = Text::new(title);
            let minih = self.minimize_header.clone();
            let minik = self.minimize_key.clone();
            let scratchh = self.scratch_header.clone();
            let scratchk = self.scratch_key.clone();
            let premove = self.locale.minimization.premove.clone();
            let movetxt = self.locale.minimization.movetxt.clone();
            let postmove = self.locale.minimization.postmove.clone();
            let prefocus = self.locale.minimization.prefocus.clone();
            let focus = self.locale.minimization.focus.clone();
            let postfocus = self.locale.minimization.postfocus.clone();
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
            OurTheme::Light => iced::theme::Palette{
                background: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                text: Color::from_rgb8(0x00, 0x19, 0x36),
                primary: Color::from_rgb8(0x00, 0x19, 0x36),
                success: Color::from_rgb8(1, 1, 1),
                danger: Color::from_rgb8(1, 1, 1),
                },
            OurTheme::Dark => iced::theme::Palette{
                background: Color::from_rgb8(0x00, 0x19, 0x36),
                text: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                primary: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                success: Color::from_rgb8(1, 1, 1),
                danger: Color::from_rgb8(1, 1, 1),
                },
            OurTheme::Custom => iced::theme::Palette{
                background: self.cust_theme.bg,
                text: self.cust_theme.text,
                primary: Color::from_rgb8(1, 1, 1),
                success: Color::from_rgb8(1, 1, 1),
                danger: Color::from_rgb8(1, 1, 1),
            },
        };
        let cust = Theme::Custom(std::boxed::Box::new(iced::theme::Custom::new(colors)));
        cust
    }
}
