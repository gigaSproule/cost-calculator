use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use gtk4::{glib::clone, prelude::*};

use crate::{store, store::materials::get_materials, store::materials::StoredMaterial};

pub(crate) fn materials() -> gtk4::Grid {
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
        .margin_top(5)
        .margin_bottom(5)
        .row_spacing(5)
        .column_spacing(5)
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

    let material_costs_container_rows: Rc<RefCell<i32>> = Rc::new(RefCell::new(1));
    let material_costs_entries: Arc<
        Mutex<Vec<(gtk4::EditableLabel, gtk4::SpinButton, gtk4::SpinButton)>>,
    > = Arc::new(Mutex::new(vec![]));

    for material in existing_materials {
        let mut rows = material_costs_container_rows.borrow_mut();
        let material_label = gtk4::EditableLabel::builder()
            .text(&material.name)
            .halign(gtk4::Align::Start)
            .valign(gtk4::Align::Center)
            .build();
        material_costs_container.attach(&material_label, 0, *rows, 1, 1);
        let material_quantity_per_pack_adjustment =
            gtk4::Adjustment::new(0.0, 0.0, 1000.0, 1.0, 5.0, 0.0);
        let material_quantity_per_pack_input = gtk4::SpinButton::builder()
            .name(format!("{}_quantity_per_pack", &material.name))
            .hexpand(true)
            .adjustment(&material_quantity_per_pack_adjustment)
            .climb_rate(1.0)
            .numeric(true)
            .digits(0)
            .value(material.value)
            .build();
        material_costs_container.attach(&material_quantity_per_pack_input, 1, *rows, 1, 1);
        let material_price_per_pack_adjustment =
            gtk4::Adjustment::new(0.0, 0.0, 1000.0, 0.01, 5.0, 0.0);
        let material_price_per_pack_input = gtk4::SpinButton::builder()
            .name(format!("{}_price_per_pack", &material.name))
            .hexpand(true)
            .adjustment(&material_price_per_pack_adjustment)
            .climb_rate(0.5)
            .numeric(true)
            .digits(2)
            .value(material.value)
            .build();
        material_costs_container.attach(&material_price_per_pack_input, 2, *rows, 1, 1);
        let mut material_entries = material_costs_entries.lock().unwrap();
        material_entries.push((
            material_label,
            material_quantity_per_pack_input,
            material_price_per_pack_input,
        ));
        *rows = *rows + 1;
    }

    let add_material_costs = gtk4::Button::builder()
        .icon_name("list-add-symbolic")
        .halign(gtk4::Align::End)
        .valign(gtk4::Align::End)
        .build();
    material_costs_wrapper.attach(&add_material_costs, 1, 1, 1, 1);

    let material_costs_entries_add = material_costs_entries.clone();
    add_material_costs.connect_clicked(clone!(@strong material_costs_container, @strong material_costs_container_rows =>
        move |_| {
            let material_costs_label = gtk4::EditableLabel::builder()
                .text("Material")
                .halign(gtk4::Align::Start)
                .valign(gtk4::Align::Center)
                .build();
            let mut rows = material_costs_container_rows.borrow_mut();
            material_costs_container.attach(&material_costs_label, 0, *rows, 1, 1);
            let material_quantity_per_pack_adjustment = gtk4::Adjustment::new(0.0, 0.0, 1000.0, 1.0, 5.0, 0.0);
            let material_quantity_per_pack_input = gtk4::SpinButton::builder()
                .name("material_quantity_per_pack")
                .hexpand(true)
                .adjustment(&material_quantity_per_pack_adjustment)
                .climb_rate(1.0)
                .numeric(true)
                .digits(0)
                .build();
            material_costs_container.attach(&material_quantity_per_pack_input, 1, *rows, 1, 1);
            let material_price_per_pack_adjustment = gtk4::Adjustment::new(0.0, 0.0, 1000.0, 0.01, 5.0, 0.0);
            let material_price_per_pack_input = gtk4::SpinButton::builder()
                .name("material_price_per_pack")
                .hexpand(true)
                .adjustment(&material_price_per_pack_adjustment)
                .climb_rate(0.5)
                .numeric(true)
                .digits(2)
                .build();
            material_costs_container.attach(&material_price_per_pack_input, 2, *rows, 1, 1);
            let mut material_entries = material_costs_entries_add.lock().unwrap();
            material_entries.push((material_costs_label, material_quantity_per_pack_input, material_price_per_pack_input));
            *rows = *rows + 1;
        }
    ));

    let save = gtk4::Button::builder()
        .label("Save")
        .halign(gtk4::Align::End)
        .valign(gtk4::Align::Center)
        .build();

    let material_costs_entries_calculate = material_costs_entries.clone();
    save.connect_clicked(clone!(@strong material_costs_container_rows =>
        move |_| {
            let material_entries = material_costs_entries_calculate.lock().unwrap();
            let materials: Vec<StoredMaterial> = material_entries.iter().map(|(label, quantity_per_pack, price_per_pack)| {
                StoredMaterial {
                    name: label.text().to_string(),
                    quantity_per_pack: quantity_per_pack.value(),
                    price_per_pack: price_per_pack.value(),
                    value: quantity_per_pack.value() / price_per_pack.value(),
                }
            }).collect();
            store::materials::store_materials(materials);
        }
    ));

    container.attach(&save, 1, 4, 1, 1);
    container
}
