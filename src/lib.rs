use iced::{
    Color, Element, Length, Padding,
    theme::{self, Custom, Palette, Theme, palette},
    widget::{button as ibutton, center, container, text},
};
use iced_box::icon::lucide::{Lucide, lucide_font};
use std::{fs, sync::Arc};
pub mod reexports {
    pub use serde;
    pub use serde_json;
}

pub const DEFAULT_PALETTE: &'static [u8] = include_bytes!("default.json");

#[derive(serde::Deserialize, Debug)]
struct JsonPalette {
    background: String,
    primary: String,
    text: String,
    success: String,
    warning: String,
    danger: String,
}

pub fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap() as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap() as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap() as f32 / 255.0;
    Color::from_rgb(r, g, b)
}

impl From<JsonPalette> for Palette {
    fn from(p: JsonPalette) -> Self {
        Palette {
            background: hex_to_color(&p.background),
            primary: hex_to_color(&p.primary),
            text: hex_to_color(&p.text),
            success: hex_to_color(&p.success),
            warning: hex_to_color(&p.warning),
            danger: hex_to_color(&p.danger),
        }
    }
}

pub fn get_matugen_theme(conf_path: String) -> Theme {
    let file = fs::read_to_string(conf_path)
        .unwrap_or(String::from_utf8(DEFAULT_PALETTE.to_vec()).unwrap());
    let palette: JsonPalette = serde_json::from_str(&file).unwrap();
    Theme::Custom(Arc::new(Custom::new(
        "background".to_string(),
        palette.into(),
    )))
}

pub fn icon(l: Lucide) -> iced::widget::Text<'static> {
    text(l.to_string()).font(lucide_font()).size(20)
}

pub fn lerp_color(a: Color, b: Color, t: f64) -> Color {
    let t = t as f32;
    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

/// 34×34 square icon button, subtle fill
pub fn icon_button<'a, M: 'a>(e: impl Into<Element<'a, M>>) -> ibutton::Button<'a, M> {
    button(center(e).width(Length::Fill).height(Length::Fill))
        .style(|t: &Theme, s| ibutton::Style {
            background: Some(iced::Background::Color(match s {
                ibutton::Status::Pressed => palette::lighten(t.palette().background, 0.35),
                _ => palette::lighten(t.palette().background, 0.15),
            })),
            border: iced::Border {
                radius: 10.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: t.palette().text,
            ..ibutton::Style::default()
        })
        .width(34)
        .height(34)
        .padding(0)
}

pub fn icon_button_active<'a, M: 'a>(
    e: impl Into<Element<'a, M>>,
    activity: f64,
) -> ibutton::Button<'a, M> {
    button(center(e).width(Length::Fill).height(Length::Fill))
        .style(move |t: &Theme, _| ibutton::Style {
            background: Some(iced::Background::Color(lerp_color(
                t.palette().background,
                t.palette().primary,
                activity,
            ))),
            border: iced::Border {
                radius: 10.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: lerp_color(t.palette().text, t.palette().background, activity),
            ..ibutton::Style::default()
        })
        .width(34)
        .height(34)
        .padding(0)
}

pub fn button_active<'a, M: 'a>(
    e: impl Into<Element<'a, M>>,
    activity: f64,
) -> ibutton::Button<'a, M> {
    button(center(e).width(Length::Fill).height(Length::Fill))
        .style(move |t: &Theme, _| ibutton::Style {
            background: Some(iced::Background::Color(lerp_color(
                t.palette().background,
                t.palette().primary,
                activity,
            ))),
            border: iced::Border {
                radius: 10.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: lerp_color(t.palette().text, t.palette().background, activity),
            ..ibutton::Style::default()
        })
        .width(Length::Shrink)
        .height(34)
        .padding(Padding::new(0.0).horizontal(14))
}

/// Wide pill-shaped button (e.g. wifi, volume) with icon + label
pub fn button<'a, M: 'a>(e: impl Into<Element<'a, M>>) -> ibutton::Button<'a, M> {
    ibutton(center(e).width(Length::Fill).height(Length::Fill))
        .style(|t: &Theme, s| ibutton::Style {
            background: Some(iced::Background::Color(match s {
                ibutton::Status::Pressed => palette::lighten(t.palette().background, 0.35),
                _ => palette::lighten(t.palette().background, 0.15),
            })),
            border: iced::Border {
                radius: 10.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: t.palette().text,
            ..ibutton::Style::default()
        })
        .height(34)
        .width(Length::Shrink)
        .padding(Padding::new(0.0).horizontal(14))
}

/// Pill container — rounded, semi-transparent background, vertically centred
pub fn pill<'a, M: 'a>(wd: impl Into<Element<'a, M>>) -> Element<'a, M> {
    container(wd)
        .style(|t: &theme::Theme| container::Style {
            background: Some(iced::Background::Color(
                t.palette().background.scale_alpha(0.75),
            )),
            border: iced::Border {
                color: t.palette().text.scale_alpha(0.08),
                width: 1.0,
                radius: 14.into(),
            },
            ..container::Style::default()
        })
        .padding(Padding::new(0.0).horizontal(10))
        .height(Length::Fill)
        .center_y(Length::Fill)
        .into()
}
