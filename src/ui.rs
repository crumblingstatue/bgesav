use eframe::egui::{self, Ui};
use libbgesav::{FollowState, Inventory, Sav};

use crate::{metadata, App, InvTab, Tab, UiState};

pub(crate) fn top_panel(app: &mut App, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("File");
        let displayed = app.save_path.display().to_string();
        let s = if displayed.is_empty() {
            "<No file>".to_string()
        } else {
            displayed
        };
        ui.label(s);
        if ui.button("🗁 Open...").clicked() {
            if let Some(path) = rfd::FileDialog::default().pick_file() {
                let sav = Sav::load_from_file(&path).unwrap();
                app.sav = Some(sav);
                app.save_path = path;
            }
        }
        let (ctrl_r, ctrl_s) = {
            let inp = ui.input();
            let ctrl = inp.modifiers.ctrl;
            (
                ctrl && inp.key_pressed(egui::Key::R),
                ctrl && inp.key_pressed(egui::Key::S),
            )
        };
        if !app.save_path.as_os_str().is_empty()
            && (ui.button("⟲ Reload").on_hover_text("Ctrl+R").clicked() || ctrl_r)
        {
            let sav = Sav::load_from_file(&app.save_path).unwrap();
            app.sav = Some(sav);
        }
        if let Some(sav) = &app.sav {
            if ui.button("💾 Save").on_hover_text("Ctrl+S").clicked() || ctrl_s {
                sav.save_to_file(&app.save_path).unwrap();
            }
        }
    });
    ui.horizontal(|ui| {
        ui.selectable_value(&mut app.ui_state.tab, Tab::Inventory, "Inventory");
        ui.selectable_value(&mut app.ui_state.tab, Tab::Map, "Map");
        ui.selectable_value(&mut app.ui_state.tab, Tab::Party, "Party");
        ui.selectable_value(&mut app.ui_state.tab, Tab::MDisk, "MDisk");
    });
}

pub(crate) fn map(sav: &mut Sav, ui_state: &mut UiState, ui: &mut Ui) {
    ui.horizontal(|ui| {
        egui::ComboBox::from_label("Current map")
            .selected_text(map_text(sav.current_map))
            .width(200.0)
            .show_ui(ui, |ui| {
                for i in 0..=255 {
                    if metadata::map::NAMES[i as usize]
                        .to_ascii_lowercase()
                        .contains(&ui_state.map_filter.to_ascii_lowercase())
                    {
                        ui.selectable_value(&mut sav.current_map, i, map_text(i));
                    }
                }
            });
        ui.add(egui::TextEdit::singleline(&mut ui_state.map_filter).hint_text("Filter"));
        if !ui_state.map_filter.is_empty() && ui.button("🗙").on_hover_text("Clear").clicked() {
            ui_state.map_filter.clear();
        }
    });
    ui.separator();
    ui.label("Entry");
    ui.add(egui::DragValue::new(&mut sav.map_entry));
}

fn map_text(idx: u8) -> String {
    format!("{idx}: {name}", name = metadata::map::NAMES[idx as usize])
}

pub(crate) fn mdisk(sav: &mut Sav, ui: &mut Ui) {
    for en in metadata::mdisk::TABLE {
        ui.checkbox(&mut sav.mdisks.disks[en.bit_idx as usize], en.name);
    }
}

pub(crate) fn party(sav: &mut Sav, ui: &mut Ui) {
    ui.heading("Jade");
    ui.horizontal(|ui| {
        ui.label("Max health");
        ui.add(egui::DragValue::new(&mut sav.jade_max_health));
        ui.label("Current health");
        ui.add(egui::DragValue::new(&mut sav.jade_curr_health));
    });
    ui.heading("Pey'j");
    ui.horizontal(|ui| {
        ui.checkbox(&mut sav.party.peyj, "present");
        follow_state_ui(0, &mut sav.peyj_follow_state, ui);
        ui.separator();
        ui.label("Health");
        ui.add(egui::DragValue::new(&mut sav.peyj_curr_health));
        ui.label("/");
        ui.add(egui::DragValue::new(&mut sav.peyj_max_health));
    });
    ui.heading("Double H");
    ui.horizontal(|ui| {
        ui.checkbox(&mut sav.party.double_h, "present");
        follow_state_ui(1, &mut sav.double_h_follow_state, ui);
        ui.separator();
        ui.label("Health");
        ui.add(egui::DragValue::new(&mut sav.double_h_curr_health));
        ui.label("/");
        ui.add(egui::DragValue::new(&mut sav.double_h_max_health));
    });
    ui.heading("Hovercraft");
    ui.horizontal(|ui| {
        ui.label("Health");
        ui.add(egui::DragValue::new(&mut sav.hovercraft_curr_health));
        ui.label("/");
        ui.add(egui::DragValue::new(&mut sav.hovercraft_max_health));
    });
    ui.heading("Alpha Soldier");
    ui.checkbox(&mut sav.party.alpha_soldier, "present");
    //follow_state_ui(&mut sav.peyj_follow_state.0, ui);
}

pub(crate) fn follow_state_ui(id: u8, follow_state: &mut FollowState, ui: &mut Ui) {
    fn label(s: FollowState) -> &'static str {
        match s {
            FollowState::Follow => "Follow",
            FollowState::Unknown1 => "Unknown1",
            FollowState::Unknown2 => "Unknown2",
            FollowState::Unknown3 => "Unknown3",
            FollowState::Unknown4 => "Unknown4",
        }
    }

    egui::ComboBox::new(id, "Follow state")
        .selected_text(label(*follow_state))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                follow_state,
                FollowState::Follow,
                label(FollowState::Follow),
            );
            ui.selectable_value(
                follow_state,
                FollowState::Unknown1,
                label(FollowState::Unknown1),
            );
            ui.selectable_value(
                follow_state,
                FollowState::Unknown2,
                label(FollowState::Unknown2),
            );
            ui.selectable_value(
                follow_state,
                FollowState::Unknown3,
                label(FollowState::Unknown3),
            );
            ui.selectable_value(
                follow_state,
                FollowState::Unknown4,
                label(FollowState::Unknown4),
            );
        });
}

const PEARL_INV_IDX: usize = 46;

pub(crate) fn inventory(sav: &mut Sav, ui: &mut Ui, ui_state: &mut UiState) {
    ui.horizontal(|ui| {
        ui.label("Units");
        ui.add(egui::DragValue::new(&mut sav.units));
        ui.label("Pearls");
        let re = ui.add(egui::DragValue::new(&mut sav.pearls));
        if ui_state.sync_pearls && re.changed() {
            sav.jade_inventory[PEARL_INV_IDX] = sav.pearls;
        }
    });
    ui.separator();
    ui.heading("Party inventories");
    ui.horizontal(|ui| {
        ui.selectable_value(&mut ui_state.inv_tab, InvTab::Jade, "Jade");
        ui.selectable_value(&mut ui_state.inv_tab, InvTab::Peyj, "Pey'j");
        ui.selectable_value(&mut ui_state.inv_tab, InvTab::DoubleH, "Double H");
        ui.selectable_value(&mut ui_state.inv_tab, InvTab::Hovercraft, "Hovercraft");
    });
    ui.separator();
    let inv_ref = match ui_state.inv_tab {
        InvTab::Jade => &mut sav.jade_inventory,
        InvTab::Peyj => &mut sav.peyj_inventory,
        InvTab::DoubleH => &mut sav.double_h_inventory,
        InvTab::Hovercraft => &mut sav.hovercraft_inventory,
    };
    inv_ui(inv_ref, &mut sav.pearls, ui_state, ui);
    ui.checkbox(&mut ui_state.sync_pearls, "Pearl sync")
        .on_hover_text(
            "Synchronizes the pearl currency value with the pearl item amount in Jade's inventory.",
        );
}

fn inv_ui(inventory: &mut Inventory, pearls: &mut i32, ui_state: &mut UiState, ui: &mut Ui) {
    egui::Grid::new("inv_grid").show(ui, |ui| {
        for (i, qty) in inventory.iter_mut().enumerate() {
            ui.label(metadata::inventory::NAMES[i]);
            let re = ui.add(egui::DragValue::new(qty));
            if i == PEARL_INV_IDX && re.changed() && ui_state.sync_pearls {
                *pearls = *qty;
            }
            if (i + 1) % 3 == 0 {
                ui.end_row();
            }
        }
    });
    if ui.button("🗑 Clear").clicked() {
        inventory.fill(0);
    }
}
