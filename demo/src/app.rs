use enum2egui::{Gui, GuiInspect};
use enum2str::EnumStr;
use std::collections::HashMap;

#[derive(Gui, EnumStr, Debug, Clone, Default, serde::Deserialize, serde::Serialize, PartialEq)]
pub enum Color {
    #[default]
    Red,

    Green,

    #[enum2str("Custom")]
    Custom(u8, u8, u8),

    #[enum2egui(skip)]
    #[enum2str("Skipped Custom")]
    SkippedCustom(u8, u8, u8),

    #[enum2egui(skip)]
    SkippedNamedCustom {
        red: u8,
        blue: u8,
        green: u8,
        metadata: Metadata,
    },

    NamedCustom {
        red: u8,
        blue: u8,
        green: u8,
        metadata: Metadata,
    },

    #[enum2str("Named Vec")]
    NamedVec {
        bytes: Vec<u8>,
    },

    #[enum2str("Unnamed Vec")]
    UnnamedVec(Vec<u8>),

    #[enum2str("Tuple Vec")]
    TupleVec(Vec<(u8, u32)>),
}

#[derive(Gui, Clone, serde::Deserialize, serde::Serialize)]
pub struct Data {
    #[enum2egui(skip)]
    skipped_data: u32,

    #[enum2egui(skip)]
    hashmap: std::collections::HashMap<String, Metadata>,

    #[enum2egui(label = "Labeled Field")]
    a: String,

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
    primary_color: Color,
    secondary_color: Color,
    optional: Option<SubData>,
    list: Vec<Color>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            hashmap: HashMap::default(),
            skipped_data: 0,
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
            primary_color: Color::default(),
            secondary_color: Color::default(),
            optional: Some(SubData::default()),
            list: vec![
                Color::Red,
                Color::Green,
                Color::Custom(3, 2, 1),
                Color::NamedCustom {
                    red: 23,
                    blue: 100,
                    green: 30,
                    metadata: Metadata {
                        message: "Hello!".to_string(),
                    },
                },
            ],
            a: "".to_string(),
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
    list: Vec<Metadata>,
}

#[derive(
    Default,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    Eq,
    PartialEq,
    Hash,
    Clone,
    Copy,
    EnumStr,
    PartialOrd,
    Gui,
)]
pub enum Simple {
    #[default]
    Variant,
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
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .id_source(ui.next_auto_id())
                .max_width(1000.0)
                .max_height(1000.0)
                .show(ui, |ui| {
                    data.ui_mut(ui);
                });
        });
    }
}
