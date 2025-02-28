use {
    crate::{
        App, InvTab, Tab, UiState,
        bitmanip_yet_again::Bitmanip,
        metadata::{self, map::MapInfo},
        sally_idx::IndexEntry,
    },
    eframe::egui::{self, Ui},
    libbgesav::{FollowState, Inventory, Password, Sav},
};

pub(crate) fn top_panel(app: &mut App, ui: &mut Ui) {
    let displayed = app.save_path.display().to_string();
    let s = if displayed.is_empty() {
        "<No file>".to_string()
    } else {
        displayed
    };
    ui.label(s);
    ui.horizontal(|ui| {
        if ui.button("Create clean save").clicked() {
            app.sav = Some(Sav::default());
        }
        if ui.button("🗁 Load file...").clicked() {
            if let Some(path) = rfd::FileDialog::default().pick_file() {
                match Sav::load_from_file(&path) {
                    Ok(sav) => {
                        app.sav = Some(sav);
                        app.save_path = path;
                    }
                    Err(e) => err_msg(e),
                }
            }
        }
        if let Some(path) = &app.bge_path {
            if ui
                .menu_button("Load slot...", |ui| {
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                    for (i, &exists) in app.slot_exist_array.iter().enumerate() {
                        if exists && ui.button(format!("Slot {i}")).clicked() {
                            ui.close_menu();
                            let sav_path = path.join(format!("slot{i}.sav"));
                            let sav = match Sav::load_from_file(&sav_path) {
                                Ok(sav) => sav,
                                Err(e) => {
                                    err_msg(e);
                                    break;
                                }
                            };
                            app.sav = Some(sav);
                            app.save_path = sav_path;
                        }
                    }
                })
                .response
                .clicked()
            {
                for (i, slot) in app.slot_exist_array.iter_mut().enumerate() {
                    *slot = path.join(format!("slot{i}.sav")).exists();
                }
            };
            if let Some(sav) = &app.sav {
                ui.menu_button("Save to slot...", |ui| {
                    for i in 0..5 {
                        if ui.button(format!("Slot {i}")).clicked() {
                            eprintln!("{:?}", sav.save_to_file(&path.join(format!("slot{i}.sav"))));
                            let idx_en = IndexEntry {
                                location: sav.current_map as u16,
                                entrance: sav.map_entry as u16,
                                play_time: 1, // TODO: Fetch from save file
                            };
                            eprintln!("{:?}", idx_en.write_to_index(&path.join("sally.idx"), i));
                            ui.close_menu();
                        }
                    }
                });
            }
            if ui.button("Open save dir").clicked() {
                eprintln!("{:?}", open::that(path));
            }
        }
        let (ctrl_r, ctrl_s) = ui.input(|inp| {
            let ctrl = inp.modifiers.ctrl;
            (
                ctrl && inp.key_pressed(egui::Key::R),
                ctrl && inp.key_pressed(egui::Key::S),
            )
        });
        if !app.save_path.as_os_str().is_empty()
            && (ui.button("⟲ Reload").on_hover_text("Ctrl+R").clicked() || ctrl_r)
        {
            match Sav::load_from_file(&app.save_path) {
                Ok(sav) => app.sav = Some(sav),
                Err(e) => err_msg(e),
            }
        }
        if let Some(sav) = &app.sav {
            if ui.button("💾 Save").on_hover_text("Ctrl+S").clicked() || ctrl_s {
                if let Err(e) = sav.save_to_file(&app.save_path) {
                    err_msg(e);
                }
            }
        }
    });
    if let Some(sav) = &app.sav {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut app.ui_state.tab, Tab::Inventory, "Inventory");
            ui.selectable_value(&mut app.ui_state.tab, Tab::Map, "Map");
            ui.selectable_value(&mut app.ui_state.tab, Tab::Party, "Party");
            ui.selectable_value(&mut app.ui_state.tab, Tab::MDisk, "MDisk");
            if ui
                .selectable_value(&mut app.ui_state.tab, Tab::Passwords, "Passwords")
                .clicked()
            {
                update_pw_bufs(&mut app.ui_state.password_bufs, sav);
            }
        });
    }
}

fn err_msg(e: std::io::Error) {
    rfd::MessageDialog::new()
        .set_title("Error")
        .set_description(&e.to_string())
        .show();
}

fn update_pw_bufs(pw_bufs: &mut [String; 30], sav: &Sav) {
    for (i, buf) in pw_bufs.iter_mut().enumerate() {
        buf.clear();
        pw_decode(&sav.passwords[i], buf);
    }
}

pub(crate) fn map(sav: &mut Sav, ui_state: &mut UiState, ui: &mut Ui, map_info: &MapInfo) {
    map_select_dropdown(ui, "Current map", &mut sav.current_map, map_info, ui_state);
    ui.separator();
    ui.horizontal(|ui| {
        if let Some(map_info_en) = map_info.get(&sav.current_map) {
            if !map_info_en.entrances.is_empty() {
                egui::ComboBox::from_label("Entrance")
                    .selected_text(
                        *map_info_en
                            .entrances
                            .get(&sav.map_entry)
                            .unwrap_or(&"<unknown>"),
                    )
                    .show_ui(ui, |ui| {
                        for (&idx, &name) in &map_info_en.entrances {
                            ui.selectable_value(&mut sav.map_entry, idx, name);
                        }
                    });
            }
        }
        ui.add(egui::DragValue::new(&mut sav.map_entry));
    });
}

fn map_select_dropdown(
    ui: &mut Ui,
    label: &str,
    map_val: &mut u8,
    map_info: &std::collections::HashMap<u8, metadata::map::MapDef>,
    ui_state: &mut UiState,
) {
    ui.horizontal(|ui| {
        egui::ComboBox::from_label(label)
            .selected_text(map_text(*map_val, map_info))
            .width(200.0)
            .show_ui(ui, |ui| {
                for i in 0..=255 {
                    if let Some(map_info_en) = map_info.get(&i) {
                        if map_info_en
                            .name
                            .to_ascii_lowercase()
                            .contains(&ui_state.map_filter.to_ascii_lowercase())
                        {
                            ui.selectable_value(map_val, i, map_text(i, map_info));
                        }
                    }
                }
            });
        ui.add(egui::TextEdit::singleline(&mut ui_state.map_filter).hint_text("Filter"));
        if !ui_state.map_filter.is_empty() && ui.button("🗙").on_hover_text("Clear").clicked() {
            ui_state.map_filter.clear();
        }
    });
}

fn map_text(idx: u8, map_info: &MapInfo) -> String {
    format!(
        "{idx}: {name}",
        name = map_info.get(&idx).map_or("<unknown>", |en| { en.name })
    )
}

pub(crate) fn mdisk(sav: &mut Sav, ui: &mut Ui) {
    for en in metadata::mdisk::TABLE {
        ui.checkbox(&mut sav.mdisks.disks[en.bit_idx as usize], en.name);
    }
}

pub(crate) fn party(sav: &mut Sav, ui: &mut Ui, map_info: &MapInfo, ui_state: &mut UiState) {
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
    ui.label("Condition flags");
    let labels = ["?", "motor healthy", "?", "?", "?", "?", "?", "?"];
    ui.horizontal(|ui| {
        for (i, label) in labels.into_iter().enumerate() {
            let mut on = sav.hovercraft_condition.bitflags.nth_bit_set(i);
            if ui.checkbox(&mut on, label).clicked() {
                sav.hovercraft_condition.bitflags.toggle_nth_bit(i);
            }
        }
    });
    map_select_dropdown(
        ui,
        "Docking map",
        &mut sav.hovercraft_dock_map,
        map_info,
        ui_state,
    );
    ui.label("Docking position");
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut sav.hovercraft_dock_x));
        ui.add(egui::DragValue::new(&mut sav.hovercraft_dock_y));
    });
    ui.label("Ride state");
    ui.add(egui::DragValue::new(&mut sav.hovercraft_state.value));
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
            FollowState::Invalid(_) => "Invalid",
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
        ui.checkbox(&mut ui_state.sync_pearls, "Pearl sync")
            .on_hover_text(
                "Synchronizes global pearl count with the number of pearls in Jade's inventory.",
            );
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
    let mut sync_pearls = false;
    let inv_ref = match ui_state.inv_tab {
        InvTab::Jade => {
            sync_pearls = ui_state.sync_pearls;
            &mut sav.jade_inventory
        }
        InvTab::Peyj => &mut sav.peyj_inventory,
        InvTab::DoubleH => &mut sav.double_h_inventory,
        InvTab::Hovercraft => &mut sav.hovercraft_inventory,
    };
    inv_ui(inv_ref, &mut sav.pearls, sync_pearls, ui);
}

fn inv_ui(inventory: &mut Inventory, pearls: &mut i32, pearl_sync: bool, ui: &mut Ui) {
    egui::Grid::new("inv_grid").show(ui, |ui| {
        for (i, qty) in inventory.iter_mut().enumerate() {
            ui.label(metadata::inventory::NAMES[i]);
            let re = ui.add(egui::DragValue::new(qty));
            if pearl_sync && i == PEARL_INV_IDX && re.changed() {
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

pub(crate) fn passwords(sav: &mut Sav, ui_state: &mut UiState, ui: &mut Ui) {
    egui::Grid::new("pw_grid").show(ui, |ui| {
        for (i, pw) in sav.passwords.iter_mut().enumerate() {
            ui.label(metadata::password::NAMES[i]);
            if ui
                .add_sized(
                    (48.0, 24.0),
                    egui::TextEdit::singleline(&mut ui_state.password_bufs[i]),
                )
                .lost_focus()
                && ui.input(|inp| inp.key_pressed(egui::Key::Enter))
            {
                *pw = pw_encode(&ui_state.password_bufs[i]);
            }
            if (i + 1) % 5 == 0 {
                ui.end_row();
            }
        }
    });
}

fn pw_decode(pw: &Password, buf: &mut String) {
    for &digit in pw {
        if digit == 0 {
            break;
        }
        let ch = metadata::password::CHARSET[(digit - 1) as usize];
        buf.push(ch as char);
    }
}

fn pw_encode(text: &str) -> Password {
    let bytes = text.as_bytes();
    let mut pw = [0; 7];
    for (&inp, out) in bytes.iter().zip(pw.iter_mut()) {
        *out = pw_digit(inp);
    }
    pw
}

fn pw_digit(ascii: u8) -> i32 {
    let ascii = ascii.to_ascii_lowercase();
    if ascii.is_ascii_digit() {
        (ascii - b'0') as i32 + 27
    } else if ascii.is_ascii_lowercase() {
        (ascii - b'a') as i32 + 1
    } else {
        0
    }
}
