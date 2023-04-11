use iced_style::{Color, button, Background};
use iced::Theme;

#[derive(Clone)]
pub struct ButtonStyle {
    pub border_radius: f32,
    pub txt_color: Color,
    pub bg_color: Color,
    pub border_color: Color,
    pub border_width: f32
}
impl button::StyleSheet for ButtonStyle {
    type Style = Theme;
    fn active(&self, style: &Theme) -> button::Appearance {
        button::Appearance {
            border_radius: self.border_radius.clone(),
            text_color: self.txt_color.clone(),
            border_color: self.border_color.clone(),
            border_width: self.border_width.clone(),
            background: Some(Background::Color(self.bg_color.clone())),
            ..button::Appearance::default()
        }
    }
}