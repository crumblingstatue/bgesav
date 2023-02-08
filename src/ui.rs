use eframe::egui::{self, Ui};
use libbgesav::{FollowState, Sav, SavExt};

use crate::{metadata, App, Tab, UiState};

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
            .selected_text(map_text(sav.current_map.0))
            .width(200.0)
            .show_ui(ui, |ui| {
                for i in 0..=255 {
                    if metadata::map::NAMES[i as usize]
                        .to_ascii_lowercase()
                        .contains(&ui_state.map_filter.to_ascii_lowercase())
                    {
                        ui.selectable_value(&mut sav.current_map.0, i, map_text(i));
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
    ui.add(egui::DragValue::new(&mut sav.map_entry.0));
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
    ui.heading("Pey'j");
    ui.checkbox(&mut sav.party.peyj, "present");
    follow_state_ui(0, &mut sav.peyj_follow_state.0, ui);
    ui.heading("Double H");
    ui.checkbox(&mut sav.party.double_h, "present");
    follow_state_ui(1, &mut sav.double_h_follow_state.0, ui);
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

pub(crate) fn inventory(sav: &mut Sav, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("Units");
        ui.add(egui::DragValue::new(&mut sav.units.0));
    });
}
