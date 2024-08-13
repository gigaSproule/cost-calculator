use adw::ApplicationWindow;
use gtk4::{glib::clone, prelude::*};

use crate::store::config::{self, get_config, ConfigImpl};

pub(crate) fn options(window: &ApplicationWindow) -> gtk4::Grid {
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
        .adjustment(&markup_percentage_adjustment)
        .climb_rate(0.5)
        .numeric(true)
        .digits(2)
        .build();
    markup_percentage_input.set_value(existing_config.markup_percentage);
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
        .adjustment(&hourly_adjustment)
        .climb_rate(0.5)
        .numeric(true)
        .digits(2)
        .build();
    hourly_rate_input.set_value(existing_config.hourly_rate);
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
        .adjustment(&tax_rate_adjustment)
        .climb_rate(0.5)
        .numeric(true)
        .digits(2)
        .build();
    tax_rate_input.set_value(existing_config.tax_rate);
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
        .adjustment(&vat_adjustment)
        .climb_rate(0.5)
        .numeric(true)
        .digits(2)
        .build();
    vat_input.set_value(existing_config.vat);
    container.attach(&vat_input, 1, 3, 1, 1);

    let currency_label = gtk4::Label::builder()
        .label("Currency")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&currency_label, 0, 4, 1, 1);
    let currency_input = gtk4::Entry::builder()
        .name("currency")
        .hexpand(true)
        .max_length(1)
        .build();
    currency_input.set_text(&existing_config.currency);
    container.attach(&currency_input, 1, 4, 1, 1);

    let save = gtk4::Button::builder()
        .label("Save")
        .halign(gtk4::Align::End)
        .valign(gtk4::Align::Center)
        .build();
    save.connect_clicked(clone!(
        #[strong]
        markup_percentage_input,
        #[strong]
        hourly_rate_input,
        #[strong]
        tax_rate_input,
        #[strong]
        vat_input,
        #[strong]
        window,
        move |_| {
            let config = ConfigImpl {
                markup_percentage: markup_percentage_input.value(),
                hourly_rate: hourly_rate_input.value(),
                tax_rate: tax_rate_input.value(),
                vat: vat_input.value(),
                currency: currency_input.text().to_string(),
            };
            config::store_config(&config);

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
    container.attach(&save, 1, 5, 1, 1);
    container
}
