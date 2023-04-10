use iced::theme::{self, Theme};
use iced::{Result, Settings, alignment, Alignment, Length, Application, Command, executor};
use iced::widget::{Button, Row, Column, Container, Text, Scrollable};
use libcfg::{getcfgdata};
mod libcfg;
use langswayman::{get_lang, Translation};
mod langswayman;

mod liblocale;

fn main() -> Result {
    Manual::run(Settings::default())
}

struct Manual {
    theme:Theme,
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
    scratch_key: String
}

pub fn decodetheme(x: &str, default: Theme) -> Theme {
    match x {
        "dark" => Theme::Dark,
        "light" => Theme::Light,
        &_ => default
    }
}
pub fn prettypri(x: &str) -> &'static str {
    match x {
        "super" => "Command/Win",
        "alt" => "Alt",
        "control" => "Control",
        "shift" => "Shift",
        &_ => panic!()
    }
}
pub fn prettyheader(x: &str, pri: &str, sec: &str) -> String {
    let primary = pri.to_string();
    let secondary = sec.to_string();
    match x {
        "pri" => primary,
        "sec" => secondary,
        "both" => format!("{primary}+{secondary}"),
        &_ => panic!()
    }
}

impl Default for Manual {
    fn default() -> Self {
        let data = getcfgdata();
        let pri = prettypri(&data.primary);
        let sec = prettypri(&data.secondary);
        Manual {
            theme: decodetheme(&data.theme, Theme::Light),
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
        let globalstr = self.locale.globals.as_ref().unwrap();
        let backtxt = String::as_str(&globalstr.backtxt);
        let forwardtxt = String::as_str(&globalstr.forwardtxt);
        let mut pageleft = Button::new(backtxt)
            .on_press(Message::PageDecr);
        let mut pageright = Button::new(forwardtxt)
            .on_press(Message::PageIncr);
        let mut settings = Column::new().spacing(10);
        let humanpg = self.current_page+1;
        let pgnum = Text::new(format!("{humanpg}"));
        let mut pgtitle = Text::new("Page Title").horizontal_alignment(alignment::Horizontal::Center);
        if self.current_page == 0 {
            let navstr = self.locale.navigation.as_ref().unwrap();
            pageleft = pageleft.style(theme::Button::Secondary);
            let title = navstr.title.clone();
            pgtitle = Text::new(format!("{title}"));
            let primary_key = self.primary_key.clone();
            let secondary_key = self.secondary_key.clone();
            let prefocus = navstr.prefocus.clone();
            let focus = navstr.focus.clone();
            let postfocus = navstr.postfocus.clone();
            let premove = navstr.premove.clone();
            let movestr = navstr.movetxt.clone();
            let postmove = navstr.postmove.clone();
            let immutable = navstr.immutable.clone();
            let text = Text::new(format!("{prefocus}{primary_key}{focus}{postfocus}{premove}{primary_key}+{secondary_key}{movestr}{postmove}{immutable}")).horizontal_alignment(alignment::Horizontal::Center);
            settings = settings.push(text);
        } else if self.current_page == 1 {
            let advstr = self.locale.advanced.as_ref().unwrap();
            let title = advstr.title.clone();
            pgtitle = Text::new(format!("{title}"));
            let launchh = self.launch_header.clone();
            let launchk = self.launch_key.clone();
            let killh = self.kill_header.clone();
            let killk = self.kill_key.clone();
            let exith = self.exit_header.clone();
            let exitk = self.exit_key.clone();
            let presearch = advstr.presearch.clone();
            let search = advstr.search.clone();
            let postsearch = advstr.postsearch.clone();
            let prekill = advstr.prekill.clone();
            let kill = advstr.kill.clone();
            let postkill = advstr.postkill.clone();
            let preexit = advstr.preexit.clone();
            let exit = advstr.exit.clone();
            let postexit = advstr.postexit.clone();
            let text = Text::new(format!("{presearch}{launchh}+{launchk}{search}{postsearch}{prekill}{killh}+{killk}{kill}{postkill}{preexit}{exith}+{exitk}{exit}{postexit}")).horizontal_alignment(alignment::Horizontal::Center);
            settings = settings.push(text);
        } else if self.current_page == 2 {
            let workstr = self.locale.workspaces.as_ref().unwrap();
            let title = workstr.title.clone();
            pgtitle = Text::new(title);
            let primary_key = self.primary_key.clone();
            let secondary_key = self.secondary_key.clone();
            let head = workstr.head.clone();
            let prefocus = workstr.prefocus.clone();
            let focus = workstr.focus.clone();
            let postfocus = workstr.postfocus.clone();
            let premove = workstr.premove.clone();
            let movetxt = workstr.movetxt.clone();
            let postmove = workstr.postmove.clone();
            let immutable = workstr.immutable.clone();
            let text = Text::new(format!("{head}{prefocus}{primary_key}{focus}{postfocus}{premove}{primary_key}+{secondary_key}{movetxt}{postmove}{immutable}")).horizontal_alignment(alignment::Horizontal::Center);
            settings = settings.push(text);
        } else if self.current_page == 3 {
            let ministr = self.locale.minimization.as_ref().unwrap();
            pageright = pageright.style(theme::Button::Secondary);
            let title = ministr.title.clone();
            pgtitle = Text::new(title);
            let minih = self.minimize_header.clone();
            let minik = self.minimize_key.clone();
            let scratchh = self.scratch_header.clone();
            let scratchk = self.scratch_key.clone();
            let premove = ministr.premove.clone();
            let movetxt = ministr.movetxt.clone();
            let postmove = ministr.postmove.clone();
            let prefocus = ministr.prefocus.clone();
            let focus = ministr.focus.clone();
            let postfocus = ministr.postfocus.clone();
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
        self.theme.clone()
    }
}
