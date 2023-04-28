use iced_style::{Color, button, pick_list, menu, Background};
use iced::Theme;

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