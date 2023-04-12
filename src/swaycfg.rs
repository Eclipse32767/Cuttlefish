use iced::theme::{self, Theme};
use iced::{Result, Application, Settings, Alignment, Length, executor};
use iced::widget::{Button, Row, Column, Container, pick_list, Text, Scrollable};
use iced_style::Color;
use libcfg::{getcfgdata, BindKey, Border, ShortcutKey, decodeheader, decodeborder, decodepri, decodetheme, mkwmcfg, mkselfcfg};
mod libcfg;
use langswaycfg::{get_lang, Translation};
mod langswaycfg;
use libstyle::ButtonStyle;
mod libstyle;
mod liblocale;

fn main() -> Result {
    Configurator::run(Settings::default())
}


struct Configurator {
    theme: Theme,
    locale: Translation,
    border: Option<Border>,
    current_page: Page,
    primary_key: Option<ShortcutKey>,
    secondary_key: Option<ShortcutKey>,
    exit_header: Option<BindKey>,
    exit_key: String,
    launch_header: Option<BindKey>,
    launch_key: String,
    kill_header: Option<BindKey>,
    kill_key: String,
    minimize_header: Option<BindKey>,
    minimize_key: String,
    scratch_header: Option<BindKey>,
    scratch_key: String,
    width: i32,
    unsaved: bool,
    capturenext: Option<CaptureInput>,
    index: u8,
    indexmax: u8,
}

#[derive(PartialEq, Debug, Clone)]
enum CaptureInput {
    NoKey,
    ExitKey,
    LaunchKey,
    KillKey,
    MiniKey,
    ScratchKey
}

impl Default for Configurator {
    fn default() -> Self {
        let data = getcfgdata();
        Configurator {
            theme: decodetheme(&data.theme, Theme::Light),
            locale: get_lang(),
            border: decodeborder(&data.border, Border::Normal),
            current_page: Page::Main,
            primary_key: decodepri(&data.primary, ShortcutKey::Super),
            secondary_key: decodepri(&data.secondary, ShortcutKey::Shift),
            exit_header: decodeheader(&data.exith, BindKey::BothKey),
            exit_key: data.exitk,
            launch_header: decodeheader(&data.launchh, BindKey::PrimaryKey),
            launch_key: data.launchk,
            kill_header: decodeheader(&data.killh, BindKey::BothKey),
            kill_key: data.killk,
            minimize_header: decodeheader(&data.minih, BindKey::BothKey),
            minimize_key: data.minik,
            scratch_header: decodeheader(&data.scratchh, BindKey::PrimaryKey),
            scratch_key: data.scratchk,
            width: data.width,
            unsaved: false,
            capturenext: Some(CaptureInput::NoKey),
            index: 0,
            indexmax: 3,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Save,
    ThemeLight,
    ThemeDark,
    BorderToggled(Border),
    PageChanged(Page),
    PrimaryKeyChanged(ShortcutKey),
    SecondaryKeyChanged(ShortcutKey),
    ExitHeaderChanged(BindKey),
    LaunchHeaderChanged(BindKey),
    KillHeaderChanged(BindKey),
    MiniHeaderChanged(BindKey),
    ScratchHeaderChanged(BindKey),
    WidthIncr,
    WidthDecr,
    KeyboardUpdate(iced::keyboard::Event),
    Capture(CaptureInput)
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Page {
    #[default]
    Main,
    Bind,
    Bar,
    Init,
    Anim
}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let locale = get_lang();
        let pretty = locale.prettyprint.unwrap();
        write!(
            f,
            "{}",
            match self {
                Page::Main => pretty.pagemain,
                Page::Bind => pretty.pagebind,
                Page::Bar => pretty.pagebar,
                Page::Init => pretty.pageinit,
                Page::Anim => pretty.pageanim,
            }
        )
    }
}

impl Application for Configurator {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();
    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        (
            Self::default(),
            iced::Command::none()
        )
    }
    fn title(&self) -> String {
        let globalstr = self.locale.global.as_ref().unwrap();
        let title = globalstr.title.clone();
        format!("{title}{}", self.current_page.to_string())
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        match message {
            Message::Save => {
                if self.unsaved {
                    mkwmcfg(self.primary_key, self.secondary_key, self.exit_header, self.exit_key.clone(), self.launch_header, self.launch_key.clone(), self.kill_header, self.kill_key.clone(), self.minimize_header, self.minimize_key.clone(), self.scratch_header, self.scratch_key.clone(), self.border, self.width);
                    mkselfcfg(self.primary_key, self.secondary_key, self.exit_header, self.exit_key.clone(), self.launch_header, self.launch_key.clone(), self.kill_header, self.kill_key.clone(), self.minimize_header, self.minimize_key.clone(), self.scratch_header, self.scratch_key.clone(), self.border, self.width, self.theme.clone());
                }
                self.unsaved = false;
                iced::Command::none()
            }
            Message::ThemeLight => {
                self.theme = Theme::Light;
                self.unsaved = true;
                iced::Command::none()
            }
            Message::ThemeDark => {
                self.theme = Theme::Dark;
                self.unsaved = true;
                iced::Command::none()
            }
            Message::BorderToggled(x) => {
                self.border = Some(x);
                self.unsaved = true;
                iced::Command::none()
            }
            Message::PageChanged(x) => {
                self.current_page = x;
                match x {
                    Page::Main => {
                        self.indexmax = 3;
                    }
                    Page::Bind => {
                        self.indexmax = 6;
                    }
                    Page::Bar => {
                        self.indexmax = 0;
                    }
                    Page::Init => {
                        self.indexmax = 0;
                    }
                    Page::Anim => {
                        self.indexmax = 0;
                    }
                }
                iced::Command::none()
            }
            Message::PrimaryKeyChanged(x) => {
                self.primary_key = Some(x);
                self.unsaved = true;
                iced::Command::none()
            }
            Message::SecondaryKeyChanged(x) => {
                self.secondary_key = Some(x);
                self.unsaved = true;
                iced::Command::none()
            }
            Message::ExitHeaderChanged(x) => {
                self.exit_header = Some(x);
                self.unsaved = true;
                iced::Command::none()
            }
            Message::LaunchHeaderChanged(x) => {
                self.launch_header = Some(x);
                self.unsaved = true;
                iced::Command::none()
            }
            Message::KillHeaderChanged(x) => {
                self.kill_header = Some(x);
                self.unsaved = true;
                iced::Command::none()
            }
            Message::MiniHeaderChanged(x) => {
                self.minimize_header = Some(x);
                self.unsaved = true;
                iced::Command::none()
            }
            Message::ScratchHeaderChanged(x) => {
                self.scratch_header = Some(x);
                self.unsaved = true;
                iced::Command::none()
            }
            Message::WidthIncr => {
                if self.width <= 19 {
                    self.width = self.width + 1;
                    self.unsaved = true;
                }
                iced::Command::none()
            }
            Message::WidthDecr => {
                if self.width >= 1 {
                    self.width = self.width -1;
                    self.unsaved = true;
                }
                iced::Command::none()
            }
            Message::KeyboardUpdate(x) => {
                match x {
                    iced::keyboard::Event::KeyPressed { key_code, modifiers} => {
                        match self.capturenext.as_ref().unwrap() {
                            &CaptureInput::NoKey => {
                                if key_code == iced::keyboard::KeyCode::Up {
                                    if iced::keyboard::Modifiers::shift(modifiers) {//go up a page
                                        self.current_page = match self.current_page {
                                            Page::Main => {
                                                Page::Init
                                            }
                                            Page::Bind => {
                                                self.indexmax = 3;
                                                Page::Main
                                            }
                                            Page::Anim => {
                                                self.indexmax = 6;
                                                Page::Bind
                                            }
                                            Page::Bar => {
                                                Page::Anim
                                            }
                                            Page::Init => {
                                                Page::Bar
                                            }
                                        }
                                    } else {
                                        if self.index != 0 {
                                            self.index = self.index -1;
                                        }
                                    }
                                } else if key_code == iced::keyboard::KeyCode::Down {
                                    if iced::keyboard::Modifiers::shift(modifiers) {//go down a page
                                        self.current_page = match self.current_page {
                                            Page::Main => {
                                                self.indexmax = 6;
                                                Page::Bind
                                            }
                                            Page::Bind => {
                                                Page::Anim
                                            }
                                            Page::Anim => {
                                                Page::Bar
                                            }
                                            Page::Bar => {
                                                Page::Init
                                            }
                                            Page::Init => {
                                                self.indexmax = 3;
                                                Page::Main
                                            }
                                       }
                                    } else {
                                        if self.index != self.indexmax {
                                            self.index = self.index +1;
                                        }
                                    }
                                } else if key_code == iced::keyboard::KeyCode::S { //save
                                    if self.unsaved {
                                        mkwmcfg(self.primary_key, self.secondary_key, self.exit_header, self.exit_key.clone(), self.launch_header, self.launch_key.clone(), self.kill_header, self.kill_key.clone(), self.minimize_header, self.minimize_key.clone(), self.scratch_header, self.scratch_key.clone(), self.border, self.width);
                                        mkselfcfg(self.primary_key, self.secondary_key, self.exit_header, self.exit_key.clone(), self.launch_header, self.launch_key.clone(), self.kill_header, self.kill_key.clone(), self.minimize_header, self.minimize_key.clone(), self.scratch_header, self.scratch_key.clone(), self.border, self.width, self.theme.clone());
                                    }
                                    self.unsaved = false;
                            } else if key_code == iced::keyboard::KeyCode::Enter {
                                match self.current_page {
                                    Page::Main => {
                                        if self.index == 0 {
                                            if self.theme == Theme::Dark {
                                                self.theme = Theme::Light;
                                            } else {
                                                self.theme = Theme::Dark;
                                            }
                                            self.unsaved = true;
                                        }
                                    }
                                    Page::Bind => {

                                    }
                                    Page::Bar => {

                                    }
                                    Page::Init => {

                                    }
                                    Page::Anim => {

                                    }
                                }
                            }
                            } 
                            &CaptureInput::ExitKey => {
                                self.exit_key = format!("{:?}", key_code);
                                self.capturenext = Some(CaptureInput::NoKey);
                                self.unsaved = true;
                            }
                            &CaptureInput::LaunchKey => {
                                self.launch_key = format!("{:?}", key_code);
                                self.capturenext = Some(CaptureInput::NoKey);
                                self.unsaved = true;
                            }
                            &CaptureInput::KillKey => {
                                self.kill_key = format!("{:?}", key_code);
                                self.capturenext = Some(CaptureInput::NoKey);
                                self.unsaved = true;
                            }
                            &CaptureInput::MiniKey => {
                                self.minimize_key = format!("{:?}", key_code);
                                self.capturenext = Some(CaptureInput::NoKey);
                                self.unsaved = true;
                            }
                            &CaptureInput::ScratchKey => {
                                self.scratch_key = format!("{:?}", key_code);
                                self.capturenext = Some(CaptureInput::NoKey);
                                self.unsaved = true;
                            }
                        }
                    }
                    iced::keyboard::Event::KeyReleased {..} => {

                    }
                    iced::keyboard::Event::CharacterReceived(..) => {

                    }
                    iced::keyboard::Event::ModifiersChanged(..) => {

                    }
                }
                iced::Command::none()
            }
            Message::Capture(x) => {
                self.capturenext = Some(x);
                iced::Command::none()
            }
        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {

        //let sidebarActiveBtn = ButtonStyle{ border_radius: 10.0, txt_color: Color::from_rgb(202.0, 211.0, 245.0), bg_color: Color::from_rgb(36.0, 39.0, 58.0), border_color: Color::from_rgb(0.0, 0.0, 0.0), border_width: 1.0};
        let sidebarActiveBtn = ButtonStyle{
            border_radius: 10.0,
            txt_color: Color::from_rgb8( 0xCA, 0xD3, 0xF5),
            bg_color: Color::from_rgb8(0x24, 0x27, 0x3A),
            border_color: Color::from_rgb8(0, 0, 0),
            border_width: 0.0,
            shadow_offset: iced::Vector {x: 0.0, y: 0.0}
        };
        let sidebarInactiveBtn = ButtonStyle{
            border_radius: 10.0,
            txt_color: Color::from_rgb8(0xA5, 0xAD, 0xCB),
            bg_color: Color::from_rgb8(0x18, 0x19, 0x26),
            border_color: Color::from_rgb8(0, 0, 0),
            border_width: 0.0,
            shadow_offset: iced::Vector {x: 0.0, y: 0.0}
        };

        let selectionmarker: Text = Text::new("=>");
        let globalstr = self.locale.global.as_ref().unwrap();
        let mainstr = self.locale.mainpage.as_ref().unwrap();
        let bindstr = self.locale.bindpage.as_ref().unwrap();

        let maintxt = String::as_str(&globalstr.main);
        let bindtxt = String::as_str(&globalstr.bind);
        let bartxt = String::as_str(&globalstr.bar);
        let inittxt = String::as_str(&globalstr.init);
        let animtxt = String::as_str(&globalstr.anim);
        let pagetxt = String::as_str(&globalstr.label);
        let mut pagemain = Button::new(maintxt)
            .on_press(Message::PageChanged(Page::Main))
            .width(150)
            .style(theme::Button::Custom(std::boxed::Box::new(sidebarActiveBtn.clone())));
        let mut pagebind = Button::new(bindtxt)
            .on_press(Message::PageChanged(Page::Bind))
            .width(150)
            .style(theme::Button::Custom(std::boxed::Box::new(sidebarActiveBtn.clone())));
        let mut pagebar = Button::new(bartxt)
            .on_press(Message::PageChanged(Page::Bar))
            .width(150)
            .style(theme::Button::Custom(std::boxed::Box::new(sidebarActiveBtn.clone())));
        let mut pageinit = Button::new(inittxt)
            .on_press(Message::PageChanged(Page::Init))
            .width(150)
            .style(theme::Button::Custom(std::boxed::Box::new(sidebarActiveBtn.clone())));
        let mut pageanim = Button::new(animtxt)
            .on_press(Message::PageChanged(Page::Anim))
            .width(150)
            .style(theme::Button::Custom(std::boxed::Box::new(sidebarActiveBtn.clone())));
        let pagelabel = Text::new(pagetxt);
        match self.current_page {
            Page::Main => {
                pagemain = pagemain.style(theme::Button::Custom(std::boxed::Box::new(sidebarInactiveBtn.clone())));
            }
            Page::Bind => {
                pagebind = pagebind.style(theme::Button::Custom(std::boxed::Box::new(sidebarInactiveBtn.clone())));
            }
            Page::Bar => {
                pagebar = pagebar.style(theme::Button::Custom(std::boxed::Box::new(sidebarInactiveBtn.clone())));
            }
            Page::Init => {
                pageinit = pageinit.style(theme::Button::Custom(std::boxed::Box::new(sidebarInactiveBtn.clone())));
            }
            Page::Anim => {
                pageanim = pageanim.style(theme::Button::Custom(std::boxed::Box::new(sidebarInactiveBtn.clone())));
            }
        }
        let pagecol = Column::new()
            .push(pagelabel)
            .push(pagemain)
            .push(pagebind)
            .push(pageanim)
            .push(pagebar)
            .push(pageinit)
            .spacing(10)
            .align_items(Alignment::Start);

        let save;
        let savetxt = String::as_str(&globalstr.save);
        let savedtxt = String::as_str(&globalstr.saved);
        if self.unsaved {
            save = Button::new(savetxt)
            .on_press(Message::Save);
        } else {
            save = Button::new(savedtxt)
            .on_press(Message::Save)
            .style(theme::Button::Secondary);
        }
        let saverow = Row::new()
            .push(save)
            .align_items(Alignment::Center);

        
        let mut settings = Column::new().spacing(10);


        match self.current_page {
            Page::Main => {

                let primarypick = pick_list(
                    &ShortcutKey::ALL[..], 
                    self.primary_key, 
                    Message::PrimaryKeyChanged,
                    )
                    .placeholder("choose");
                let secondarypick = pick_list(
                    &ShortcutKey::ALL[..], 
                    self.secondary_key, 
                    Message::SecondaryKeyChanged,
                    )
                    .placeholder("choose");
                let primarytxt = String::as_str(&globalstr.primary);
                let secondarytxt = String::as_str(&globalstr.secondary);
                let primarylabel: Text = Text::new(primarytxt);
                let secondarylabel: Text = Text::new(secondarytxt);

                let bordertoggle = pick_list(
                    &Border::ALL[..], 
                    self.border, 
                    Message::BorderToggled,
                    )
                    .placeholder("choose");
                let borderlabel = Text::new(mainstr.borders.clone());
                let widthlabel: Text = Text::new(format!("{} {}px", mainstr.width, self.width));
                let mut widthup = Button::new("+")
                    .on_press(Message::WidthIncr);
                let mut widthdown = Button::new("-")
                    .on_press(Message::WidthDecr);
                if self.width == 20 {
                    widthup = widthup.style(theme::Button::Secondary);
                } else if self.width == 0 {
                    widthdown = widthdown.style(theme::Button::Secondary);
                }

                let lighttxt = String::as_str(&mainstr.light);
                let darktxt = String::as_str(&mainstr.dark);
                let themetxt = String::as_str(&mainstr.theme);
                let mut light = Button::new(lighttxt)
                    .on_press(Message::ThemeLight);
                let mut dark = Button::new(darktxt)
                    .on_press(Message::ThemeDark);
                let themelabel = Text::new(themetxt);
                match self.theme {
                    Theme::Light => {
                        light = light.style(theme::Button::Secondary);
                    }
                    Theme::Dark => {
                        dark = dark.style(theme::Button::Secondary);
                    }
                    Theme::Custom(..) => {
                        panic!("oops");
                    }
                }
                let mut borderrow = Row::new().spacing(10);
                let mut themerow = Row::new().spacing(10);
                let mut primaryrow = Row::new().spacing(10);
                let mut secondaryrow = Row::new().spacing(10);

                if self.index == 0 {
                    themerow = themerow.push(selectionmarker);
                } else if self.index == 1 {
                    borderrow = borderrow.push(selectionmarker);
                } else if self.index == 2 {
                    primaryrow = primaryrow.push(selectionmarker);
                } else if self.index == 3 {
                    secondaryrow = secondaryrow.push(selectionmarker);
                }

                borderrow = borderrow
                    .push(borderlabel)
                    .push(bordertoggle)
                    .push(widthlabel)
                    .push(widthup)
                    .push(widthdown);
                themerow = themerow
                    .push(themelabel)
                    .push(light)
                    .push(dark);
                primaryrow = primaryrow
                    .push(primarylabel)
                    .push(primarypick);
                secondaryrow = secondaryrow
                    .push(secondarylabel)
                    .push(secondarypick);
        
                settings = settings
                    .push(themerow)
                    .push(borderrow)
                    .push(primaryrow)
                    .push(secondaryrow);
            }
            Page::Bind => {
                let primarypick = pick_list(
                    &ShortcutKey::ALL[..], 
                    self.primary_key, 
                    Message::PrimaryKeyChanged,
                    )
                    .placeholder("choose");
                let secondarypick = pick_list(
                    &ShortcutKey::ALL[..], 
                    self.secondary_key, 
                    Message::SecondaryKeyChanged,
                    )
                    .placeholder("choose");
                let primarytxt = String::as_str(&globalstr.primary);
                let secondarytxt = String::as_str(&globalstr.secondary);
                let primarylabel: Text = Text::new(primarytxt);
                let secondarylabel: Text = Text::new(secondarytxt);


                let exitsclabel = Text::new(bindstr.exit.clone());
                let exitheaderselect = pick_list(
                &BindKey::ALL[..], 
                self.exit_header, 
                Message::ExitHeaderChanged,
                )
                .placeholder("choose");
                let exitkey = String::as_str(&self.exit_key);
                let mut exitkeyselect = Button::new(exitkey).on_press(Message::Capture(CaptureInput::ExitKey)).width(50);
                let launchsclabel: Text = Text::new(bindstr.launch.clone());
                let launchheaderselect = pick_list(
                    &BindKey::ALL[..], 
                    self.launch_header, 
                    Message::LaunchHeaderChanged,
                    )
                    .placeholder("choose");
                let launchkey = String::as_str(&self.launch_key);
                let mut launchkeyselect = Button::new(launchkey).on_press(Message::Capture(CaptureInput::LaunchKey)).width(50);
                let killsclabel: Text = Text::new(bindstr.kill.clone());
                let killheaderselect = pick_list(
                    &BindKey::ALL[..], 
                    self.kill_header, 
                    Message::KillHeaderChanged,
                    )
                    .placeholder("choose");
                let killkey = String::as_str(&self.kill_key);
                let mut killkeyselect = Button::new(killkey).on_press(Message::Capture(CaptureInput::KillKey)).width(50);
                let minisclabel: Text = Text::new(bindstr.mini.clone());
                let miniheaderselect = pick_list(
                 &BindKey::ALL[..], 
                 self.minimize_header, 
                 Message::MiniHeaderChanged,
                 )
                    .placeholder("choose");
                let minikey = String::as_str(&self.minimize_key);
                let mut minikeyselect = Button::new(minikey).on_press(Message::Capture(CaptureInput::MiniKey)).width(50);
                let scratchsclabel: Text = Text::new(bindstr.scratch.clone());
                let scratchheaderselect = pick_list(
                    &BindKey::ALL[..], 
                    self.scratch_header, 
                    Message::ScratchHeaderChanged,
                    )
                    .placeholder("choose");
                let scratchkey = String::as_str(&self.scratch_key);
                let mut scratchkeyselect = Button::new(scratchkey).on_press(Message::Capture(CaptureInput::ScratchKey)).width(50);
                
                match self.capturenext.as_ref().unwrap() {
                    CaptureInput::NoKey => {
                    }
                    CaptureInput::ExitKey => {
                        exitkeyselect = exitkeyselect.style(theme::Button::Secondary);
                    }
                    CaptureInput::KillKey => {
                        killkeyselect = killkeyselect.style(theme::Button::Secondary);
                    }
                    CaptureInput::LaunchKey => {
                        launchkeyselect = launchkeyselect.style(theme::Button::Secondary);
                    }
                    CaptureInput::MiniKey => {
                        minikeyselect = minikeyselect.style(theme::Button::Secondary);
                    }
                    CaptureInput::ScratchKey => {
                        scratchkeyselect = scratchkeyselect.style(theme::Button::Secondary);
                    }
                }

                let primaryrow = Row::new()
                    .push(primarylabel)
                    .push(primarypick)
                    .spacing(10);
                let secondaryrow = Row::new()
                    .push(secondarylabel)
                    .push(secondarypick)
                    .spacing(10);
                let exitscrow = Row::new()
                    .push(exitsclabel)
                    .push(exitheaderselect)
                    .push(exitkeyselect)
                    .spacing(10);
                let launchscrow = Row::new()
                    .push(launchsclabel)
                    .push(launchheaderselect)
                    .push(launchkeyselect)
                    .spacing(10);
                let killscrow = Row::new()
                    .push(killsclabel)
                    .push(killheaderselect)
                    .push(killkeyselect)
                    .spacing(10);
                let miniscrow = Row::new()
                    .push(minisclabel)
                    .push(miniheaderselect)
                    .push(minikeyselect)
                    .spacing(10);
                let scratchscrow = Row::new()
                    .push(scratchsclabel)
                    .push(scratchheaderselect)
                    .push(scratchkeyselect)
                    .spacing(10);
                settings = settings
                    .push(primaryrow)
                    .push(secondaryrow)
                    .push(exitscrow)
                    .push(launchscrow)
                    .push(killscrow)
                    .push(miniscrow)
                    .push(scratchscrow);
            }
            Page::Bar => {

            }
            Page::Init => {

            }
            Page::Anim => {

            }
        }

        let scroll = Scrollable::new(settings);
        let col = Column::new()
            .push(scroll)
            .push(saverow)
            .width(Length::Fill)
            .align_items(Alignment::Start)
            .spacing(10);
        let master = Row::new()
            .push(pagecol)
            .push(col)
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
