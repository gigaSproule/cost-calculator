use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use gtk4::{glib::clone, prelude::*};

use crate::{store, store::materials::get_materials, store::materials::Material};

pub(crate) fn materials() -> gtk4::Grid {
    let mut existing_materials = get_materials();
    if existing_materials.len() == 0 {
        existing_materials = vec![Material {
            name: String::from("Material"),
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

    let material_costs_container_rows: Rc<RefCell<i32>> = Rc::new(RefCell::new(1));
    let material_costs_entries: Arc<Mutex<Vec<(gtk4::EditableLabel, gtk4::SpinButton)>>> =
        Arc::new(Mutex::new(vec![]));

    for material in existing_materials {
        let mut rows = material_costs_container_rows.borrow_mut();
        let material_costs_label = gtk4::EditableLabel::builder()
            .text(&material.name)
            .halign(gtk4::Align::Start)
            .valign(gtk4::Align::Center)
            .build();
        material_costs_container.attach(&material_costs_label, 0, *rows, 1, 1);
        let material_costs_adjustment = gtk4::Adjustment::new(0.0, 0.0, 1000.0, 0.01, 5.0, 0.0);
        let material_costs_input = gtk4::SpinButton::builder()
            .name(format!("{}_costs", &material.name))
            .hexpand(true)
            .adjustment(&material_costs_adjustment)
            .climb_rate(0.5)
            .numeric(true)
            .digits(2)
            .value(material.value)
            .build();
        material_costs_container.attach(&material_costs_input, 1, *rows, 1, 1);
        let mut material_entries = material_costs_entries.lock().unwrap();
        material_entries.push((material_costs_label, material_costs_input));
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
            let material_costs_adjustment = gtk4::Adjustment::new(0.0, 0.0, 1000.0, 0.01, 5.0, 0.0);
            let material_costs_input = gtk4::SpinButton::builder()
                .name("material_costs")
                .hexpand(true)
                .adjustment(&material_costs_adjustment)
                .climb_rate(0.5)
                .numeric(true)
                .digits(2)
                .build();
            material_costs_container.attach(&material_costs_input, 1, *rows, 1, 1);
            let mut material_entries = material_costs_entries_add.lock().unwrap();
            material_entries.push((material_costs_label, material_costs_input));
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
            let materials: Vec<Material> = material_entries.iter().map(|(label, entry)| {Material {name: label.text().to_string(), value: entry.value()}}).collect();
            store::materials::store_materials(materials);
        }
    ));

    container.attach(&save, 1, 4, 1, 1);
    container
}
