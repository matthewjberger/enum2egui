use egui::{widgets::DragValue, Ui};

pub use egui;

pub trait GuiInspect {
    fn ui(&self, ui: &mut Ui);
    fn ui_mut(&mut self, ui: &mut Ui);
}

macro_rules! impl_gui_for_primitive {
    ($($t:ty)*) => ($(
        impl GuiInspect for $t {
            fn ui(&self, ui: &mut Ui) {
                ui.label(format!("{self}"));
            }

            fn ui_mut(&mut self, ui: &mut Ui) {
                ui.add(DragValue::new(self));
            }
        }
    )*)
}

impl_gui_for_primitive!(i8 i16 i32 i64 u8 u16 u32 u64 f32 f64);

impl GuiInspect for bool {
    fn ui(&self, ui: &mut Ui) {
        ui.label(format!("{self}"));
    }

    fn ui_mut(&mut self, ui: &mut Ui) {
        ui.checkbox(self, "");
    }
}

impl GuiInspect for String {
    fn ui(&self, ui: &mut Ui) {
        ui.label(self);
    }

    fn ui_mut(&mut self, ui: &mut Ui) {
        ui.text_edit_singleline(self);
    }
}

impl<T> GuiInspect for Option<T>
where
    T: Default + GuiInspect,
{
    fn ui(&self, ui: &mut Ui) {
        if let Some(value) = self {
            value.ui(ui);
        } else {
            ui.label("None");
        }
    }

    fn ui_mut(&mut self, ui: &mut Ui) {
        let mut checked = self.is_some();
        ui.checkbox(&mut checked, "");

        match (checked, self.as_mut()) {
            (true, Some(value)) => value.ui_mut(ui),
            (true, None) => *self = Some(T::default()),
            (false, _) => *self = None,
        }
    }
}

macro_rules! impl_large_numerics {
    ($($t:ty)*) => ($(
        impl GuiInspect for $t {
            fn ui(&self, ui: &mut Ui) {
                ui.label(format!("{self}"));
            }

            fn ui_mut(&mut self, ui: &mut Ui) {
                let mut text = format!("{self}");
                ui.text_edit_singleline(&mut text);
                if let Ok(value) = text.parse() {
                    *self = value;
                }
            }
        }
    )*)
}

impl_large_numerics!(u128 i128 usize);

impl<T> GuiInspect for Vec<T>
where
    T: GuiInspect + Default,
{
    fn ui(&self, ui: &mut Ui) {
        egui::ScrollArea::vertical()
            .id_source(ui.next_auto_id())
            .show(ui, |ui| {
                self.iter().enumerate().for_each(|(index, item)| {
                    ui.group(|ui| {
                        ui.label(format!("Item {index}"));
                        item.ui(ui);
                    });
                });
                if self.is_empty() {
                    ui.label("Empty Vec");
                }
            });
    }

    fn ui_mut(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("Add Item").clicked() {
                        self.push(T::default());
                    }

                    if !self.is_empty() && ui.button("Remove Last Item").clicked() {
                        self.pop();
                    }
                });
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical(|ui| {
                        self.iter_mut().enumerate().for_each(|(index, item)| {
                            ui.group(|ui| {
                                ui.label(format!("Item {index}"));
                                item.ui_mut(ui);
                            });
                        });
                    });
                });
            });
        });
    }
}
