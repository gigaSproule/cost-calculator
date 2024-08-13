use std::sync::{Arc, Mutex};

use adw::{glib::GString, ApplicationWindow};
use gtk4::{glib::clone, prelude::*};

use crate::{store, store::materials::get_materials, store::materials::StoredMaterial};

pub(crate) fn materials(window: &ApplicationWindow) -> gtk4::Grid {
    let mut existing_materials = get_materials();
    if existing_materials.len() == 0 {
        existing_materials = vec![StoredMaterial {
            name: String::from("Material"),
            quantity_per_pack: 0.0,
            price_per_pack: 0.0,
            value: 0.0,
        }]
    }

    let container = gtk4::Grid::builder()
        .margin_start(5)
        .margin_end(5)
        .margin_top(5)
        .margin_bottom(5)
        .row_spacing(5)
        .column_spacing(5)
        .build();

    let material_costs_wrapper = gtk4::Grid::builder()
        .margin_top(5)
        .margin_bottom(5)
        .row_spacing(5)
        .column_spacing(5)
        .build();
    let material_costs_container = gtk4::Grid::builder()
        .row_spacing(5)
        .column_spacing(5)
        .hexpand(true)
        .build();
    material_costs_wrapper.attach(&material_costs_container, 0, 0, 2, 1);
    let material_costs_frame = gtk4::Frame::builder()
        .label("Cost of materials")
        .child(&material_costs_wrapper)
        .build();
    container.attach(&material_costs_frame, 0, 1, 2, 1);

    let material_name_label = gtk4::Label::builder()
        .label("Name")
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .build();
    material_costs_container.attach(&material_name_label, 0, 0, 1, 1);
    let quantity_per_pack_label = gtk4::Label::builder()
        .label("Quantity Per Pack")
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .build();
    material_costs_container.attach(&quantity_per_pack_label, 1, 0, 1, 1);
    let price_per_pack_label = gtk4::Label::builder()
        .label("Price Per Pack")
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .build();
    material_costs_container.attach(&price_per_pack_label, 2, 0, 1, 1);

    let material_costs_list_box = gtk4::ListBox::builder()
        .selection_mode(gtk4::SelectionMode::None)
        .margin_top(5)
        .margin_bottom(5)
        .css_name("no-background")
        .build();
    material_costs_container.attach(&material_costs_list_box, 0, 1, 3, 1);

    let material_costs_entries: Arc<
        Mutex<Vec<(gtk4::EditableLabel, gtk4::SpinButton, gtk4::SpinButton)>>,
    > = Arc::new(Mutex::new(vec![]));

    for material in existing_materials {
        add_material_row(&material_costs_list_box, &material_costs_entries, material);
    }

    let add_material_costs = gtk4::Button::builder()
        .icon_name("list-add-symbolic")
        .halign(gtk4::Align::End)
        .valign(gtk4::Align::End)
        .margin_end(5)
        .build();
    material_costs_wrapper.attach(&add_material_costs, 1, 1, 1, 1);

    let material_costs_entries_add = material_costs_entries.clone();
    add_material_costs.connect_clicked(clone!(
        #[strong]
        material_costs_list_box,
        #[strong]
        material_costs_container,
        move |_| {
            add_material_row(
                &material_costs_list_box,
                &material_costs_entries_add,
                StoredMaterial::default(),
            );
        }
    ));

    let save = gtk4::Button::builder()
        .label("Save")
        .halign(gtk4::Align::End)
        .valign(gtk4::Align::Center)
        .build();

    let material_costs_entries_calculate = material_costs_entries.clone();
    save.connect_clicked(clone!(
        #[strong]
        window,
        move |_| {
            let material_entries = material_costs_entries_calculate.lock().unwrap();
            let materials: Vec<StoredMaterial> = material_entries
                .iter()
                .map(
                    |(label, quantity_per_pack, price_per_pack)| StoredMaterial {
                        name: label.text().to_string(),
                        quantity_per_pack: quantity_per_pack.value(),
                        price_per_pack: price_per_pack.value(),
                        value: price_per_pack.value() / quantity_per_pack.value(),
                    },
                )
                .collect();
            store::materials::store_materials(materials);

            let saved_dialog = gtk4::MessageDialog::builder()
                .modal(true)
                .text("Options saved.")
                .buttons(gtk4::ButtonsType::Ok)
                .message_type(gtk4::MessageType::Info)
                .transient_for(&window)
                .build();
            saved_dialog.connect_response(|dialog, _| {
                dialog.close();
            });
            saved_dialog.show();
        }
    ));

    container.attach(&save, 1, 4, 1, 1);
    container
}

fn add_material_row(
    material_costs_list_box: &gtk4::ListBox,
    material_costs_entries: &Arc<
        Mutex<Vec<(gtk4::EditableLabel, gtk4::SpinButton, gtk4::SpinButton)>>,
    >,
    material: StoredMaterial,
) {
    let material_costs_list_child = gtk4::ListBoxRow::builder()
        .margin_start(5)
        .margin_top(2)
        .margin_bottom(2)
        .activatable(false)
        .focusable(false)
        .build();
    material_costs_list_box.append(&material_costs_list_child);

    let material_costs_box = gtk4::Box::builder().build();
    material_costs_list_child.set_child(Some(&material_costs_box));

    let material_label = gtk4::EditableLabel::builder()
        .text(&material.name)
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .margin_start(5)
        .build();
    material_costs_box.append(&material_label);

    let material_quantity_per_pack_adjustment =
        gtk4::Adjustment::new(0.0, 0.0, 1000.0, 1.0, 5.0, 0.0);
    let material_quantity_per_pack_input = gtk4::SpinButton::builder()
        .name(format!("{}_quantity_per_pack", &material.name))
        .hexpand(true)
        .margin_start(5)
        .adjustment(&material_quantity_per_pack_adjustment)
        .climb_rate(1.0)
        .numeric(true)
        .digits(0)
        .value(material.quantity_per_pack)
        .build();
    material_costs_box.append(&material_quantity_per_pack_input);

    let material_price_per_pack_adjustment =
        gtk4::Adjustment::new(0.0, 0.0, 1000.0, 0.01, 5.0, 0.0);
    let material_price_per_pack_input = gtk4::SpinButton::builder()
        .name(format!("{}_price_per_pack", &material.name))
        .hexpand(true)
        .margin_start(5)
        .adjustment(&material_price_per_pack_adjustment)
        .climb_rate(0.5)
        .numeric(true)
        .digits(2)
        .value(material.price_per_pack)
        .build();
    material_costs_box.append(&material_price_per_pack_input);

    let material_remove = gtk4::Button::builder()
        .icon_name("list-remove-symbolic")
        .halign(gtk4::Align::End)
        .valign(gtk4::Align::End)
        .margin_start(5)
        .margin_end(5)
        .build();
    // TODO: Need to move material_costs_entries into a centralised place, so updates can be worked upon
    let material_costs_entries_remove = material_costs_entries.clone();
    let material_name = GString::from_string_unchecked(material.name.to_string());
    material_remove.connect_clicked(clone!(
        #[strong]
        material_costs_list_box,
        #[strong]
        material_costs_list_child,
        move |_| {
            let mut material_entries = material_costs_entries_remove.lock().unwrap();
            let row = material_entries
                .iter()
                .position(|entry| entry.0.text() == material_name)
                .unwrap();
            material_entries.remove(row);
            material_costs_list_box.remove(&material_costs_list_child);
        }
    ));
    material_costs_box.append(&material_remove);

    let mut material_entries = material_costs_entries.lock().unwrap();
    material_entries.push((
        material_label,
        material_quantity_per_pack_input,
        material_price_per_pack_input,
    ));
}
