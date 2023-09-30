use gtk4::{glib::clone, prelude::*};

use crate::config::{self, get_config, Config};

pub(crate) fn options() -> gtk4::Grid {
    let existing_config = get_config();

    let container = gtk4::Grid::builder()
        .margin_start(5)
        .margin_end(5)
        .margin_top(5)
        .margin_bottom(5)
        .row_spacing(5)
        .column_spacing(5)
        .build();

    let markup_percentage_label = gtk4::Label::builder()
        .label("Markup percentage")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&markup_percentage_label, 0, 0, 1, 1);
    let markup_percentage_adjustment = gtk4::Adjustment::new(0.0, 0.0, 100.0, 0.01, 5.0, 0.0);
    let markup_percentage_input = gtk4::SpinButton::builder()
        .name("markup_percentage")
        .hexpand(true)
        .value(existing_config.markup_percentage)
        .adjustment(&markup_percentage_adjustment)
        .climb_rate(0.5)
        .numeric(true)
        .digits(2)
        .build();
    container.attach(&markup_percentage_input, 1, 0, 1, 1);

    let hourly_rate_label = gtk4::Label::builder()
        .label("Hourly rate")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&hourly_rate_label, 0, 1, 1, 1);
    let hourly_adjustment = gtk4::Adjustment::new(0.0, 0.0, 100.0, 0.01, 5.0, 0.0);
    let hourly_rate_input = gtk4::SpinButton::builder()
        .name("hourly_rate")
        .hexpand(true)
        .value(existing_config.hourly_rate)
        .adjustment(&hourly_adjustment)
        .climb_rate(0.5)
        .numeric(true)
        .digits(2)
        .build();
    container.attach(&hourly_rate_input, 1, 1, 1, 1);

    let tax_rate_label = gtk4::Label::builder()
        .label("Tax rate")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&tax_rate_label, 0, 2, 1, 1);
    let tax_rate_adjustment = gtk4::Adjustment::new(0.0, 0.0, 100.0, 0.01, 5.0, 0.0);
    let tax_rate_input = gtk4::SpinButton::builder()
        .name("tax_rate")
        .hexpand(true)
        .value(existing_config.tax_rate)
        .adjustment(&tax_rate_adjustment)
        .climb_rate(0.5)
        .numeric(true)
        .digits(2)
        .build();
    container.attach(&tax_rate_input, 1, 2, 1, 1);

    let vat_label = gtk4::Label::builder()
        .label("VAT")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&vat_label, 0, 3, 1, 1);
    let vat_adjustment = gtk4::Adjustment::new(0.0, 0.0, 100.0, 0.01, 5.0, 0.0);
    let vat_input = gtk4::SpinButton::builder()
        .name("vat")
        .hexpand(true)
        .value(existing_config.vat)
        .adjustment(&vat_adjustment)
        .climb_rate(0.5)
        .numeric(true)
        .digits(2)
        .build();
    container.attach(&vat_input, 1, 3, 1, 1);

    let save = gtk4::Button::builder()
        .label("Save")
        .halign(gtk4::Align::End)
        .valign(gtk4::Align::Center)
        .build();
    save.connect_clicked(
        clone!(@strong markup_percentage_input, @strong hourly_rate_input, @strong tax_rate_input, @strong vat_input =>
            move |_| {
                let config = Config {
                    markup_percentage: markup_percentage_input.value(),
                    hourly_rate: hourly_rate_input.value(),
                    tax_rate: tax_rate_input.value(),
                    vat: vat_input.value(),
                };
                config::store_config(&config);
            }
        ),
    );
    container.attach(&save, 1, 4, 1, 1);
    container
}
