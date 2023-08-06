use enum2egui::{Gui, GuiInspect};
use enum2str::EnumStr;

#[derive(Gui, EnumStr, Debug, Clone, Default, serde::Deserialize, serde::Serialize, PartialEq)]
pub enum Color {
    #[default]
    Red,
    Green,
    #[enum2str("Custom")]
    Custom(u8, u8, u8),
    NamedCustom {
        red: u8,
        blue: u8,
        green: u8,
        metadata: Metadata,
    },
}

#[derive(Gui, Clone, serde::Deserialize, serde::Serialize)]
pub struct Data {
    string: String,
    i8: i8,
    i16: i16,
    i32: i32,
    i64: i64,
    i128: i128,
    bool: bool,
    u8: u8,
    u16: u16,
    u32: u32,
    f32: f32,
    f64: f64,
    u128: u128,
    usize: usize,
    nested_struct: SubData,
    unnamed_struct: TupleStruct,
    color: Color,
    optional: Option<SubData>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            string: "Hello!".to_string(),
            i8: 42,
            i16: 1555,
            i32: -242522,
            i64: 23425259,
            i128: i128::MAX,
            bool: true,
            u8: 94,
            u16: 14029,
            u32: 3025844,
            f32: std::f32::consts::PI,
            f64: std::f64::consts::PI,
            u128: u128::MAX,
            usize: usize::MAX,
            nested_struct: SubData::default(),
            unnamed_struct: TupleStruct::default(),
            color: Color::default(),
            optional: Some(SubData::default()),
        }
    }
}

#[derive(Gui, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct TupleStruct(u8, u32, String, SubData);

impl Default for TupleStruct {
    fn default() -> Self {
        Self(3, 24, "Hello!".to_string(), SubData::default())
    }
}

#[derive(Gui, Clone, Default, serde::Deserialize, serde::Serialize, PartialEq, Debug)]
pub struct Metadata {
    message: String,
}

#[derive(Gui, Clone, Default, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct SubData {
    value: String,
    number: u32,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Default)]
pub struct DemoApp {
    data: Data,
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { data } = self;
        egui::CentralPanel::default().show(ctx, |ui| data.ui_mut(ui));
    }
}
