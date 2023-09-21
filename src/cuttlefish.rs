use iced::theme::Theme;
use iced::{Result, Application, Settings, Alignment, Length, executor};
use iced::widget::{Button, Row, Column, Container, Text, Scrollable, Rule};
use iced::Color;
use iced_style::theme;
use lib_cfg::{get_cfg_data, BindKey, ShortcutKey, OurTheme, BarWidget, WindowAnimation, WorkAnimation, Border, decode_header, decode_pri, decode_theme, decode_win_anim, decode_work_anim, decode_blur, decode_widget};
mod lib_cfg;
use lib_style::{ButtonStyle, ListStyle, MenuStyle, ThemeCustom, make_custom_theme, ThemeSet};
mod lib_style;
use gettextrs::*;
use gettextrs::gettext as tr;
use rfd::FileDialog;

mod cuttlefish_pages;
mod kb_parser;
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
    wallpaper: String,
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
    capture_next: Option<CaptureInput>,
    index: u8,
    index_max: u8,
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
        let data = get_cfg_data();
        let mut left_widgets = vec![];
        let mut center_widgets = vec![];
        let mut right_widgets = vec![];
        for i in 0..data.widgets_left.len() {
            left_widgets.push(decode_widget(&data.widgets_left[i], BarWidget::Clock))
        }
        for i in 0..data.widgets_center.len() {
            center_widgets.push(decode_widget(&data.widgets_center[i], BarWidget::Clock))
        }
        for i in 0..data.widgets_right.len() {
            right_widgets.push(decode_widget(&data.widgets_right[i], BarWidget::Clock))
        }
        Configurator { //here we extract all of the data from the config file
            theme: decode_theme(&data.theme, OurTheme::Light),
            current_page: Page::Main,
            wallpaper: data.wallpaper,
            primary_key: decode_pri(&data.primary, ShortcutKey::Super),
            secondary_key: decode_pri(&data.secondary, ShortcutKey::Shift),
            exit_header: decode_header(&data.exit_h, BindKey::BothKey),
            exit_key: data.exit_k,
            launch_header: decode_header(&data.launch_h, BindKey::PrimaryKey),
            launch_key: data.launch_k,
            kill_header: decode_header(&data.kill_h, BindKey::BothKey),
            kill_key: data.kill_k,
            minimize_header: decode_header(&data.mini_h, BindKey::BothKey),
            minimize_key: data.mini_k,
            scratch_header: decode_header(&data.scratch_h, BindKey::PrimaryKey),
            scratch_key: data.scratch_k,
            unsaved: false,
            capture_next: Some(CaptureInput::NoKey),
            index: 0,
            index_max: 4,
            border: data.border.clone(),
            window_anim: decode_win_anim(&data.win_anim, WindowAnimation::None),
            work_anim: decode_work_anim(&data.work_anim, WorkAnimation::None),
            blur: decode_blur(&data.blur),
            theme_set: ThemeSet {
                light: ThemeCustom {
                    application: theme::Palette {
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
                    application: theme::Palette {
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
            bar_left: left_widgets,
            bar_center: center_widgets,
            bar_right: right_widgets,
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
    WallpaperPrompt,
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
            match self { //respect locale preferences when pretty-printing
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
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
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
                    self.mk_config();
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
                        self.index_max = 4;
                    }
                    Page::Bind => {
                        self.index_max = 7;
                    }
                    Page::Bar => {
                        self.index_max = 5;
                    }
                    Page::Init => {
                        self.index_max = 1;
                    }
                    Page::Anim => {
                        self.index_max = 8;
                    }
                }
                if self.index > self.index_max {
                    self.index = self.index_max;
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
                self.kb_parse(x);
                iced::Command::none()
            }
            Message::Capture(x) => {
                self.capture_next = Some(x);
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
            Message::WallpaperPrompt => {
                match FileDialog::new().set_directory(self.wallpaper.clone()).pick_file() {
                    Some(path) => {
                        self.wallpaper = path.to_string_lossy().to_string();
                        self.unsaved = true;
                    },
                    None => {}
                };
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

        let main_txt = Text::new(Page::Main.to_string());
        let bind_txt = Text::new(Page::Bind.to_string());
        let bar_txt = Text::new(Page::Bar.to_string());
        let init_txt = Text::new(Page::Init.to_string());
        let anim_txt = Text::new(Page::Anim.to_string());
        let mut page_main = Button::new(main_txt)
            .on_press(Message::PageChanged(Page::Main))
            .width(SIDEBAR_WIDTH)
            .style(style.sidebar.mk_theme());
        let mut page_bind = Button::new(bind_txt)
            .on_press(Message::PageChanged(Page::Bind))
            .width(SIDEBAR_WIDTH)
            .style(style.sidebar.mk_theme());
        let mut page_bar = Button::new(bar_txt)
            .on_press(Message::PageChanged(Page::Bar))
            .width(SIDEBAR_WIDTH)
            .style(style.sidebar.mk_theme());
        let mut page_init = Button::new(init_txt)
            .on_press(Message::PageChanged(Page::Init))
            .width(SIDEBAR_WIDTH)
            .style(style.sidebar.mk_theme());
        let mut page_anim = Button::new(anim_txt)
            .on_press(Message::PageChanged(Page::Anim))
            .width(SIDEBAR_WIDTH)
            .style(style.sidebar.mk_theme());
        let page_cap = Button::new("").width(SIDEBAR_WIDTH).height(10000).style(style.sidebar.mk_theme()).on_press(Message::NoOp);
        let page_label = Text::new(tr("Available Pages"));
        match self.current_page {
            Page::Main => page_main = page_main.style(style.secondary.mk_theme()),
            Page::Bind => page_bind = page_bind.style(style.secondary.mk_theme()),
            Page::Bar => page_bar = page_bar.style(style.secondary.mk_theme()),
            Page::Init => page_init = page_init.style(style.secondary.mk_theme()),
            Page::Anim => page_anim = page_anim.style(style.secondary.mk_theme()),
        }
        let page_col = Column::new()
            .push(page_label)
            .push(page_main)
            .push(page_bind)
            .push(page_anim)
            .push(page_bar)
            .push(page_init)
            .push(page_cap)
            .align_items(Alignment::Start);

        let save_txt = Text::new(tr("Save"));
        let saved_txt = Text::new(tr("Saved!"));
        let save = match (self.unsaved, self.index == self.index_max) {
            (true, true) => {
                Button::new(save_txt).on_press(Message::Save).style(theme::Button::Positive)
            }
            (true, false) => {
                Button::new(save_txt).on_press(Message::Save)
            }
            (false, true) => {
                Button::new(saved_txt).on_press(Message::Save).style(theme::Button::Positive)
            }
            (false, false) => {
                Button::new(saved_txt).on_press(Message::Save).style(style.secondary.mk_theme())
            }
        };
        /* 
        if self.unsaved {
            save = Button::new(save_txt)
            .on_press(Message::Save);
        } else {
            save = Button::new(saved_txt)
            .on_press(Message::Save)
            .style(style.secondary.mk_theme());
        }
        */
        let save_row = Row::new()
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
        let test_rule = Rule::vertical(1);
        let scroll = Scrollable::new(settings);
        let col = Column::new()
            .push(scroll)
            .push(save_row)
            .width(Length::Fill)
            .align_items(Alignment::Start)
            .spacing(10);
        let master = Row::new()
            .push(page_col)
            .push(test_rule)
            .push(col);
        Container::new(master)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
    fn theme(&self) -> Theme {
        let colors = match self.theme {
            OurTheme::Light => self.theme_set.light.application,
            OurTheme::Dark => self.theme_set.dark.application,
            OurTheme::Custom => self.theme_set.custom.application,
        };
        let custom = Theme::Custom(Box::new(theme::Custom::new(colors)));
        custom
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
}
