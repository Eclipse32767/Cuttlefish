use iced::theme::Theme;
use iced::{Result, Application, Settings, Alignment, Length, executor};
use iced::widget::{Button, Row, Column, Container, Text, Scrollable, Rule};
use iced::Color;
use iced_style::theme;
use libcfg::{getcfgdata, BindKey, ShortcutKey, OurTheme, BarWidget, WindowAnimation, WorkAnimation, Border, decodeheader, decodepri, decodetheme, decodewinanim, decodeworkanim, decodeblur, decodewidget};
mod libcfg;
use libstyle::{ButtonStyle, ListStyle, MenuStyle, ThemeCustom, make_custom_theme, ThemeSet};
mod libstyle;
use gettextrs::*;
use gettextrs::gettext as tr;
mod cuttlefish_pages;
mod kbparser;
mod cuttlefish_save_helper;


//This is Cuttlefish, Our Configuration Tool

const SIDEBAR_WIDTH: u16 = 175;
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
    NoKey,//TODO: REMOVE THIS IT'S STUPID
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
        let mut leftwidgets = vec![];
        let mut centerwidgets = vec![];
        let mut rightwidgets = vec![];
        for i in 0..data.widgetsleft.len() {
            leftwidgets.push(decodewidget(&data.widgetsleft[i], BarWidget::Clock))
        }
        for i in 0..data.widgetscenter.len() {
            centerwidgets.push(decodewidget(&data.widgetscenter[i], BarWidget::Clock))
        }
        for i in 0..data.widgetsright.len() {
            rightwidgets.push(decodewidget(&data.widgetsright[i], BarWidget::Clock))
        }
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
            indexmax: 3,
            border: data.border.clone(),
            window_anim: decodewinanim(&data.winanim, WindowAnimation::None),
            work_anim: decodeworkanim(&data.workanim, WorkAnimation::None),
            blur: decodeblur(&data.blur),
            theme_set: ThemeSet {
                light: ThemeCustom {
                    application: iced::theme::Palette {
                        background: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                        text: Color::from_rgb8(0x00, 0x19, 0x36),
                        primary: Color::from_rgb8(0x00, 0x77, 0xFF),
                        success: Color::from_rgb8(0x00, 0xCB, 0x40),
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
                        border_radius: 5.0,
                        border_width: 2.0,
                        border_color: Color::from_rgb8( 0x00, 0x19, 0x36),
                        menu: MenuStyle {
                            txt_color: Color::from_rgb8( 0x00, 0x19, 0x36),
                            bg_color: Color::from_rgb8(0xE0, 0xF5, 0xFF),
                            border_radius: 5.0,
                            border_width: 2.0,
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
                        primary: Color::from_rgb8(0x00, 0xAB, 0xE1),
                        success: Color::from_rgb8(0x00, 0xA9, 0x35),
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
            bar_left: leftwidgets,
            bar_center: centerwidgets,
            bar_right: rightwidgets,
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
    RemoveWidget(WidgetBank),
    NoOp,
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
                Page::Main => tr("Main Page"),
                Page::Bind => tr("Keybindings Page"),
                Page::Bar => tr("Status Bar Page"),
                Page::Init => tr("Autostart Page"),
                Page::Anim => tr("Animations Page"),
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
        format!("{}{}", tr("Cuttlefish Configurator--"), self.current_page.to_string())
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<Message> { //update function, parses messages
        match message {
            Message::Save => {
                if self.unsaved {
                    self.mkconfig();
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
                        self.indexmax = 3;
                    }
                    Page::Bind => {
                        self.indexmax = 7;
                    }
                    Page::Bar => {
                        self.indexmax = 5;
                    }
                    Page::Init => {
                        self.indexmax = 1;
                    }
                    Page::Anim => {
                        self.indexmax = 8;
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
                self.kbparse(x);
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
            Message::NoOp => {
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
        let mut pagemain = Button::new(maintxt)
            .on_press(Message::PageChanged(Page::Main))
            .width(SIDEBAR_WIDTH)
            .style(style.sidebar.mk_theme());
        let mut pagebind = Button::new(bindtxt)
            .on_press(Message::PageChanged(Page::Bind))
            .width(SIDEBAR_WIDTH)
            .style(style.sidebar.mk_theme());
        let mut pagebar = Button::new(bartxt)
            .on_press(Message::PageChanged(Page::Bar))
            .width(SIDEBAR_WIDTH)
            .style(style.sidebar.mk_theme());
        let mut pageinit = Button::new(inittxt)
            .on_press(Message::PageChanged(Page::Init))
            .width(SIDEBAR_WIDTH)
            .style(style.sidebar.mk_theme());
        let mut pageanim = Button::new(animtxt)
            .on_press(Message::PageChanged(Page::Anim))
            .width(SIDEBAR_WIDTH)
            .style(style.sidebar.mk_theme());
        let pagecap = Button::new("").width(SIDEBAR_WIDTH).height(10000).style(style.sidebar.mk_theme()).on_press(Message::NoOp);
        let pagelabel = Text::new(tr("Available Pages"));
        match self.current_page {
            Page::Main => pagemain = pagemain.style(style.secondary.mk_theme()),
            Page::Bind => pagebind = pagebind.style(style.secondary.mk_theme()),
            Page::Bar => pagebar = pagebar.style(style.secondary.mk_theme()),
            Page::Init => pageinit = pageinit.style(style.secondary.mk_theme()),
            Page::Anim => pageanim = pageanim.style(style.secondary.mk_theme()),
        }
        let pagecol = Column::new()
            .push(pagelabel)
            .push(pagemain)
            .push(pagebind)
            .push(pageanim)
            .push(pagebar)
            .push(pageinit)
            .push(pagecap)
            .align_items(Alignment::Start);

        let savetxt = Text::new(tr("Save"));
        let savedtxt = Text::new(tr("Saved!"));
        let save = match (self.unsaved, self.index == self.indexmax) {
            (true, true) => {
                Button::new(savetxt).on_press(Message::Save).style(theme::Button::Positive)
            }
            (true, false) => {
                Button::new(savetxt).on_press(Message::Save)
            }
            (false, true) => {
                Button::new(savedtxt).on_press(Message::Save).style(theme::Button::Positive)
            }
            (false, false) => {
                Button::new(savedtxt).on_press(Message::Save).style(style.secondary.mk_theme())
            }
        };
        /* 
        if self.unsaved {
            save = Button::new(savetxt)
            .on_press(Message::Save);
        } else {
            save = Button::new(savedtxt)
            .on_press(Message::Save)
            .style(style.secondary.mk_theme());
        }
        */
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
        let testrule = Rule::vertical(1);
        let scroll = Scrollable::new(settings);
        let col = Column::new()
            .push(scroll)
            .push(saverow)
            .width(Length::Fill)
            .align_items(Alignment::Start)
            .spacing(10);
        let master = Row::new()
            .push(pagecol)
            .push(testrule)
            .push(col);
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
