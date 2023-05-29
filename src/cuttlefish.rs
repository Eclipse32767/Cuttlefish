use iced::theme::{self, Theme};
use iced::{Result, Application, Settings, Alignment, Length, executor};
use iced::widget::{Button, Row, Column, Container, pick_list, Text, Scrollable};
use iced::keyboard::KeyCode;
use iced_style::Color;
use libcfg::{getcfgdata, BindKey, ShortcutKey, OurTheme, BarWidget, WindowAnimation, WorkAnimation, Border, decodeheader, decodepri, decodetheme, mkwmcfg, mkselfcfg, decodewinanim, decodeworkanim, decodeblur};
mod libcfg;
use langcfg::{get_lang, Translation};
mod langcfg;
use libstyle::{ButtonStyle, ListStyle, MenuStyle, ThemeCustom, make_custom_theme};
mod libstyle;
mod liblocale;


//This is Cuttlefish, Our Configuration Tool

fn main() -> Result {
    Configurator::run(Settings::default())
}


struct Configurator { //The basic configurator struct, contains most program state
    theme: OurTheme,
    locale: Translation,
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
    cust_theme: ThemeCustom,
    width: ShrinkValue,
    bar_left: Vec<BarWidget>,
    bar_center: Vec<BarWidget>,
    bar_right: Vec<BarWidget>,
    next_widget: BarWidget,
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
            locale: get_lang(),
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
            cust_theme: make_custom_theme(),
            width: ShrinkValue::Full,
            bar_left: vec![],
            bar_center: vec![],
            bar_right: vec![],
            next_widget: BarWidget::None,
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
        let locale = get_lang();
        let pretty = locale.prettyprint.unwrap();
        write!(
            f,
            "{}",
            match self { //respect locale preferences when prettyprinting
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
    fn new(_flags: ()) -> (Self, iced::Command<Message>) { //code that initializes the app
        (
            Self::default(),
            iced::Command::none()
        )
    }
    fn title(&self) -> String { //code that sets the app title
        let globalstr = self.locale.global.as_ref().unwrap();
        let title = globalstr.title.clone();
        format!("{title}{}", self.current_page.to_string())
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
                                                if self.next_widget != BarWidget::None {
                                                    if self.index == 5 {
                                                        self.bar_left.push(self.next_widget);
                                                    } else if self.index == 6 {
                                                        self.bar_center.push(self.next_widget);
                                                    } else if self.index == 7 {
                                                        self.bar_right.push(self.next_widget);
                                                    }
                                                };
                                                println!("{:?}", self.bar_left);
                                                println!("{}", self.bar_center.len());
                                                self.next_widget = BarWidget::None;
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
                                            self.next_widget = BarWidget::Audio;
                                        } else if self.index == 1 {
                                            self.next_widget = BarWidget::Bluetooth;
                                        } else if self.index == 2 {
                                            self.next_widget = BarWidget::Disk;
                                        } else if self.index == 3 {
                                            self.next_widget = BarWidget::RAM;
                                        } else if self.index == 4 {
                                            self.next_widget = BarWidget::Tray;
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
                                            self.next_widget = BarWidget::Backlight;
                                        } else if self.index == 1 {
                                            self.next_widget = BarWidget::CPU;
                                        } else if self.index == 2 {
                                            self.next_widget = BarWidget::KeyboardState;
                                        } else if self.index == 3 {
                                            self.next_widget = BarWidget::Taskbar;
                                        } else if self.index == 4 {
                                            self.next_widget = BarWidget::User;
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
                                            self.next_widget = BarWidget::Battery;
                                        } else if self.index == 1 {
                                            self.next_widget = BarWidget::Clock;
                                        } else if self.index == 2 {
                                            self.next_widget = BarWidget::Network;
                                        } else if self.index == 3 {
                                            self.next_widget = BarWidget::Temperature;
                                        } else if self.index == 4 {
                                            self.next_widget = BarWidget::Workspaces;
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
                if self.next_widget != BarWidget::None {
                    match bank {
                        WidgetBank::Left => {
                            self.bar_left.push(self.next_widget);
                        }
                        WidgetBank::Center => {
                            self.bar_center.push(self.next_widget);
                        }
                        WidgetBank::Right => {
                            self.bar_right.push(self.next_widget);
                        }
                    }
                    self.unsaved = true;
                };
                println!("{:?}", self.bar_left);
                println!("{}", self.bar_center.len());
                self.next_widget = BarWidget::None;
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
                            self.bar_left.remove(val)
                        } else {
                            BarWidget::None
                        }
                    },
                    WidgetBank::Center => {
                        let val;
                        if center > 0 {
                            val = center - 1;
                            self.bar_center.remove(val)
                        } else {
                            BarWidget::None
                        }
                    },
                    WidgetBank::Right => {
                        let val;
                        if right > 0 {
                            val = right - 1;
                            self.bar_right.remove(val)
                        } else {
                            BarWidget::None
                        }
                    },
                };
                self.unsaved = true;
                println!("{:?}", pulled);
                iced::Command::none()
            }
            Message::AwaitDestination(x) => {
                self.next_widget = x;
                iced::Command::none()
            }

        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {

        //define button styles
        let sidebar_active_btn = match self.theme {
            OurTheme::Light => ButtonStyle{
                border_radius: 10.0,
                txt_color: Color::from_rgb8( 0x00, 0x19, 0x36),
                bg_color: Color::from_rgb8(0xD2, 0xF0, 0xFF),
                border_color: Color::from_rgb8(0, 0, 0),
                border_width: 0.0,
                shadow_offset: iced::Vector {x: 0.0, y: 0.0}
            },
            OurTheme::Dark => ButtonStyle{
                border_radius: 10.0,
                txt_color: Color::from_rgb8( 0xD2, 0xF0, 0xFF),
                bg_color: Color::from_rgb8(0x00, 0x20, 0x46),
                border_color: Color::from_rgb8(0, 0, 0),
                border_width: 0.0,
                shadow_offset: iced::Vector {x: 0.0, y: 0.0}
            },
            OurTheme::Custom => self.cust_theme.sidebar.active.clone()
        };
        let sidebar_inactive_btn = match self.theme {
            OurTheme::Custom => self.cust_theme.sidebar.inactive.clone(),
            OurTheme::Light => ButtonStyle{
                border_radius: 10.0,
                txt_color: Color::from_rgb8(0x00, 0x19, 0x36),
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
        let body_active_btn = match self.theme {
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
        let body_inactive_btn = match self.theme {
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

        let picklist_style = match self.theme {
            OurTheme::Custom => self.cust_theme.list.clone(),
            OurTheme::Light => ListStyle {
                txt_color: Color::from_rgb8(0x00, 0x20, 0x46),
                bg_color: Color::from_rgb8(0xC6, 0xEC, 0xFF),
                handle_color: Color::from_rgb8(0x00, 0x20, 0x46),
                border_radius: 10.0,
                border_width: 2.0,
                border_color: Color::from_rgb8(0x00, 0x20, 0x46)
            },
            OurTheme::Dark => ListStyle {
                txt_color: Color::from_rgb8(0xD2, 0xF0, 0xFF),
                bg_color: Color::from_rgb8(0x00, 0x29, 0x58),
                handle_color: Color::from_rgb8(0xD2, 0xF0, 0xFF),
                border_radius: 10.0,
                border_width: 2.0,
                border_color: Color::from_rgb8(0xD2, 0xF0, 0xFF)
            },
        };
        let menu_style = match self.theme {
            OurTheme::Custom => self.cust_theme.menu.clone(),
            OurTheme::Light => MenuStyle {
                txt_color: Color::from_rgb8(0x00, 0x20, 0x46),
                bg_color: Color::from_rgb8(0xC6, 0xEC, 0xFF),
                border_width: 2.0,
                border_radius: 10.0,
                border_color: Color::from_rgb8(0x00, 0x20, 0x46),
                sel_txt_color: Color::from_rgb8(0x00, 0x20, 0x46),
                sel_bg_color: Color::from_rgb8(0x00, 0xF1, 0xD6),
            },
            OurTheme::Dark => MenuStyle {
                txt_color: Color::from_rgb8(0xD2, 0xF0, 0xFF),
                bg_color: Color::from_rgb8(0x00, 0x29, 0x58),
                border_width: 2.0,
                border_radius: 10.0,
                border_color: Color::from_rgb8(0xD2, 0xF0, 0xFF),
                sel_txt_color: Color::from_rgb8(0xD2, 0xF0, 0xFF),
                sel_bg_color: Color::from_rgb8(0x00, 0xCD, 0xB6),
            }
        };

        let selectionmarker: Text = Text::new("=>");
        let globalstr = self.locale.global.as_ref().unwrap();
        let mainstr = self.locale.mainpage.as_ref().unwrap();
        let bindstr = self.locale.bindpage.as_ref().unwrap();
        let animstr = self.locale.animpage.as_ref().unwrap();

        let maintxt = String::as_str(&globalstr.main);
        let bindtxt = String::as_str(&globalstr.bind);
        let bartxt = String::as_str(&globalstr.bar);
        let inittxt = String::as_str(&globalstr.init);
        let animtxt = String::as_str(&globalstr.anim);
        let pagetxt = String::as_str(&globalstr.label);
        let mut pagemain = Button::new(maintxt)
            .on_press(Message::PageChanged(Page::Main))
            .width(150)
            .style(theme::Button::Custom(std::boxed::Box::new(sidebar_active_btn.clone())));
        let mut pagebind = Button::new(bindtxt)
            .on_press(Message::PageChanged(Page::Bind))
            .width(150)
            .style(theme::Button::Custom(std::boxed::Box::new(sidebar_active_btn.clone())));
        let mut pagebar = Button::new(bartxt)
            .on_press(Message::PageChanged(Page::Bar))
            .width(150)
            .style(theme::Button::Custom(std::boxed::Box::new(sidebar_active_btn.clone())));
        let mut pageinit = Button::new(inittxt)
            .on_press(Message::PageChanged(Page::Init))
            .width(150)
            .style(theme::Button::Custom(std::boxed::Box::new(sidebar_active_btn.clone())));
        let mut pageanim = Button::new(animtxt)
            .on_press(Message::PageChanged(Page::Anim))
            .width(150)
            .style(theme::Button::Custom(std::boxed::Box::new(sidebar_active_btn.clone())));
        let pagelabel = Text::new(pagetxt);
        match self.current_page {
            Page::Main => {
                pagemain = pagemain.style(theme::Button::Custom(std::boxed::Box::new(sidebar_inactive_btn.clone())));
            }
            Page::Bind => {
                pagebind = pagebind.style(theme::Button::Custom(std::boxed::Box::new(sidebar_inactive_btn.clone())));
            }
            Page::Bar => {
                pagebar = pagebar.style(theme::Button::Custom(std::boxed::Box::new(sidebar_inactive_btn.clone())));
            }
            Page::Init => {
                pageinit = pageinit.style(theme::Button::Custom(std::boxed::Box::new(sidebar_inactive_btn.clone())));
            }
            Page::Anim => {
                pageanim = pageanim.style(theme::Button::Custom(std::boxed::Box::new(sidebar_inactive_btn.clone())));
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
            .on_press(Message::Save)
            .style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
        } else {
            save = Button::new(savedtxt)
            .on_press(Message::Save)
            .style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone())));
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
                    .placeholder("choose")
                    .style(theme::PickList::Custom(std::rc::Rc::new(picklist_style.clone()),std::rc::Rc::new(menu_style.clone())));
                let secondarypick = pick_list(
                    &ShortcutKey::ALL[..], 
                    self.secondary_key, 
                    Message::SecondaryKeyChanged,
                    )
                    .placeholder("choose")
                    .style(theme::PickList::Custom(std::rc::Rc::new(picklist_style.clone()),std::rc::Rc::new(menu_style.clone())));
                let primarytxt;
                let temp_primary = format!("{}{}", globalstr.primary, globalstr.primary_addendum);
                let secondarytxt;
                let temp_secondary = format!("{}{}", globalstr.secondary, globalstr.secondary_addendum);
                if self.width == ShrinkValue::Full {
                    primarytxt = String::as_str(&temp_primary);
                    secondarytxt = String::as_str(&temp_secondary);
                } else {
                    primarytxt = String::as_str(&globalstr.primary);
                    secondarytxt = String::as_str(&globalstr.secondary);
                }
                
                let primarylabel: Text = Text::new(primarytxt.to_owned());
                let secondarylabel: Text = Text::new(secondarytxt.to_owned());

                let lighttxt = String::as_str(&mainstr.light);
                let darktxt = String::as_str(&mainstr.dark);
                let themetxt = String::as_str(&mainstr.theme);
                let customtxt = String::as_str(&mainstr.custom);
                let mut light = Button::new(lighttxt)
                    .on_press(Message::ThemeChanged(OurTheme::Light))
                    .style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut dark = Button::new(darktxt)
                    .on_press(Message::ThemeChanged(OurTheme::Dark))
                    .style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut custom = Button::new(customtxt)
                    .on_press(Message::ThemeChanged(OurTheme::Custom))
                    .style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let themelabel = Text::new(themetxt);
                match self.theme {
                    OurTheme::Light => {
                        light = light.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone())));
                    }
                    OurTheme::Dark => {
                        dark = dark.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone())));
                    }
                    OurTheme::Custom => {
                        custom = custom.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone())));
                    }
                }
                let mut themerow = Row::new().spacing(10);
                let mut primaryrow = Row::new().spacing(10);
                let mut secondaryrow = Row::new().spacing(10);

                if self.index == 0 {
                    themerow = themerow.push(selectionmarker);
                } else if self.index == 1 {
                    primaryrow = primaryrow.push(selectionmarker);
                } else if self.index == 2 {
                    secondaryrow = secondaryrow.push(selectionmarker);
                }
                themerow = themerow
                    .push(themelabel)
                    .push(light)
                    .push(dark)
                    .push(custom);
                primaryrow = primaryrow
                    .push(primarylabel)
                    .push(primarypick);
                secondaryrow = secondaryrow
                    .push(secondarylabel)
                    .push(secondarypick);
        
                settings = settings
                    .push(themerow)
                    .push(primaryrow)
                    .push(secondaryrow);
            }
            Page::Bind => {
                let primarypick = pick_list(
                    &ShortcutKey::ALL[..], 
                    self.primary_key, 
                    Message::PrimaryKeyChanged,
                    )
                    .placeholder("choose")
                    .style(theme::PickList::Custom(std::rc::Rc::new(picklist_style.clone()),std::rc::Rc::new(menu_style.clone())));
                let secondarypick = pick_list(
                    &ShortcutKey::ALL[..], 
                    self.secondary_key, 
                    Message::SecondaryKeyChanged,
                    )
                    .placeholder("choose")
                    .style(theme::PickList::Custom(std::rc::Rc::new(picklist_style.clone()),std::rc::Rc::new(menu_style.clone())));
                let primarytxt;
                let temp_primary = format!("{}{}", globalstr.primary, globalstr.primary_addendum);
                let secondarytxt;
                let temp_secondary = format!("{}{}", globalstr.secondary, globalstr.secondary_addendum);
                if self.width == ShrinkValue::Full {
                    primarytxt = String::as_str(&temp_primary);
                    secondarytxt = String::as_str(&temp_secondary);
                } else {
                    primarytxt = String::as_str(&globalstr.primary);
                    secondarytxt = String::as_str(&globalstr.secondary);
                }
                let primarylabel: Text = Text::new(primarytxt.to_owned());
                let secondarylabel: Text = Text::new(secondarytxt.to_owned());


                let exitsclabel = Text::new(bindstr.exit.clone());
                let exitheaderselect = pick_list(
                &BindKey::ALL[..], 
                self.exit_header, 
                Message::ExitHeaderChanged,
                )
                .placeholder("choose")
                .style(theme::PickList::Custom(std::rc::Rc::new(picklist_style.clone()),std::rc::Rc::new(menu_style.clone())));
                let exitkey = String::as_str(&self.exit_key);
                let mut exitkeyselect = Button::new(exitkey).on_press(Message::Capture(CaptureInput::ExitKey)).width(50).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let launchsclabel: Text = Text::new(bindstr.launch.clone());
                let launchheaderselect = pick_list(
                    &BindKey::ALL[..], 
                    self.launch_header, 
                    Message::LaunchHeaderChanged,
                    )
                    .placeholder("choose")
                    .style(theme::PickList::Custom(std::rc::Rc::new(picklist_style.clone()),std::rc::Rc::new(menu_style.clone())));
                let launchkey = String::as_str(&self.launch_key);
                let mut launchkeyselect = Button::new(launchkey).on_press(Message::Capture(CaptureInput::LaunchKey)).width(50).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let killsclabel: Text = Text::new(bindstr.kill.clone());
                let killheaderselect = pick_list(
                    &BindKey::ALL[..], 
                    self.kill_header, 
                    Message::KillHeaderChanged,
                    )
                    .placeholder("choose")
                    .style(theme::PickList::Custom(std::rc::Rc::new(picklist_style.clone()),std::rc::Rc::new(menu_style.clone())));
                let killkey = String::as_str(&self.kill_key);
                let mut killkeyselect = Button::new(killkey).on_press(Message::Capture(CaptureInput::KillKey)).width(50).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let minisclabel: Text = Text::new(bindstr.mini.clone());
                let miniheaderselect = pick_list(
                 &BindKey::ALL[..], 
                 self.minimize_header, 
                 Message::MiniHeaderChanged,
                 )
                    .placeholder("choose")
                    .style(theme::PickList::Custom(std::rc::Rc::new(picklist_style.clone()),std::rc::Rc::new(menu_style.clone())));
                let minikey = String::as_str(&self.minimize_key);
                let mut minikeyselect = Button::new(minikey).on_press(Message::Capture(CaptureInput::MiniKey)).width(50).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let scratchsclabel: Text = Text::new(bindstr.scratch.clone());
                let scratchheaderselect = pick_list(
                    &BindKey::ALL[..], 
                    self.scratch_header, 
                    Message::ScratchHeaderChanged,
                    )
                    .placeholder("choose")
                    .style(theme::PickList::Custom(std::rc::Rc::new(picklist_style.clone()),std::rc::Rc::new(menu_style.clone())));
                let scratchkey = String::as_str(&self.scratch_key);
                let mut scratchkeyselect = Button::new(scratchkey).on_press(Message::Capture(CaptureInput::ScratchKey)).width(50).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                
                match self.capturenext.as_ref().unwrap() {
                    CaptureInput::NoKey => {
                    }
                    CaptureInput::ExitKey => {
                        exitkeyselect = exitkeyselect.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone())));
                    }
                    CaptureInput::KillKey => {
                        killkeyselect = killkeyselect.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone())));
                    }
                    CaptureInput::LaunchKey => {
                        launchkeyselect = launchkeyselect.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone())));
                    }
                    CaptureInput::MiniKey => {
                        minikeyselect = minikeyselect.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone())));
                    }
                    CaptureInput::ScratchKey => {
                        scratchkeyselect = scratchkeyselect.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone())));
                    }
                }
                let mut primaryrow = Row::new();
                let mut secondaryrow = Row::new();
                let mut exitscrow = Row::new();
                let mut launchscrow = Row::new();
                let mut killscrow = Row::new();
                let mut miniscrow = Row::new();
                let mut scratchscrow = Row::new();
                if self.index == 0 {
                    primaryrow = primaryrow.push(selectionmarker);
                } else if self.index == 1 {
                    secondaryrow = secondaryrow.push(selectionmarker);
                } else if self.index == 2 {
                    exitscrow = exitscrow.push(selectionmarker);
                } else if self.index == 3 {
                    launchscrow = launchscrow.push(selectionmarker);
                } else if self.index == 4 {
                    killscrow = killscrow.push(selectionmarker);
                } else if self.index == 5 {
                    miniscrow = miniscrow.push(selectionmarker);
                } else if self.index == 6 {
                    scratchscrow = scratchscrow.push(selectionmarker);
                }
                primaryrow = primaryrow
                    .push(primarylabel)
                    .push(primarypick)
                    .spacing(10);
                secondaryrow = secondaryrow
                    .push(secondarylabel)
                    .push(secondarypick)
                    .spacing(10);
                exitscrow = exitscrow
                    .push(exitsclabel)
                    .push(exitheaderselect)
                    .push(exitkeyselect)
                    .spacing(10);
                launchscrow = launchscrow
                    .push(launchsclabel)
                    .push(launchheaderselect)
                    .push(launchkeyselect)
                    .spacing(10);
                killscrow = killscrow
                    .push(killsclabel)
                    .push(killheaderselect)
                    .push(killkeyselect)
                    .spacing(10);
                miniscrow = miniscrow
                    .push(minisclabel)
                    .push(miniheaderselect)
                    .push(minikeyselect)
                    .spacing(10);
                scratchscrow = scratchscrow
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
                let mut left_contents = String::from("");
                if self.bar_left.len() > 0 {
                    for i in 0..self.bar_left.len() {
                        left_contents = format!("{left_contents}  {:#?}", self.bar_left[i]);
                    }
                }
                let mut right_contents = String::from("");
                if self.bar_right.len() > 0 {
                    for i in 0..self.bar_right.len() {
                        right_contents = format!("{right_contents}  {:#?}", self.bar_right[i]);
                    }
                }
                let mut center_contents = String::from("");
                if self.bar_center.len() > 0 {
                    for i in 0..self.bar_center.len() {
                        center_contents = format!("{center_contents}  {:#?}", self.bar_center[i]);
                    }
                }
                let barleft = Button::new("left").on_press(Message::PushWidget(WidgetBank::Left)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let barcenter = Button::new("center").on_press(Message::PushWidget(WidgetBank::Center)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let barright = Button::new("right").on_press(Message::PushWidget(WidgetBank::Right)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut audio = Button::new("Audio").on_press(Message::AwaitDestination(BarWidget::Audio)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut backlight = Button::new("Backlight").on_press(Message::AwaitDestination(BarWidget::Backlight)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut battery = Button::new("Battery").on_press(Message::AwaitDestination(BarWidget::Battery)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut bluetooth = Button::new("Bluetooth").on_press(Message::AwaitDestination(BarWidget::Bluetooth)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut cpu = Button::new("CPU").on_press(Message::AwaitDestination(BarWidget::CPU)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut clock = Button::new("Clock").on_press(Message::AwaitDestination(BarWidget::Clock)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut disk = Button::new("Disk").on_press(Message::AwaitDestination(BarWidget::Disk)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut keyboard = Button::new("Keyboard State").on_press(Message::AwaitDestination(BarWidget::KeyboardState)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut network = Button::new("Network").on_press(Message::AwaitDestination(BarWidget::Network)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut ram = Button::new("RAM").on_press(Message::AwaitDestination(BarWidget::RAM)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut taskbar = Button::new("Taskbar").on_press(Message::AwaitDestination(BarWidget::Taskbar)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut temperature = Button::new("Temperature").on_press(Message::AwaitDestination(BarWidget::Temperature)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut tray = Button::new("System Tray").on_press(Message::AwaitDestination(BarWidget::Tray)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut user = Button::new("Current User").on_press(Message::AwaitDestination(BarWidget::User)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut workspaces = Button::new("Workspaces").on_press(Message::AwaitDestination(BarWidget::Workspaces)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let removeleft = Button::new("remove").on_press(Message::RemoveWidget(WidgetBank::Left)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let removecenter = Button::new("remove").on_press(Message::RemoveWidget(WidgetBank::Center)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let removeright = Button::new("remove").on_press(Message::RemoveWidget(WidgetBank::Right)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let labelleft = Text::new(left_contents);
                let labelright = Text::new(right_contents);
                let labelcenter = Text::new(center_contents);

                match self.next_widget {
                    BarWidget::None => {

                    },
                    BarWidget::Audio => audio = audio.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone()))),
                    BarWidget::Backlight => backlight = backlight.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone()))),
                    BarWidget::Battery => battery = battery.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone()))),
                    BarWidget::Bluetooth => bluetooth = bluetooth.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone()))),
                    BarWidget::Clock => clock = clock.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone()))),
                    BarWidget::CPU => cpu = cpu.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone()))),
                    BarWidget::Disk => disk = disk.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone()))),
                    BarWidget::KeyboardState => keyboard = keyboard.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone()))),
                    BarWidget::RAM => ram = ram.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone()))),
                    BarWidget::Network => network = network.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone()))),
                    BarWidget::Temperature => temperature = temperature.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone()))),
                    BarWidget::Tray => tray = tray.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone()))),
                    BarWidget::Taskbar => taskbar = taskbar.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone()))),
                    BarWidget::Workspaces => workspaces = workspaces.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone()))),
                    BarWidget::User => user = user.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone()))),
                }
                
                let mut left_row = Row::new();
                let mut center_row = Row::new();
                let mut right_row = Row::new();
                let mut widget_row_i = Row::new();
                let mut widget_row_ii = Row::new();
                let mut widget_row_iii = Row::new();
                let mut widget_row_iv = Row::new();
                let mut widget_row_v = Row::new();

                if self.index == 0 {
                    widget_row_i = widget_row_i.push(selectionmarker)
                } else if self.index == 1 {
                    widget_row_ii = widget_row_ii.push(selectionmarker)
                } else if self.index == 2 {
                    widget_row_iii = widget_row_iii.push(selectionmarker)
                } else if self.index == 3 {
                    widget_row_iv = widget_row_iv.push(selectionmarker)
                } else if self.index == 4 {
                    widget_row_v = widget_row_v.push(selectionmarker)
                } else if self.index == 5 {
                    left_row = left_row.push(selectionmarker) 
                } else if self.index == 6 {
                    center_row = center_row.push(selectionmarker)
                } else if self.index == 7 {
                    right_row = right_row.push(selectionmarker)
                }
                
                left_row = left_row.push(barleft).push(labelleft).push(removeleft).spacing(10);
                center_row = center_row.push(barcenter).push(labelcenter).push(removecenter).spacing(10);
                right_row = right_row.push(barright).push(labelright).push(removeright).spacing(10);
                widget_row_i = widget_row_i.push(audio).push(backlight).push(battery).spacing(10);
                widget_row_ii = widget_row_ii.push(bluetooth).push(cpu).push(clock).spacing(10);
                widget_row_iii = widget_row_iii.push(disk).push(keyboard).push(network).spacing(10);
                widget_row_iv = widget_row_iv.push(ram).push(taskbar).push(temperature).spacing(10);
                widget_row_v = widget_row_v.push(tray).push(user).push(workspaces).spacing(10);

                settings = settings
                    .push(widget_row_i)
                    .push(widget_row_ii)
                    .push(widget_row_iii)
                    .push(widget_row_iv)
                    .push(widget_row_v)
                    .push(left_row)
                    .push(center_row)
                    .push(right_row);
            }
            Page::Init => {
                
            }
            Page::Anim => {

                let widthincr = Button::new("+").on_press(Message::Incr(IncrVal::WidthVal)).width(30).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut widthdecr = Button::new("-").on_press(Message::Decr(IncrVal::WidthVal)).width(30).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let widthvaluepeek = Text::new(format!("{}", self.border.width));
                let widthlabel = Text::new(animstr.width.clone());

                let mut widthrow = Row::new().spacing(10);

                let gapsincr = Button::new("+").on_press(Message::Incr(IncrVal::GapsVal)).width(30).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut gapsdecr = Button::new("-").on_press(Message::Decr(IncrVal::GapsVal)).width(30).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let gapsvaluepeek = Text::new(format!("{}", self.border.gaps));
                let gapslabel = Text::new(animstr.gaps.clone());

                let mut gapsrow = Row::new().spacing(10);

                let radincr = Button::new("+").on_press(Message::Incr(IncrVal::RadiusVal)).width(30).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut raddecr = Button::new("-").on_press(Message::Decr(IncrVal::RadiusVal)).width(30).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let radvaluepeek = Text::new(format!("{}", self.border.radius));
                let radlabel = Text::new(animstr.radius.clone());

                let mut radrow = Row::new().spacing(10);

                let winpick = pick_list(
                    &WindowAnimation::ALL[..], 
                    self.window_anim, 
                    Message::ChangeWindowAnim,
                    )
                    .placeholder("choose")
                    .style(theme::PickList::Custom(std::rc::Rc::new(picklist_style.clone()),std::rc::Rc::new(menu_style.clone())));
                let winlabel = Text::new(animstr.winanim.clone());

                let mut winrow = Row::new().spacing(10);

                let workpick = pick_list(
                    &WorkAnimation::ALL[..],
                    self.work_anim,
                    Message::ChangeWorkAnim,
                    )
                    .placeholder("choose")
                    .style(theme::PickList::Custom(std::rc::Rc::new(picklist_style.clone()),std::rc::Rc::new(menu_style.clone())));
                let worklabel = Text::new(animstr.workanim.clone());

                let mut workrow = Row::new().spacing(10);

                let enable = String::as_str(&animstr.enableblur);
                let disable = String::as_str(&animstr.disableblur);
                let enabled = String::as_str(&animstr.enabledblur);
                let disabled = String::as_str(&animstr.disabledblur);
                let blurlabel = Text::new(animstr.blur.clone());
                let mut bluron = Button::new(enable).on_press(Message::BlurToggled(true)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                let mut bluroff = Button::new(disable).on_press(Message::BlurToggled(false)).style(theme::Button::Custom(std::boxed::Box::new(body_active_btn.clone())));
                if self.blur {
                    bluron = Button::new(enabled).on_press(Message::BlurToggled(true)).style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone())));
                } else {
                    bluroff = Button::new(disabled).on_press(Message::BlurToggled(false)).style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone())));
                }
                let mut blurrow = Row::new().spacing(10);

                if self.border.width == 0 {
                    widthdecr = widthdecr.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone())));
                }
                if self.border.gaps == 0 {
                    gapsdecr = gapsdecr.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone())));
                }
                if self.border.radius == 0 {
                    raddecr = raddecr.style(theme::Button::Custom(std::boxed::Box::new(body_inactive_btn.clone())));
                }

                if self.index == 0 {
                    widthrow = widthrow.push(selectionmarker);
                } else if self.index == 1 {
                    gapsrow = gapsrow.push(selectionmarker);
                } else if self.index == 2 {
                    radrow = radrow.push(selectionmarker);
                } else if self.index == 3 {
                    winrow = winrow.push(selectionmarker);
                } else if self.index == 4 {
                    workrow = workrow.push(selectionmarker);
                } else if self.index == 5 {
                    blurrow = blurrow.push(selectionmarker);
                }

                widthrow = widthrow
                    .push(widthlabel)
                    .push(widthdecr)
                    .push(widthvaluepeek)
                    .push(widthincr);
                gapsrow = gapsrow
                    .push(gapslabel)
                    .push(gapsdecr)
                    .push(gapsvaluepeek)
                    .push(gapsincr);
                radrow = radrow
                    .push(radlabel)
                    .push(raddecr)
                    .push(radvaluepeek)
                    .push(radincr);
                winrow = winrow
                    .push(winlabel)
                    .push(winpick);
                workrow = workrow
                    .push(worklabel)
                    .push(workpick);
                blurrow = blurrow
                    .push(blurlabel)
                    .push(bluroff)
                    .push(bluron);
                settings = settings
                    .push(widthrow)
                    .push(gapsrow)
                    .push(radrow)
                    .push(winrow)
                    .push(workrow)
                    .push(blurrow);
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
