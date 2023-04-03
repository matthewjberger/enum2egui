use egui::{widgets::DragValue, Ui};

pub use egui;
pub use enum2egui_derive::Gui;

pub trait Gui {
    fn ui(&mut self, ui: &mut Ui);
}

impl Gui for String {
    fn ui(&mut self, ui: &mut Ui) {
        ui.text_edit_multiline(self);
    }
}

impl Gui for bool {
    fn ui(&mut self, ui: &mut Ui) {
        ui.checkbox(self, "");
    }
}

impl Gui for i8 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Gui for i16 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Gui for i32 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Gui for i64 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Gui for u8 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Gui for u16 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Gui for u32 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Gui for u64 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Gui for f32 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Gui for f64 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}
