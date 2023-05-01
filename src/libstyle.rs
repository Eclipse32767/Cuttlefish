#![allow(dead_code)]
use iced_style::{Color, button, pick_list, menu, Background};
use iced::Theme;
use serde_derive::{Deserialize, Serialize};
use toml::{self, from_str};
use std::fs::read_to_string;
use libcfg::get_home;
use crate::libcfg;

#[derive(Clone)]
pub struct ButtonStyle {
    pub border_radius: f32,
    pub txt_color: Color,
    pub bg_color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub shadow_offset: iced::Vector
}
impl button::StyleSheet for ButtonStyle {
    type Style = Theme;
    fn active(&self, _style: &Theme) -> button::Appearance {
        button::Appearance {
            shadow_offset: self.shadow_offset.clone(),
            border_radius: self.border_radius.clone(),
            text_color: self.txt_color.clone(),
            border_color: self.border_color.clone(),
            border_width: self.border_width.clone(),
            background: Some(Background::Color(self.bg_color.clone())),
        }
    }
}

#[derive(Clone)]
pub struct ListStyle {
    pub txt_color: Color,
    pub bg_color: Color,
    pub handle_color: Color,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Color
}
impl pick_list::StyleSheet for ListStyle {
    type Style = Theme;
    fn active(&self, _style: &Theme) -> pick_list::Appearance {
        pick_list::Appearance { 
            text_color: self.txt_color.clone(), 
            placeholder_color: Color::from_rgb8(0xFF, 0x00, 0x00), 
            handle_color: self.handle_color.clone(), 
            background: Background::Color(self.bg_color.clone()),
            border_radius: self.border_radius, 
            border_width: self.border_width, 
            border_color: self.border_color.clone() 
        }
    }
    fn hovered(&self, _style: &Theme) -> pick_list::Appearance {
        pick_list::Appearance { 
            text_color: self.txt_color.clone(), 
            placeholder_color: Color::from_rgb8(0xFF, 0x00, 0x00), 
            handle_color: self.handle_color.clone(), 
            background: Background::Color(self.bg_color.clone()),
            border_radius: self.border_radius, 
            border_width: self.border_width, 
            border_color: self.border_color.clone() 
        }
    }
}
#[derive(Clone)]
pub struct MenuStyle {
    pub txt_color: Color,
    pub bg_color: Color,
    pub border_width: f32,
    pub border_radius: f32,
    pub border_color: Color,
    pub sel_txt_color: Color,
    pub sel_bg_color: Color
}
pub struct ButtonPair {
    pub active: ButtonStyle,
    pub inactive: ButtonStyle
}
pub struct ThemeCustom {
    pub sidebar: ButtonPair,
    pub body: ButtonPair,
    pub menu: MenuStyle,
    pub list: ListStyle,
    pub text: Color,
    pub bg: Color
}
#[derive(Deserialize, Debug, Serialize)]
pub struct ThemeFile {
    pub app_style: Option<FileAppStyle>,
    pub sidebar_style: Option<SidebarStyle>,
    pub body_style: Option<BodyStyle>,
    pub menu_style: Option<FileMenuStyle>,
    pub list_style: Option<FileListStyle>,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct SidebarStyle {
    pub border_radius: f32,
    pub txt_color: String,
    pub bg_color: String,
    pub border_color: String,
    pub border_width: f32,
    pub de_border_radius: f32,
    pub de_txt_color: String,
    pub de_bg_color: String,
    pub de_border_color: String,
    pub de_border_width: f32
}
#[derive(Deserialize, Debug, Serialize)]
pub struct BodyStyle {
    pub border_radius: f32,
    pub txt_color: String,
    pub bg_color: String,
    pub border_color: String,
    pub border_width: f32,
    pub de_border_radius: f32,
    pub de_txt_color: String,
    pub de_bg_color: String,
    pub de_border_color: String,
    pub de_border_width: f32
}
#[derive(Deserialize, Debug, Serialize)]
pub struct FileMenuStyle {
    pub txt_color: String,
    pub bg_color: String,
    pub border_width: f32,
    pub border_radius: f32,
    pub border_color: String,
    pub sel_txt_color: String,
    pub sel_bg_color: String
}
#[derive(Deserialize, Debug, Serialize)]
pub struct FileAppStyle {
    pub background: String,
    pub text: String
}
#[derive(Deserialize, Debug, Serialize)]
pub struct FileListStyle {
    pub txt_color: String,
    pub bg_color: String,
    pub handle_color: String,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: String
}
pub fn string_to_color(x: String) -> Color {
    let splitstr = x.split_at(2);
    let redstr = splitstr.0;
    let splitstr = splitstr.1.split_at(2);
    let greenstr = splitstr.0;
    let bluestr = splitstr.1;
    let rednum = u8::from_str_radix(redstr, 16).expect("failed to parse red value");
    let greennum = u8::from_str_radix(greenstr, 16).expect("failed to parse green value");
    let bluenum = u8::from_str_radix(bluestr, 16).expect("failed to parse blue value");

    Color::from_rgb8(rednum, greennum, bluenum)
}
pub fn make_custom_theme() -> ThemeCustom {
    let home = get_home();
    let path = format!("{home}/swaycfg/theme.toml");
    let file = match read_to_string(path) {
        Ok(var) => var,
        Err(..) => match read_to_string("/etc/swaycfg/theme.toml") {
            Ok(var) => var,
            Err(..) => panic!("Failed to find theme.toml in any valid directory")
        }
    };
    let decoded: ThemeFile = from_str(&file).unwrap();
    let appstyle = decoded.app_style.unwrap();
    let sidebarstyle = decoded.sidebar_style.unwrap();
    let bodystyle = decoded.body_style.unwrap();
    let liststyle = decoded.list_style.unwrap();
    let menustyle = decoded.menu_style.unwrap();

    ThemeCustom { 
        sidebar: ButtonPair { 
            active: ButtonStyle {
                border_radius: sidebarstyle.border_radius,
                txt_color: string_to_color(sidebarstyle.txt_color),
                bg_color: string_to_color(sidebarstyle.bg_color),
                border_color: string_to_color(sidebarstyle.border_color),
                border_width: sidebarstyle.border_width,
                shadow_offset: iced::Vector { x: 0.0, y: 0.0 },
            }, 
            inactive: ButtonStyle {
                border_radius: sidebarstyle.de_border_radius,
                txt_color: string_to_color(sidebarstyle.de_txt_color),
                bg_color: string_to_color(sidebarstyle.de_bg_color),
                border_color: string_to_color(sidebarstyle.de_border_color),
                border_width: sidebarstyle.de_border_width,
                shadow_offset: iced::Vector { x: 0.0, y: 0.0 },
            }
        }, 
        body: ButtonPair { 
            active: ButtonStyle {
                border_radius: bodystyle.border_radius,
                txt_color: string_to_color(bodystyle.txt_color),
                bg_color: string_to_color(bodystyle.bg_color),
                border_color: string_to_color(bodystyle.border_color),
                border_width: bodystyle.border_width,
                shadow_offset: iced::Vector { x: 0.0, y: 0.0 },
            }, 
            inactive: ButtonStyle {
                border_radius: bodystyle.de_border_radius,
                txt_color: string_to_color(bodystyle.de_txt_color),
                bg_color: string_to_color(bodystyle.de_bg_color),
                border_color: string_to_color(bodystyle.de_border_color),
                border_width: bodystyle.de_border_width,
                shadow_offset: iced::Vector { x: 0.0, y: 0.0 },
            }, 
        },
        menu: MenuStyle { 
            txt_color: string_to_color(menustyle.txt_color), 
            bg_color: string_to_color(menustyle.bg_color), 
            border_width: menustyle.border_width, 
            border_radius: menustyle.border_radius, 
            border_color: string_to_color(menustyle.border_color), 
            sel_txt_color: string_to_color(menustyle.sel_txt_color), 
            sel_bg_color: string_to_color(menustyle.sel_bg_color) 
        }, 
        list: ListStyle { 
            txt_color: string_to_color(liststyle.txt_color), 
            bg_color: string_to_color(liststyle.bg_color), 
            handle_color: string_to_color(liststyle.handle_color), 
            border_radius: liststyle.border_radius, 
            border_width: liststyle.border_width, 
            border_color: string_to_color(liststyle.border_color) 
        }, 
        text: string_to_color(appstyle.text), 
        bg: string_to_color(appstyle.background) 
    }
}

impl menu::StyleSheet for MenuStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Theme) -> menu::Appearance {
        menu::Appearance { 
            text_color: self.txt_color.clone(), 
            background: Background::Color(self.bg_color.clone()), 
            border_width: self.border_width, 
            border_radius: self.border_radius, 
            border_color: self.border_color.clone(), 
            selected_text_color: self.sel_txt_color.clone(), 
            selected_background: Background::Color(self.sel_bg_color.clone())
        }
    }
}