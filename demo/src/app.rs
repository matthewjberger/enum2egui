use enum2egui::Gui;

#[derive(Gui, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Data {
    value_string: String,
    value_i8: i8,
    value_i16: i16,
    value_i32: i32,
    value_i64: i64,
    value_bool: bool,
    value_u8: u8,
    value_u16: u16,
    value_u32: u32,
    value_f32: f32,
    value_f64: f64,
    sub_data: SubData,
}

#[derive(Gui, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct SubData {
    value: String,
    number: u32,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DemoApp {
    data: Data,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            data: Data::default(),
        }
    }
}

impl DemoApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for DemoApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { data } = self;

        egui::CentralPanel::default().show(ctx, |ui| data.ui(ui));
    }
}
