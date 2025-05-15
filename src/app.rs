use core::f32;
use std::ops::Shl;

use egui::{menu::menu_button, style::HandleShape, *};
use crate::bitwidget::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
pub enum EditMode {
    Locked = 0,
    #[default]
    Edit = 1,
    AutoCollatz = 2,
    AutoFastCollatz = 3,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CollatzApp {
    // Example stuff:
    label: String,
    base_mask: u64,
    masks: Vec<u64>,
    edit_mode: EditMode,
    test_mask: u8,
    #[serde(skip)] // This how you opt-out of serialization of a field
    tab_index: usize,
}

impl Default for CollatzApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            base_mask: 0,
            masks: vec![0],
            edit_mode: EditMode::Edit,
            test_mask: 0,
            tab_index: 0,
        }
    }
}

impl CollatzApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

fn collatz(n: u64) -> u64 {
    if (n & 1) == 0 {
        n / 2
    } else {
        n * 3 + 1
    }
}
fn fast_collatz(n: u64) -> u64 {
    if (n & 1) == 0 {
        let zeros = n.trailing_zeros();
        n >> zeros
    } else {
        let n = n * 3 + 1;
        let zeros = n.trailing_zeros();
        n >> zeros
    }
}

fn fill_collatz(init: u64, masks: &mut Vec<u64>) {
    let mut n = init;
    masks.clear();
    loop {
        n = collatz(n);
        masks.push(n);
        if n == 1 {
            break;
        }
    }
}

fn fill_fast_collatz(init: u64, masks: &mut Vec<u64>) {
    let mut n = init;
    masks.clear();
    loop {
        n = fast_collatz(n);
        masks.push(n);
        if n == 1 {
            break;
        }
    }
}

impl CollatzApp {
    pub fn on_mask_update(&mut self) {
        match self.edit_mode {
            EditMode::Locked => (),
            EditMode::Edit => (),
            EditMode::AutoCollatz => {
                fill_collatz(self.base_mask, &mut self.masks);
            },
            EditMode::AutoFastCollatz => {
                fill_fast_collatz(self.base_mask, &mut self.masks);
            },
        }
    }
}

#[test]
fn incr_test() {
    struct Incr<'a>(&'a mut u32);

    impl<'a> Incr<'a> {
        pub fn next(&mut self) -> u32 {
            let next = *self.0;
            *self.0 += 1;
            next
        }
    }
    let mut value = 5u32;
    let mut incr = Incr(&mut value);
    println!("Next: {}", incr.next());
    println!("Next: {}", incr.next());
    println!("Next: {}", incr.next());
    println!("Next: {}", incr.next());
    println!("Value: {value}");
}

fn set_style(style: &mut Style) {
    style.visuals.widgets.active.corner_radius = CornerRadius::ZERO;
    style.visuals.widgets.hovered.corner_radius = CornerRadius::ZERO;
    style.visuals.widgets.inactive.corner_radius = CornerRadius::ZERO;
    style.visuals.widgets.noninteractive.corner_radius = CornerRadius::ZERO;
    style.visuals.widgets.open.corner_radius = CornerRadius::ZERO;
    style.visuals.menu_corner_radius = CornerRadius::ZERO;
    style.visuals.window_corner_radius = CornerRadius::ZERO;
}

impl eframe::App for CollatzApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            set_style(ui.style_mut());
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            set_style(ui.style_mut());
            // The central panel the region left after adding TopPanel's and SidePanel's
            let mut index = u32::MAX;
            ui.tracked_bitmask(self.test_mask, &mut index);
            if index != u32::MAX {
                self.label = format!("Interact Index: {index}");
            }
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    let old_edit_mode = self.edit_mode;
                    ComboBox::new("EditModeCombo", "Edit Mode")
                        .selected_text(format!("{:?}", self.edit_mode))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.edit_mode, EditMode::Edit, "Edit");
                            ui.selectable_value(&mut self.edit_mode, EditMode::AutoCollatz, "Auto Collatz");
                            ui.selectable_value(&mut self.edit_mode, EditMode::AutoFastCollatz, "Auto Fast Collatz");
                            ui.selectable_value(&mut self.edit_mode, EditMode::Locked, "Locked");
                        });
                    if old_edit_mode != self.edit_mode {
                        self.on_mask_update();
                    }
                    if ui.button("Clear").clicked() {
                        self.base_mask = 0;
                        self.on_mask_update();
                    }
                    if ui.button("Collatz").clicked() {
                        fill_collatz(self.base_mask, &mut self.masks);
                    }
                    if ui.button("Fast Collatz").clicked() {
                        fill_fast_collatz(self.base_mask, &mut self.masks);
                    }
                    if ui.button("Base-2 Steps").clicked() {
                        let mut n = self.base_mask;
                        let mut i = 0;
                        if (n & 1) == 0 {
                            n = self.base_mask >> self.base_mask.trailing_zeros();
                            self.masks[0] = n;
                            i += 1;
                        }
                        while (i + 3) < self.masks.len() {
                            // Mult 3 =>
                            // m := (n << 2) =>
                            // k := n + 1 =>
                            // g := m + k =>
                            // g >> g.trailing_zeros()
                            let m = n << 2;
                            self.masks[i] = m;
                            i += 1;
                            let k = n + n + 1;
                            self.masks[i] = k;
                            i += 1;
                            let g = m + k;
                            self.masks[i] = g;
                            i += 1;
                            let f = g >> g.trailing_zeros();
                            self.masks[i] = f;
                            i += 1;
                            n = f;
                        }
                        for i in i..self.masks.len() {
                            self.masks[i] = 0;
                        }
                    }
                    menu_button(ui, "Actions", |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("<<").clicked() {
                                self.base_mask <<= 1;
                                self.on_mask_update();
                            }
                            if ui.button(">>").clicked() {
                                self.base_mask >>= 1;
                                self.on_mask_update();
                            }
                        });
                    });
                });
                ui.label(&self.label);
                // let mut rem_index = None;
                ui.group(|ui| {
                    let mut index = u32::MAX;
                    let mut temp_mask = self.base_mask;
                    let base_resp = ui.tracked_bitmask(&mut temp_mask, &mut index);
                    match self.edit_mode {
                        EditMode::Locked => (),
                        EditMode::Edit => {
                            self.base_mask = temp_mask;
                        },
                        EditMode::AutoCollatz => {
                            self.base_mask = temp_mask;
                            if base_resp.clicked() {
                                fill_collatz(self.base_mask, &mut self.masks);
                            }
                        },
                        EditMode::AutoFastCollatz => {
                            self.base_mask = temp_mask;
                            if base_resp.clicked() {
                                fill_fast_collatz(self.base_mask, &mut self.masks);
                            }
                        },
                    }
                    if index != u32::MAX {
                        self.label = format!("Interact index: {index}");
                    }

                    let height = ui.available_height();
                    let scroll = ScrollArea::new(Vec2b::new(false, true))
                        .max_height(height * 0.75);
                    scroll.show_rows(ui, 24.0, self.masks.len(), |ui, range| {
                        // ui.style_mut().spacing.item_spacing = vec2(0.0, 0.0);
                        for i in range.into_iter() {
                            ui.horizontal(|ui| {
                                let mask = self.masks[i];
                                let mut index = u32::MAX;
                                let _mr = ui.tracked_bitmask(mask, &mut index);
                                if index != u32::MAX {
                                    self.label = format!("Interact index: {index}");
                                }
                                // if _mr.hovered() {
                                //     self.label = format!("Interact index: {index}.");
                                //     // self.masks[i] = mask;
                                // }
                                ui.label(format!("[{i:>5}] Ones: {:>2}", mask.count_ones()));
                                // if ui.button("⊗").clicked() {
                                //     rem_index = Some(i);
                                // }
                                _ = ui.allocate_space(vec2(16.0, 24.0));
                            });
                        }
                    });
                });
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.style_mut().spacing.item_spacing = vec2(0.0, 0.0);
                        for i in 0..8 {
                            if ui.selectable_value(&mut self.tab_index, i, format!("Tab_{i}")).clicked() {

                            }
                        }
                    });
                    ScrollArea::new(Vec2b::new(false, true))
                        .id_salt("messages_scroll")
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            let width = ui.available_width();
                            ui.set_width(width);
                            // ui.allocate_exact_size(vec2(width, 0.0), Sense::hover());
                            for i in 0..32 {
                                if i > 0 {
                                    let (rect, _) = ui.allocate_exact_size(vec2(width, 2.0), Sense::empty());
                                    ui.painter().rect_filled(rect, CornerRadius::ZERO, ui.style().visuals.text_color());
                                }
                                ui.label("The quick brown fox jumps over the lazy dog.");
                            }
                        });

                });
                // if let Some(index) = rem_index {
                //     self.masks.remove(index);
                // }
            });
        });
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.label(&self.label);
        });
    }
}