# enum2egui

[<img alt="github" src="https://img.shields.io/badge/github-matthewjberger/enum2egui-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/matthewjberger/enum2egui)
[<img alt="crates.io" src="https://img.shields.io/crates/v/enum2egui.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/enum2egui)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-enum2egui-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/enum2egui)

enum2egui is a rust derive macro that creates egui UI's from arbitrary structs and enums.
This is useful for generating data bindings that can be modified and displayed in an [egui](https://github.com/emilk/egui) ui. 

`Default` and `Display` are required. [enum2str](https://github.com/matthewjberger/enum2str) is recommended for deriving `Display` on enums.

## Usage

Add this to your `Cargo.toml`:

```toml
enum2egui = "0.1.5"
```

### Example

Declare your data:

```rust
use enum2egui::{Gui, GuiInspect};

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

    #[enum2egui(skip)]
    SkippedGreen,

    #[enum2egui(skip)]
    #[enum2str("Skipped Custom")]
    SkippedCustom(u8, u8, u8),

}

#[derive(Gui, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct Data {
    string: String,
    i8: i8,
    i16: i16,
    i32: i32,
    i64: i64,
    bool: bool,
    u8: u8,
    u16: u16,
    u32: u32,
    f32: f32,
    f64: f64,
    nested_struct: SubData,
    unnamed_struct: TupleStruct,
    primary_color: Color,
    secondary_color: Color,
    optional: Option<SubData>,
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
```

Then render it with `GuiInspect::ui(..)` or `GuiInspect::ui_mut()`. For example, with `eframe`:

```rust
impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { data } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            // Read-Only UI
            data.ui(ui):

            // Mutable UI
            data.ui_mut(ui);
        });
    }
}
```

![image](https://github.com/matthewjberger/enum2egui/assets/7131091/4a7119e0-0ea1-4ce6-b492-8eca48da792c)
