use iced::theme::Theme;
use iced::{Result, Application, Settings, Alignment, Length, executor};
use iced::widget::{Button, Row, Column, Container, Text, Scrollable};
use iced::keyboard::KeyCode;
use iced::Color;
use libcfg::{getcfgdata, BindKey, ShortcutKey, OurTheme, BarWidget, WindowAnimation, WorkAnimation, Border, decodeheader, decodepri, decodetheme, mkwmcfg, mkselfcfg, decodewinanim, decodeworkanim, decodeblur};
mod libcfg;
use libstyle::{ButtonStyle, ListStyle, MenuStyle, ThemeCustom, make_custom_theme, ThemeSet};
mod libstyle;
use gettextrs::*;
mod cuttlefish_pages;


//This is Cuttlefish, Our Configuration Tool

fn main() -> Result {
    let _ = textdomain("CuttlefishCfg");
    let _ = bind_textdomain_codeset("CuttlefishCfg", "UTF-8");
    Configurator::run(Settings::default())
}


struct Configurator { //The basic configurator struct, contains most program state
    theme: OurTheme,
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
    unsaved: bool,
    capturenext: Option<CaptureInput>,
    index: u8,
    indexmax: u8,
    border: Border,
    window_anim: Option<WindowAnimation>,
    work_anim: Option<WorkAnimation>,
    blur: bool,
    theme_set: ThemeSet,
    width: ShrinkValue,
    bar_left: Vec<BarWidget>,
    bar_center: Vec<BarWidget>,
    bar_right: Vec<BarWidget>,
    next_widget: Option<BarWidget>,
}
#[derive(PartialEq, Debug, Clone)]
enum CaptureInput { //enum used to store what binding should be captured into
    NoKey,
    ExitKey,
    LaunchKey,
    KillKey,
    MiniKey,
    ScratchKey
}
#[derive(PartialEq, Debug, Clone)]
enum WidgetBank {
    Left,
    Center,
    Right
}
#[derive(PartialEq, Debug, Clone)]
enum ShrinkValue {
    Full,
    Medium,
    Tiny
}

impl Default for Configurator {
    fn default() -> Self {
        let data = getcfgdata();
        Configurator { //here we extract all of the data from the config file
            theme: decodetheme(&data.theme, OurTheme::Light),
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
            unsaved: false,
            capturenext: Some(CaptureInput::NoKey),
            index: 0,
            indexmax: 2,
            border: data.border.clone().unwrap(),
            window_anim: decodewinanim(&data.winanim, WindowAnimation::None),
            work_anim: decodeworkanim(&data.workanim, WorkAnimation::None),
            blur: decodeblur(&data.blur),
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
            width: ShrinkValue::Full,
            bar_left: vec![],
            bar_center: vec![],
            bar_right: vec![],
            next_widget: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Message { // The Message enum, used to send data to the configurator's update function
    Save,
    ThemeChanged(OurTheme),
    PageChanged(Page),
    PrimaryKeyChanged(ShortcutKey),
    SecondaryKeyChanged(ShortcutKey),
    ExitHeaderChanged(BindKey),
    LaunchHeaderChanged(BindKey),
    KillHeaderChanged(BindKey),
    MiniHeaderChanged(BindKey),
    ScratchHeaderChanged(BindKey),
    KeyboardUpdate(iced::keyboard::Event),
    Capture(CaptureInput),
    Incr(IncrVal),
    Decr(IncrVal),
    ChangeWindowAnim(WindowAnimation),
    ChangeWorkAnim(WorkAnimation),
    BlurToggled(bool),
    WindowUpdate(iced::window::Event),
    AwaitDestination(BarWidget),
    PushWidget(WidgetBank),
    RemoveWidget(WidgetBank)
}
#[derive(Debug, Clone)]
enum IncrVal {
    WidthVal,
    RadiusVal,
    GapsVal,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Page { //page enum, used to store the currently focused page
    #[default]
    Main,
    Bind,
    Bar,
    Init,
    Anim
}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self { //respect locale preferences when prettyprinting
                Page::Main => gettext("Main Page"),
                Page::Bind => gettext("Keybindings Page"),
                Page::Bar => gettext("Status Bar Page"),
                Page::Init => gettext("Autostart Page"),
                Page::Anim => gettext("Animations Page"),
            }
        )
    }
}

impl Application for Configurator {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();
    fn new(_flags: ()) -> (Self, iced::Command<Message>) { //code that initializes the app
        (
            Self::default(),
            iced::Command::none()
        )
    }
    fn title(&self) -> String { //code that sets the app title
        format!("{}{}", gettext("Cuttlefish Configurator--"), self.current_page.to_string())
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<Message> { //update function, parses messages
        match message {
            Message::Save => {
                if self.unsaved {
                    mkwmcfg(self.primary_key, self.secondary_key, self.exit_header, self.exit_key.clone(), self.launch_header, self.launch_key.clone(), self.kill_header, self.kill_key.clone(), self.minimize_header, self.minimize_key.clone(), self.scratch_header, self.scratch_key.clone(), Some(self.border), self.window_anim, self.work_anim, self.blur);
                    mkselfcfg(self.primary_key, self.secondary_key, self.exit_header, self.exit_key.clone(), self.launch_header, self.launch_key.clone(), self.kill_header, self.kill_key.clone(), self.minimize_header, self.minimize_key.clone(), self.scratch_header, self.scratch_key.clone(), self.theme.clone(), Some(self.border), self.window_anim, self.work_anim, self.blur);
                }
                self.unsaved = false;
                iced::Command::none()
            }
            Message::ThemeChanged(x) => {
                self.theme = x;
                self.unsaved = true;
                iced::Command::none()
            }
            Message::PageChanged(x) => {
                self.current_page = x;
                match x {
                    Page::Main => {
                        self.indexmax = 2;
                    }
                    Page::Bind => {
                        self.indexmax = 6;
                    }
                    Page::Bar => {
                        self.indexmax = 7;
                    }
                    Page::Init => {
                        self.indexmax = 0;
                    }
                    Page::Anim => {
                        self.indexmax = 5;
                    }
                }
                if self.index > self.indexmax {
                    self.index = self.indexmax;
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
            Message::KeyboardUpdate(x) => { //keyboard event parser
                match x {
                    iced::keyboard::Event::KeyPressed { key_code, modifiers} => { // code for handling keypresses
                        match self.capturenext.as_ref().unwrap() { //check if next input should be captured
                            &CaptureInput::NoKey => { // if no captures are wanted, go through this parsing block
                                if key_code == KeyCode::Up {
                                    if iced::keyboard::Modifiers::shift(modifiers) {//go up a page
                                        self.current_page = match self.current_page {
                                            Page::Main => {
                                                self.indexmax = 0;
                                                Page::Init
                                            }
                                            Page::Bind => {
                                                self.indexmax = 2;
                                                Page::Main
                                            }
                                            Page::Anim => {
                                                self.indexmax = 6;
                                                Page::Bind
                                            }
                                            Page::Bar => {
                                                self.indexmax = 5;
                                                Page::Anim
                                            }
                                            Page::Init => {
                                                self.indexmax = 7;
                                                Page::Bar
                                            }
                                        };
                                        if self.index > self.indexmax {
                                            self.index = self.indexmax;
                                        }
                                    } else { //move the minicursor up
                                        if self.index != 0 {
                                            self.index = self.index -1;
                                        }
                                    }
                                } else if key_code == KeyCode::Down {
                                    if iced::keyboard::Modifiers::shift(modifiers) {//go down a page
                                        self.current_page = match self.current_page {
                                            Page::Main => {
                                                self.indexmax = 6;
                                                Page::Bind
                                            }
                                            Page::Bind => {
                                                self.indexmax = 5;
                                                Page::Anim
                                            }
                                            Page::Anim => {
                                                self.indexmax = 7;
                                                Page::Bar
                                            }
                                            Page::Bar => {
                                                self.indexmax = 0;
                                                Page::Init
                                            }
                                            Page::Init => {
                                                self.indexmax = 2;
                                                Page::Main
                                            }
                                       }
                                    } else { //move the minicursor down
                                        if self.index < self.indexmax {
                                            self.index = self.index +1;
                                        }
                                    }
                                } else if key_code == KeyCode::S { //save
                                    if self.unsaved {
                                        mkwmcfg(self.primary_key, self.secondary_key, self.exit_header, self.exit_key.clone(), self.launch_header, self.launch_key.clone(), self.kill_header, self.kill_key.clone(), self.minimize_header, self.minimize_key.clone(), self.scratch_header, self.scratch_key.clone(), Some(self.border), self.window_anim, self.work_anim, self.blur);
                                        mkselfcfg(self.primary_key, self.secondary_key, self.exit_header, self.exit_key.clone(), self.launch_header, self.launch_key.clone(), self.kill_header, self.kill_key.clone(), self.minimize_header, self.minimize_key.clone(), self.scratch_header, self.scratch_key.clone(), self.theme.clone(), Some(self.border), self.window_anim, self.work_anim, self.blur);
                                    }
                                    self.unsaved = false;
                                } else if key_code == KeyCode::Enter { // if the enter key is pressed, interact with certain widgets
                                    match self.current_page {
                                        Page::Main => {
                                            if self.index == 0 { // if theme selector block is marked
                                                self.theme = match self.theme {
                                                    OurTheme::Light => OurTheme::Dark,
                                                    OurTheme::Dark => OurTheme::Custom,
                                                    OurTheme::Custom => OurTheme::Light,
                                                };
                                                self.unsaved = true;
                                            }
                                        }
                                        Page::Bind => { // set the captures if needed
                                            if self.index == 2 {
                                                self.capturenext = Some(CaptureInput::ExitKey);
                                            } else if self.index == 3 {
                                                self.capturenext = Some(CaptureInput::LaunchKey);
                                            } else if self.index == 4 {
                                                self.capturenext = Some(CaptureInput::KillKey);
                                            } else if self.index == 5 {
                                                self.capturenext = Some(CaptureInput::MiniKey);
                                            } else if self.index == 6 {
                                                self.capturenext = Some(CaptureInput::ScratchKey);
                                            }
                                        }
                                        Page::Bar => {
                                            if self.index >= 5 {
                                                match self.next_widget {
                                                    Some(x) => {
                                                        if self.index == 5 {
                                                            self.bar_left.push(x);
                                                        } else if self.index == 6 {
                                                            self.bar_center.push(x);
                                                        } else if self.index == 7 {
                                                            self.bar_right.push(x);
                                                        }
                                                    }
                                                    None => {}
                                                }
                                                println!("{:?}", self.bar_left);
                                                println!("{}", self.bar_center.len());
                                                self.next_widget = None;
                                                self.unsaved = true;
                                            }
                                        }
                                        Page::Init => {

                                        }
                                        Page::Anim => {//toggle blur if relevant
                                            if self.index == 5 {
                                                self.blur = !self.blur;
                                                self.unsaved = true;
                                            }
                                        }
                                    }
                                } else if key_code == KeyCode::Key1 {//dropdown management with number keys
                                    if self.current_page == Page::Main {
                                        if self.index == 1 {
                                            self.primary_key = Some(ShortcutKey::Super);
                                            self.unsaved = true;
                                        } else if self.index == 2 {
                                            self.secondary_key = Some(ShortcutKey::Super);
                                            self.unsaved = true;
                                        }
                                    } else if self.current_page == Page::Bind {
                                        if self.index == 0 {
                                            self.primary_key = Some(ShortcutKey::Super);
                                            self.unsaved = true;
                                        } else if self.index == 1 {
                                            self.secondary_key = Some(ShortcutKey::Super);
                                            self.unsaved = true;
                                        } else if self.index == 2 {
                                            self.exit_header = Some(BindKey::PrimaryKey);
                                            self.unsaved = true;
                                        } else if self.index == 3 {
                                            self.launch_header = Some(BindKey::PrimaryKey);
                                            self.unsaved = true;
                                        } else if self.index == 4 {
                                            self.kill_header = Some(BindKey::PrimaryKey);
                                            self.unsaved = true;
                                        } else if self.index == 5 {
                                            self.minimize_header = Some(BindKey::PrimaryKey);
                                            self.unsaved = true;
                                        } else if self.index == 6 {
                                            self.scratch_header = Some(BindKey::PrimaryKey);
                                            self.unsaved = true;
                                        }
                                    } else if self.current_page == Page::Anim {
                                        if self.index == 3 {
                                            self.window_anim = Some(WindowAnimation::None);
                                            self.unsaved = true;
                                        } else if self.index == 4 {
                                            self.work_anim = Some(WorkAnimation::None);
                                            self.unsaved = true;
                                        }
                                    } else if self.current_page == Page::Bar {
                                        if self.index == 0 {
                                            self.next_widget = Some(BarWidget::Audio);
                                        } else if self.index == 1 {
                                            self.next_widget = Some(BarWidget::Bluetooth);
                                        } else if self.index == 2 {
                                            self.next_widget = Some(BarWidget::Disk);
                                        } else if self.index == 3 {
                                            self.next_widget = Some(BarWidget::RAM);
                                        } else if self.index == 4 {
                                            self.next_widget = Some(BarWidget::Tray);
                                        }
                                    }
                                } else if key_code == KeyCode::Key2 {
                                    if self.current_page == Page::Main {
                                        if self.index == 1 {
                                            self.primary_key = Some(ShortcutKey::Alt);
                                            self.unsaved = true;
                                        } else if self.index == 2 {
                                            self.secondary_key = Some(ShortcutKey::Alt);
                                            self.unsaved = true;
                                        }
                                    } else if self.current_page == Page::Bind {
                                        if self.index == 0 {
                                            self.primary_key = Some(ShortcutKey::Alt);
                                            self.unsaved = true;
                                        } else if self.index == 1 {
                                            self.secondary_key = Some(ShortcutKey::Alt);
                                            self.unsaved = true;
                                        } else if self.index == 2 {
                                            self.exit_header = Some(BindKey::SecondaryKey);
                                            self.unsaved = true;
                                        } else if self.index == 3 {
                                            self.launch_header = Some(BindKey::SecondaryKey);
                                            self.unsaved = true;
                                        } else if self.index == 4 {
                                            self.kill_header = Some(BindKey::SecondaryKey);
                                            self.unsaved = true;
                                        } else if self.index == 5 {
                                            self.minimize_header = Some(BindKey::SecondaryKey);
                                            self.unsaved = true;
                                        } else if self.index == 6 {
                                            self.scratch_header = Some(BindKey::SecondaryKey);
                                            self.unsaved = true;
                                        }
                                    } else if self.current_page == Page::Anim {
                                        if self.index == 3 {
                                            self.window_anim = Some(WindowAnimation::Popin);
                                            self.unsaved = true;
                                        } else if self.index == 4 {
                                            self.work_anim = Some(WorkAnimation::Slide);
                                            self.unsaved = true;
                                        }
                                    } else if self.current_page == Page::Bar {
                                        if self.index == 0 {
                                            self.next_widget = Some(BarWidget::Backlight);
                                        } else if self.index == 1 {
                                            self.next_widget = Some(BarWidget::CPU);
                                        } else if self.index == 2 {
                                            self.next_widget = Some(BarWidget::KeyboardState);
                                        } else if self.index == 3 {
                                            self.next_widget = Some(BarWidget::Taskbar);
                                        } else if self.index == 4 {
                                            self.next_widget = Some(BarWidget::User);
                                        }
                                    }
                                } else if key_code == KeyCode::Key3 {
                                    if self.current_page == Page::Main {
                                        if self.index == 1 {
                                            self.primary_key = Some(ShortcutKey::Shift);
                                            self.unsaved = true;
                                        } else if self.index == 2 {
                                            self.secondary_key = Some(ShortcutKey::Shift);
                                            self.unsaved = true;
                                        }
                                    } else if self.current_page == Page::Bind {
                                        if self.index == 0 {
                                            self.primary_key = Some(ShortcutKey::Shift);
                                            self.unsaved = true;
                                        } else if self.index == 1 {
                                            self.secondary_key = Some(ShortcutKey::Shift);
                                            self.unsaved = true;
                                        } else if self.index == 2 {
                                            self.exit_header = Some(BindKey::BothKey);
                                            self.unsaved = true;
                                        } else if self.index == 3 {
                                            self.launch_header = Some(BindKey::BothKey);
                                            self.unsaved = true;
                                        } else if self.index == 4 {
                                            self.kill_header = Some(BindKey::BothKey);
                                            self.unsaved = true;
                                        } else if self.index == 5 {
                                            self.minimize_header = Some(BindKey::BothKey);
                                            self.unsaved = true;
                                        } else if self.index == 6 {
                                            self.scratch_header = Some(BindKey::BothKey);
                                            self.unsaved = true;
                                        }
                                    } else if self.current_page == Page::Anim {
                                        if self.index == 3 {
                                            self.window_anim = Some(WindowAnimation::Slide);
                                            self.unsaved = true;
                                        } else if self.index == 4 {
                                            self.work_anim = Some(WorkAnimation::SlideVert);
                                            self.unsaved = true;
                                        }
                                    } else if self.current_page == Page::Bar {
                                        if self.index == 0 {
                                            self.next_widget = Some(BarWidget::Battery);
                                        } else if self.index == 1 {
                                            self.next_widget = Some(BarWidget::Clock);
                                        } else if self.index == 2 {
                                            self.next_widget = Some(BarWidget::Network);
                                        } else if self.index == 3 {
                                            self.next_widget = Some(BarWidget::Temperature);
                                        } else if self.index == 4 {
                                            self.next_widget = Some(BarWidget::Workspaces);
                                        }
                                    }
                                } else if key_code == KeyCode::Key4 {
                                    if self.current_page == Page::Main {
                                        if self.index == 1 {
                                            self.primary_key = Some(ShortcutKey::Ctrl);
                                            self.unsaved = true;
                                        } else if self.index == 2 {
                                            self.secondary_key = Some(ShortcutKey::Ctrl);
                                            self.unsaved = true;
                                        }
                                    } else if self.current_page == Page::Bind {
                                        if self.index == 0 {
                                            self.primary_key = Some(ShortcutKey::Ctrl);
                                            self.unsaved = true;
                                        } else if self.index == 1 {
                                            self.secondary_key = Some(ShortcutKey::Ctrl);
                                            self.unsaved = true;
                                        }
                                    } else if self.current_page == Page::Anim {
                                        if self.index == 4 {
                                            self.work_anim = Some(WorkAnimation::Fade);
                                            self.unsaved = true;
                                        }
                                    }
                                } else if key_code == KeyCode::Right {//increment values with right presses
                                    if self.current_page == Page::Anim {
                                        if self.index == 0 {
                                            self.border.width = self.border.width + 1;
                                            self.unsaved = true;
                                        } else if self.index == 1 {
                                            self.border.gaps = self.border.gaps + 1;
                                            self.unsaved = true;
                                        } else if self.index == 2 {
                                            self.border.radius = self.border.radius + 1;
                                            self.unsaved = true;
                                        }
                                    }
                                } else if key_code == KeyCode::Left { // decrement values with left presses
                                    if self.current_page == Page::Anim {
                                        if self.index == 0 && self.border.width > 0 {
                                            self.border.width = self.border.width - 1;
                                            self.unsaved = true;
                                        } else if self.index == 1 && self.border.gaps > 0 {
                                            self.border.gaps = self.border.gaps - 1;
                                            self.unsaved = true;
                                        } else if self.index == 2 && self.border.radius > 0 {
                                            self.border.radius = self.border.radius - 1;
                                            self.unsaved = true;
                                        }
                                    }
                                } else if key_code == KeyCode::Backspace {
                                    if self.current_page == Page::Bar {
                                        if self.index >= 5 {
                                            let left = self.bar_left.len();
                                            let right = self.bar_right.len();
                                            let center = self.bar_center.len();
                                            if self.index == 5 && left > 0{
                                                let val = left - 1;
                                                self.bar_left.remove(val);
                                            } else if self.index == 6 && center > 0 {
                                                let val = center - 1;
                                                self.bar_center.remove(val);
                                            } else if self.index == 7 && right > 0 {
                                                let val = right - 1;
                                                self.bar_right.remove(val);
                                            }
                                            self.unsaved = true;
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
            Message::Incr(x) => {
                match x {
                    IncrVal::WidthVal => self.border.width = self.border.width + 1,
                    IncrVal::RadiusVal => self.border.radius = self.border.radius + 1,
                    IncrVal::GapsVal => self.border.gaps = self.border.gaps + 1,
                }
                self.unsaved = true;
                iced::Command::none()
            }
            Message::Decr(x) => {
                match x {
                    IncrVal::WidthVal => if self.border.width > 0 {self.border.width = self.border.width - 1},
                    IncrVal::RadiusVal => if self.border.radius > 0 {self.border.radius = self.border.radius - 1},
                    IncrVal::GapsVal => if self.border.gaps > 0 {self.border.gaps = self.border.gaps - 1},
                }
                self.unsaved = true;
                iced::Command::none()
            }
            Message::ChangeWindowAnim(x) => {
                self.window_anim = Some(x);
                self.unsaved = true;
                iced::Command::none()
            }
            Message::ChangeWorkAnim(x) => {
                self.work_anim = Some(x);
                self.unsaved = true;
                iced::Command::none()
            }
            Message::BlurToggled(x) => {
                self.blur = x;
                self.unsaved = true;
                iced::Command::none()
            }
            Message::WindowUpdate(x) => {
                match x {
                    iced::window::Event::Moved { x: _, y: _ } => {

                    },
                    iced::window::Event::Resized { width, height: _ } => {
                        if width > 800 {
                            self.width = ShrinkValue::Full;
                        } else if width < 250 {
                            self.width = ShrinkValue::Tiny;
                        } else {
                            self.width = ShrinkValue::Medium;
                        }
                    },
                    iced::window::Event::RedrawRequested(_) => {

                    },
                    iced::window::Event::CloseRequested => {

                    },
                    iced::window::Event::Focused => {

                    },
                    iced::window::Event::Unfocused => {

                    },
                    iced::window::Event::FileHovered(_) => {

                    },
                    iced::window::Event::FileDropped(_) => {

                    },
                    iced::window::Event::FilesHoveredLeft => {

                    },
                }
                iced::Command::none()
            }
            Message::PushWidget(bank) => {
                match self.next_widget {
                    Some(x) => {
                        match bank {
                            WidgetBank::Left => {
                                self.bar_left.push(x);
                            }
                            WidgetBank::Center => {
                                self.bar_center.push(x);
                            }
                            WidgetBank::Right => {
                                self.bar_right.push(x);
                            }
                        }
                        self.unsaved = true;
                    }
                    None => {}
                };
                println!("{:?}", self.bar_left);
                println!("{}", self.bar_center.len());
                self.next_widget = None;
                iced::Command::none()
            }
            Message::RemoveWidget(bank) => {
                let left = self.bar_left.len();
                let right = self.bar_right.len();
                let center = self.bar_center.len();
                let pulled = match bank {
                    WidgetBank::Left => {
                        let val;
                        if left > 0 {
                            val = left - 1;
                            Some(self.bar_left.remove(val))
                        } else {
                            None
                        }
                    },
                    WidgetBank::Center => {
                        let val;
                        if center > 0 {
                            val = center - 1;
                            Some(self.bar_center.remove(val))
                        } else {
                            None
                        }
                    },
                    WidgetBank::Right => {
                        let val;
                        if right > 0 {
                            val = right - 1;
                            Some(self.bar_right.remove(val))
                        } else {
                            None
                        }
                    },
                };
                self.unsaved = true;
                println!("{:?}", pulled);
                iced::Command::none()
            }
            Message::AwaitDestination(x) => {
                self.next_widget = Some(x);
                iced::Command::none()
            }

        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {

        //define button styles
        let style = match self.theme {
            OurTheme::Light => self.theme_set.light.clone(),
            OurTheme::Dark => self.theme_set.dark.clone(),
            OurTheme::Custom => self.theme_set.custom.clone(),
        };

        let maintxt = Text::new(Page::Main.to_string());
        let bindtxt = Text::new(Page::Bind.to_string());
        let bartxt = Text::new(Page::Bar.to_string());
        let inittxt = Text::new(Page::Init.to_string());
        let animtxt = Text::new(Page::Anim.to_string());
        let pagemain = Button::new(maintxt)
            .on_press(Message::PageChanged(Page::Main))
            .width(150)
            .style(style.sidebar.mk_theme());
        let pagebind = Button::new(bindtxt)
            .on_press(Message::PageChanged(Page::Bind))
            .width(150)
            .style(style.sidebar.mk_theme());
        let pagebar = Button::new(bartxt)
            .on_press(Message::PageChanged(Page::Bar))
            .width(150)
            .style(style.sidebar.mk_theme());
        let pageinit = Button::new(inittxt)
            .on_press(Message::PageChanged(Page::Init))
            .width(150)
            .style(style.sidebar.mk_theme());
        let pageanim = Button::new(animtxt)
            .on_press(Message::PageChanged(Page::Anim))
            .width(150)
            .style(style.sidebar.mk_theme());
        let pagelabel = Text::new(gettext("Available Pages"));
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
        let savetxt = Text::new(gettext("Save"));
        let savedtxt = Text::new(gettext("Saved!"));
        if self.unsaved {
            save = Button::new(savetxt)
            .on_press(Message::Save);
        } else {
            save = Button::new(savedtxt)
            .on_press(Message::Save)
            .style(style.secondary.mk_theme());
        }
        let saverow = Row::new()
            .push(save)
            .align_items(Alignment::Center);

        
        let mut settings = Column::new().spacing(10);


        match self.current_page {
            Page::Main => {
                settings = self.main_page(style);
            }
            Page::Bind => {
                settings = self.bind_page(style);
            }
            Page::Bar => {
                settings = self.bar_page(style);
            }
            Page::Init => {
                
            }
            Page::Anim => {
                settings = self.anim_page(style);
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
                } else if let iced::Event::Window(window_event) = event{
                    Some(Message::WindowUpdate(window_event))
                } else {
                    None
                }
            }
        )
    }
    fn theme(&self) -> Theme {
        let colors = match self.theme {
            OurTheme::Light => self.theme_set.light.application,
            OurTheme::Dark => self.theme_set.dark.application,
            OurTheme::Custom => self.theme_set.custom.application,
        };
        let cust = Theme::Custom(std::boxed::Box::new(iced::theme::Custom::new(colors)));
        cust
    }
}
