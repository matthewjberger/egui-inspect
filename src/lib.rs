use egui::{widgets::DragValue, Ui};

pub use egui;

pub trait Inspect {
    fn ui(&mut self, ui: &mut Ui);
}

impl Inspect for String {
    fn ui(&mut self, ui: &mut Ui) {
        ui.text_edit_multiline(self);
    }
}

impl Inspect for bool {
    fn ui(&mut self, ui: &mut Ui) {
        ui.checkbox(self, "");
    }
}

impl Inspect for i8 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Inspect for i16 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Inspect for i32 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Inspect for i64 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Inspect for u8 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Inspect for u16 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Inspect for u32 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Inspect for u64 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Inspect for f32 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}

impl Inspect for f64 {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(DragValue::new(self));
    }
}
