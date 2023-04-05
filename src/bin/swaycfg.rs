use iced::theme::{self, Theme};
use iced::{Result, Application, Settings, Alignment, Length, executor};
use iced::widget::{Button, Row, Column, Container, pick_list, Text, Scrollable};
use std::process::Command;
use std::fs::{self, read_to_string};
use std::env;
use serde_derive::{Deserialize, Serialize};
use toml::{self, from_str, to_string};

fn main() -> Result {
    Configurator::run(Settings::default())
}

pub fn get_home() -> String {
    match env::var("XDG_CONFIG_HOME") {
        Ok(var) => var,
        Err(..) => match env::var("HOME") {
            Ok(var) => format!("{var}/.config"),
            Err(..) => panic!("Failed to find config directory, make sure XDG_CONFIG_HOME or HOME are set")
        }
    }
}
pub fn get_lang() -> Translation {
    let locale = whoami::lang().collect::<Vec<String>>();
    let lang = locale[1].clone();
    let home = get_home();
    let langpath = format!("{home}/swaycfg/locale/cfg/{lang}.toml");
    let langfile = read_to_string(langpath).expect("no locale found");
    let decoded: Translation = from_str(&langfile).unwrap();
    decoded
}

pub fn rip_shortcut(opt: Option<ShortcutKey>) -> &'static str {
    let x;
    match opt {
        Some(..) => {
            x = match opt.unwrap() {
                ShortcutKey::Alt => "Mod1",
                ShortcutKey::Ctrl => "Control",
                ShortcutKey::Shift => "Shift",
                ShortcutKey::Super => "Mod4"
            };
        }
        None => x = ""
    }
    x
}
pub fn rip_bind(opt: Option<BindKey>) -> &'static str {
    let x;
    match opt {
        Some(..) => {
            x = match opt.unwrap() {
                BindKey::PrimaryKey => "$pri",
                BindKey::SecondaryKey => "$sec",
                BindKey::BothKey => "$pri+$sec"
            }
        }
        None => x = ""
    }
    x
}
pub fn rip_border(opt: Option<Border>) -> &'static str {
    let x;
    match opt {
        Some(..) => {
            x = match opt.unwrap() {
                Border::No => "none",
                Border::Csd => "csd",
                Border::Normal => "normal",
                Border::Pixel => "pixel"
            }
        }
        None => x = ""
    }
    x
}
#[derive(Deserialize, Debug, Serialize)]
pub struct Translation {
    global: Option<PageGlobals>,
    mainpage: Option<MainPage>,
    bindpage: Option<BindPage>,
    autopage: Option<AutoPage>,
    barpage: Option<BarPage>,
    prettyprint: Option<PrettyPrint>
}
#[derive(Deserialize, Debug, Serialize)]
pub struct MainPage {
    borders: String,
    width: String,
    theme: String,
    light: String,
    dark: String
}
#[derive(Deserialize, Debug, Serialize)]
pub struct BindPage {
    exit: String,
    keyplaceholder: String,
    launch: String,
    kill: String,
    mini: String,
    scratch: String,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct AutoPage {

}
#[derive(Deserialize, Debug, Serialize)]
pub struct BarPage {

}
#[derive(Deserialize, Debug, Serialize)]
pub struct PrettyPrint {
    borderno: String,
    bordernormal: String,
    bordercsd: String,
    borderpixel: String,
    keysuper: String,
    keyalt: String,
    keyshift: String,
    keyctrl: String,
    bindpri: String,
    bindsec: String,
    bindboth: String,
    pagemain: String,
    pagebind: String,
    pagebar: String,
    pageinit: String
}
#[derive(Deserialize, Debug, Serialize)]
pub struct PageGlobals {
    title: String,
    label: String,
    main: String,
    bind: String,
    bar: String,
    init: String,
    save: String,
    saved: String,
    primary: String,
    secondary: String,
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
    index: u8
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

#[derive(Deserialize, Debug, Serialize)]
pub struct FileData {
    theme: String,
    border: String,
    width: i32,
    primary: String,
    secondary: String,
    exith: String,
    exitk: String,
    launchh: String,
    launchk: String,
    killh: String,
    killk: String,
    minih: String,
    minik: String,
    scratchh: String,
    scratchk: String
}
pub fn getcfgdata() -> FileData {
    let home = get_home();
    let path = format!("{home}/swaycfg/swaycfg.toml");
    let file = match read_to_string(path) {
        Ok(var) => var,
        Err(..) => match read_to_string("/etc/swaycfg/swaycfg.toml") {
            Ok(var) => var,
            Err(..) => panic!("Failed to find swaycfg.toml in any valid directory")
        }
    };
    let decoded: FileData = from_str(&file).unwrap();
    decoded
}
pub fn decodetheme(x: &str, default: Theme) -> Theme {
    match x {
        "dark" => Theme::Dark,
        "light" => Theme::Light,
        &_ => default
    }
}
pub fn decodeborder(x: &str, default: Border) -> Option<Border> {
    match x {
        "none" => Some(Border::No),
        "csd" => Some(Border::Csd),
        "normal" => Some(Border::Normal),
        "pixel" => Some(Border::Pixel),
        &_ => Some(default)
    }
}
pub fn decodepri(x: &str, default: ShortcutKey) -> Option<ShortcutKey> {
    match x {
        "super" => Some(ShortcutKey::Super),
        "alt" => Some(ShortcutKey::Alt),
        "control" => Some(ShortcutKey::Ctrl),
        "shift" => Some(ShortcutKey::Shift),
        &_ => Some(default)
    }
}
pub fn decodeheader(x: &str, default: BindKey) -> Option<BindKey> {
    match x {
        "pri" => Some(BindKey::PrimaryKey),
        "sec" => Some(BindKey::SecondaryKey),
        "both" => Some(BindKey::BothKey),
        &_ => Some(default)
    }
}
pub fn encodetheme(x: Theme) -> &'static str {
    match x {
        Theme::Dark => "dark",
        Theme::Light => "light",
        Theme::Custom(..) => "light"
    }
}
pub fn encodeborder(x: Option<Border>) -> &'static str {
    match x.unwrap() {
        Border::No => "none",
        Border::Normal => "normal",
        Border::Csd => "csd",
        Border::Pixel => "pixel"
    }
}
pub fn encodepri(x: Option<ShortcutKey>) -> &'static str {
    match x.unwrap() {
        ShortcutKey::Super => "super",
        ShortcutKey::Alt => "alt",
        ShortcutKey::Ctrl => "control",
        ShortcutKey::Shift => "shift"
    }
}
pub fn encodeheader(x: Option<BindKey>) -> &'static str {
    match x.unwrap() {
        BindKey::PrimaryKey => "pri",
        BindKey::SecondaryKey => "sec",
        BindKey::BothKey => "both"
    }
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
            index: 0
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
    Init
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Border {
    #[default]
    No,
    Normal,
    Csd,
    Pixel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShortcutKey {
    #[default]
    Super,
    Alt,
    Shift,
    Ctrl
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BindKey {
    #[default]
    PrimaryKey,
    SecondaryKey,
    BothKey
}

impl Border {
    const ALL: [Border; 4] = [
        Border::No,
        Border::Normal,
        Border::Csd,
        Border::Pixel,
    ];
}
impl ShortcutKey {
    const ALL: [ShortcutKey; 4] = [
        ShortcutKey::Super,
        ShortcutKey::Alt,
        ShortcutKey::Shift,
        ShortcutKey::Ctrl,
    ];
}
impl BindKey {
    const ALL: [BindKey; 3] = [
        BindKey::PrimaryKey,
        BindKey::SecondaryKey,
        BindKey::BothKey,
    ];
}

impl std::fmt::Display for Border {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let locale = get_lang();
        let pretty = locale.prettyprint.unwrap();
        write!(
            f,
            "{}",
            match self {
                Border::No => pretty.borderno,
                Border::Normal => pretty.bordernormal,
                Border::Csd => pretty.bordercsd,
                Border::Pixel => pretty.borderpixel,
            }
        )
    }
}

impl std::fmt::Display for ShortcutKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let locale = get_lang();
        let pretty = locale.prettyprint.unwrap();
        write!(
            f,
            "{}",
            match self {
                ShortcutKey::Super => pretty.keysuper,
                ShortcutKey::Alt => pretty.keyalt,
                ShortcutKey::Shift => pretty.keyshift,
                ShortcutKey::Ctrl => pretty.keyctrl,
            }
        )
    }
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
            }
        )
    }
}
impl std::fmt::Display for BindKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let locale = get_lang();
        let pretty = locale.prettyprint.unwrap();
        write!(
            f,
            "{}",
            match self {
                BindKey::PrimaryKey => pretty.bindpri,
                BindKey::SecondaryKey => pretty.bindsec,
                BindKey::BothKey => pretty.bindboth
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
                {//Block that writes cfgvars
                let home = get_home();
                let data;
                let primary = rip_shortcut(self.primary_key);
                let secondary = rip_shortcut(self.secondary_key);
                let exith = rip_bind(self.exit_header);
                let exitk = &self.exit_key;
                let launchh = rip_bind(self.launch_header);
                let launchk = &self.launch_key;
                let killh = rip_bind(self.kill_header);
                let killk = &self.kill_key;
                let minih = rip_bind(self.minimize_header);
                let minik = &self.minimize_key;
                let scratchh = rip_bind(self.scratch_header);
                let scratchk = &self.scratch_key;
                let borderval = rip_border(self.border);
                let widthval = &self.width;
                let path = format!("{home}/sway/cfgvars");
                data = format!("#AUTO-GENERATED CONFIG, do not edit, any changed will be overwritten\ndefault_border {borderval} {widthval} \nset $pri {primary}\nset $sec {secondary}\n \nset $exit {exith}+{exitk}\nset $launch {launchh}+{launchk}\nset $kill {killh}+{killk}\nset $mini {minih}+{minik}\nset $scratch {scratchh}+{scratchk}");

                fs::write(path, data).expect("failed to write file");

                Command::new("swaymsg")
                    .arg("reload")
                    .spawn()
                    .expect("oops, swaymsg failed, do you have sway installed?");
                }
                {//Block that writes swaycfg.toml
                let home = get_home();
                let path = format!("{home}/swaycfg/swaycfg.toml");
                let data = FileData{
                    theme: encodetheme(self.theme.clone()).to_string(),
                    border: encodeborder(self.border).to_string(),
                    width: self.width,
                    primary: encodepri(self.primary_key).to_string(),
                    secondary: encodepri(self.secondary_key).to_string(),
                    exith: encodeheader(self.exit_header).to_string(),
                    exitk: self.exit_key.clone(),
                    launchh: encodeheader(self.launch_header).to_string(),
                    launchk: self.launch_key.clone(),
                    killh: encodeheader(self.kill_header).to_string(),
                    killk: self.kill_key.clone(),
                    minih: encodeheader(self.minimize_header).to_string(),
                    minik: self.minimize_key.clone(),
                    scratchh: encodeheader(self.scratch_header).to_string(),
                    scratchk: self.scratch_key.clone()
                };
                let toml = to_string(&data).expect("failed to generate toml");
                fs::write(path, toml).expect("failed to write swaycfg.toml");
            }
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
                                                Page::Main
                                            }
                                            Page::Bar => {
                                                Page::Bind
                                            }
                                            Page::Init => {
                                                Page::Bar
                                            }
                                        }
                                    }
                                } else if key_code == iced::keyboard::KeyCode::Down {
                                    if iced::keyboard::Modifiers::shift(modifiers) {//go down a page
                                        self.current_page = match self.current_page {
                                            Page::Main => {
                                                Page::Bind
                                            }
                                            Page::Bind => {
                                                Page::Bar
                                            }
                                            Page::Bar => {
                                                Page::Init
                                            }
                                            Page::Init => {
                                                Page::Main
                                            }
                                       }
                                    }
                                } else if key_code == iced::keyboard::KeyCode::S { //save
                                if self.unsaved {
                                    {//Block that writes cfgvars
                                    let home = get_home();
                                    let data;
                                    let primary = rip_shortcut(self.primary_key);
                                    let secondary = rip_shortcut(self.secondary_key);
                                    let exith = rip_bind(self.exit_header);
                                    let exitk = &self.exit_key;
                                    let launchh = rip_bind(self.launch_header);
                                    let launchk = &self.launch_key;
                                    let killh = rip_bind(self.kill_header);
                                    let killk = &self.kill_key;
                                    let minih = rip_bind(self.minimize_header);
                                    let minik = &self.minimize_key;
                                    let scratchh = rip_bind(self.scratch_header);
                                    let scratchk = &self.scratch_key;
                                    let borderval = rip_border(self.border);
                                    let widthval = &self.width;
                                    let path = format!("{home}/sway/cfgvars");
                                    data = format!("#AUTO-GENERATED CONFIG, do not edit, any changed will be overwritten\ndefault_border {borderval} {widthval} \nset $pri {primary}\nset $sec {secondary}\n \nset $exit {exith}+{exitk}\nset $launch {launchh}+{launchk}\nset $kill {killh}+{killk}\nset $mini {minih}+{minik}\nset $scratch {scratchh}+{scratchk}");
                    
                                    fs::write(path, data).expect("failed to write file");
                    
                                    Command::new("swaymsg")
                                        .arg("reload")
                                        .spawn()
                                        .expect("oops, swaymsg failed, do you have sway installed?");
                                    }
                                    {//Block that writes swaycfg.toml
                                    let home = get_home();
                                    let path = format!("{home}/swaycfg/swaycfg.toml");
                                    let data = FileData{
                                        theme: encodetheme(self.theme.clone()).to_string(),
                                        border: encodeborder(self.border).to_string(),
                                        width: self.width,
                                        primary: encodepri(self.primary_key).to_string(),
                                        secondary: encodepri(self.secondary_key).to_string(),
                                        exith: encodeheader(self.exit_header).to_string(),
                                        exitk: self.exit_key.clone(),
                                        launchh: encodeheader(self.launch_header).to_string(),
                                        launchk: self.launch_key.clone(),
                                        killh: encodeheader(self.kill_header).to_string(),
                                        killk: self.kill_key.clone(),
                                        minih: encodeheader(self.minimize_header).to_string(),
                                        minik: self.minimize_key.clone(),
                                        scratchh: encodeheader(self.scratch_header).to_string(),
                                        scratchk: self.scratch_key.clone()
                                    };
                                    let toml = to_string(&data).expect("failed to generate toml");
                                    fs::write(path, toml).expect("failed to write swaycfg.toml");
                                }
                                }
                                    self.unsaved = false;
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
        let globalstr = self.locale.global.as_ref().unwrap();
        let mainstr = self.locale.mainpage.as_ref().unwrap();
        let bindstr = self.locale.bindpage.as_ref().unwrap();

        let maintxt = String::as_str(&globalstr.main);
        let bindtxt = String::as_str(&globalstr.bind);
        let bartxt = String::as_str(&globalstr.bar);
        let inittxt = String::as_str(&globalstr.init);
        let pagetxt = String::as_str(&globalstr.label);
        let mut pagemain = Button::new(maintxt)
            .on_press(Message::PageChanged(Page::Main));
        let mut pagebind = Button::new(bindtxt)
            .on_press(Message::PageChanged(Page::Bind));
        let mut pagebar = Button::new(bartxt)
            .on_press(Message::PageChanged(Page::Bar));
        let mut pageinit = Button::new(inittxt)
            .on_press(Message::PageChanged(Page::Init));
        let pagelabel = Text::new(pagetxt);
        match self.current_page {
            Page::Main => {
                pagemain = pagemain.style(theme::Button::Secondary);
            }
            Page::Bind => {
                pagebind = pagebind.style(theme::Button::Secondary);
            }
            Page::Bar => {
                pagebar = pagebar.style(theme::Button::Secondary);
            }
            Page::Init => {
                pageinit = pageinit.style(theme::Button::Secondary);
            }
        }
        let pagecol = Column::new()
            .push(pagelabel)
            .push(pagemain)
            .push(pagebind)
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
        let primaryrow = Row::new()
            .push(primarylabel)
            .push(primarypick)
            .spacing(10);
        let secondaryrow = Row::new()
            .push(secondarylabel)
            .push(secondarypick)
            .spacing(10);
        
        let mut settings = Column::new().spacing(10);


        match self.current_page {
            Page::Main => {
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
                let borderrow = Row::new()
                    .push(borderlabel)
                    .push(bordertoggle)
                    .push(widthlabel)
                    .push(widthup)
                    .push(widthdown)
                    .spacing(10);

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
                let themerow = Row::new()
                    .push(themelabel)
                    .push(light)
                    .push(dark)
                    .spacing(20);
        
                settings = settings
                    .push(themerow)
                    .push(borderrow)
                    .push(primaryrow)
                    .push(secondaryrow);
            }
            Page::Bind => {
                let exitsclabel = Text::new(bindstr.exit.clone());
                let exitheaderselect = pick_list(
                &BindKey::ALL[..], 
                self.exit_header, 
                Message::ExitHeaderChanged,
                )
                .placeholder("choose");
                let exitkey = String::as_str(&self.exit_key);
                let exitkeyselect = Button::new(exitkey).on_press(Message::Capture(CaptureInput::ExitKey)).width(50);
                let exitscrow = Row::new()
                    .push(exitsclabel)
                    .push(exitheaderselect)
                    .push(exitkeyselect)
                    .spacing(10);
                let launchsclabel: Text = Text::new(bindstr.launch.clone());
                let launchheaderselect = pick_list(
                    &BindKey::ALL[..], 
                    self.launch_header, 
                    Message::LaunchHeaderChanged,
                    )
                    .placeholder("choose");
                let launchkey = String::as_str(&self.launch_key);
                let launchkeyselect = Button::new(launchkey).on_press(Message::Capture(CaptureInput::LaunchKey)).width(50);
                let launchscrow = Row::new()
                    .push(launchsclabel)
                    .push(launchheaderselect)
                    .push(launchkeyselect)
                    .spacing(10);
                let killsclabel: Text = Text::new(bindstr.kill.clone());
                let killheaderselect = pick_list(
                    &BindKey::ALL[..], 
                    self.kill_header, 
                    Message::KillHeaderChanged,
                    )
                    .placeholder("choose");
                let killkey = String::as_str(&self.kill_key);
                let killkeyselect = Button::new(killkey).on_press(Message::Capture(CaptureInput::KillKey)).width(50);
                let killscrow = Row::new()
                    .push(killsclabel)
                    .push(killheaderselect)
                    .push(killkeyselect)
                    .spacing(10);
                let minisclabel: Text = Text::new(bindstr.mini.clone());
                let miniheaderselect = pick_list(
                 &BindKey::ALL[..], 
                 self.minimize_header, 
                 Message::MiniHeaderChanged,
                 )
                    .placeholder("choose");
                let minikey = String::as_str(&self.minimize_key);
                let minikeyselect = Button::new(minikey).on_press(Message::Capture(CaptureInput::MiniKey)).width(50);
                let miniscrow = Row::new()
                    .push(minisclabel)
                    .push(miniheaderselect)
                    .push(minikeyselect)
                    .spacing(10);
                let scratchsclabel: Text = Text::new(bindstr.scratch.clone());
                let scratchheaderselect = pick_list(
                    &BindKey::ALL[..], 
                    self.scratch_header, 
                    Message::ScratchHeaderChanged,
                    )
                    .placeholder("choose");
                let scratchkey = String::as_str(&self.scratch_key);
                let scratchkeyselect = Button::new(scratchkey).on_press(Message::Capture(CaptureInput::ScratchKey)).width(50);
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
