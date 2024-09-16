#![allow(non_snake_case)]

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
        self.iter().for_each(|item| {
            item.ui(ui);
        });
        if self.is_empty() {
            ui.label("Empty Vec");
        }
    }

    fn ui_mut(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("Add").clicked() {
                        self.push(T::default());
                    }

                    if ui
                        .add_enabled(!self.is_empty(), egui::Button::new("Remove Last"))
                        .clicked()
                    {
                        self.pop();
                    }
                });
                ui.separator();
                ui.vertical(|ui| {
                    self.iter_mut().for_each(|item| {
                        item.ui_mut(ui);
                    });
                });
            });
        });
    }
}

impl<K> GuiInspect for std::collections::HashSet<K>
where
    K: GuiInspect + Clone + std::hash::Hash + Eq + Default + std::fmt::Debug,
{
    fn ui(&self, _ui: &mut Ui) {}
    fn ui_mut(&mut self, _ui: &mut Ui) {}
}

impl<K, V> GuiInspect for std::collections::BTreeMap<K, V>
where
    K: GuiInspect + Clone + std::hash::Hash + Eq + Default + std::fmt::Debug,
    V: GuiInspect + Default,
{
    fn ui(&self, _ui: &mut Ui) {}
    fn ui_mut(&mut self, _ui: &mut Ui) {}
}

impl<K, V> GuiInspect for std::collections::HashMap<K, V>
where
    K: GuiInspect + Clone + std::hash::Hash + Eq + Default + std::fmt::Debug,
    V: GuiInspect + Default,
{
    fn ui(&self, _ui: &mut Ui) {}
    fn ui_mut(&mut self, _ui: &mut Ui) {}
}

#[cfg(feature = "hashbrown")]
impl<K, V> GuiInspect for hashbrown::HashMap<K, V>
where
    K: GuiInspect + Clone + std::hash::Hash + Eq + Default + std::fmt::Debug,
    V: GuiInspect + Default,
{
    fn ui(&self, _ui: &mut Ui) {}
    fn ui_mut(&mut self, _ui: &mut Ui) {}
}

macro_rules! impl_gui_for_tuples {
    ( $( $name:ident )+ ) => {
        impl<$($name: GuiInspect),+> GuiInspect for ($($name,)+) {
            fn ui(&self, ui: &mut Ui) {
                ui.horizontal(|ui| {
                    let ($($name,)+) = self;
                    $(
                        $name.ui(ui);
                    )+
                });
            }

            fn ui_mut(&mut self, ui: &mut Ui) {
                ui.horizontal(|ui| {
                    let ($($name,)+) = self;
                    $(
                        $name.ui_mut(ui);
                    )+
                });
            }
        }
    }
}

// Implement for tuples of size 1 to 12
impl_gui_for_tuples!(T1);
impl_gui_for_tuples!(T1 T2);
impl_gui_for_tuples!(T1 T2 T3);
impl_gui_for_tuples!(T1 T2 T3 T4);
impl_gui_for_tuples!(T1 T2 T3 T4 T5);
impl_gui_for_tuples!(T1 T2 T3 T4 T5 T6);
impl_gui_for_tuples!(T1 T2 T3 T4 T5 T6 T7);
impl_gui_for_tuples!(T1 T2 T3 T4 T5 T6 T7 T8);
impl_gui_for_tuples!(T1 T2 T3 T4 T5 T6 T7 T8 T9);
impl_gui_for_tuples!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10);
impl_gui_for_tuples!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11);
impl_gui_for_tuples!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12);
