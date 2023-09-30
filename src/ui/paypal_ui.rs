use gtk4::{glib::clone, prelude::*};

use crate::calculator::paypal_calculator;

pub(crate) fn paypal_options() -> gtk4::Box {
    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    let stack = gtk4::Stack::builder()
        .transition_type(gtk4::StackTransitionType::SlideLeftRight)
        .transition_duration(200)
        .build();

    let stack_switcher = gtk4::StackSwitcher::builder().hexpand(true).build();
    stack_switcher.set_stack(Some(&stack));

    let cost_of_sale = cost_of_sale();
    stack.add_titled(&cost_of_sale, Option::<&str>::None, "Cost of sale");

    let how_much_to_charge = how_much_to_charge();
    stack.add_titled(
        &how_much_to_charge,
        Option::<&str>::None,
        "How to much to charge",
    );

    container.append(&stack_switcher);
    container.append(&stack);
    container
}

fn cost_of_sale() -> gtk4::Grid {
    let container = gtk4::Grid::builder()
        .margin_start(5)
        .margin_end(5)
        .margin_top(5)
        .margin_bottom(5)
        .row_spacing(5)
        .column_spacing(5)
        .build();

    let cost_of_sale_label = gtk4::Label::builder()
        .label("Cost of sale")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&cost_of_sale_label, 0, 0, 1, 1);
    let cost_of_sale_adjustment = gtk4::Adjustment::new(0.0, 0.0, 1000.0, 0.01, 5.0, 0.0);
    let cost_of_sale_input = gtk4::SpinButton::builder()
        .name("cost_of_sale")
        .hexpand(true)
        .adjustment(&cost_of_sale_adjustment)
        .climb_rate(0.5)
        .numeric(true)
        .digits(2)
        .build();
    container.attach(&cost_of_sale_input, 1, 0, 1, 1);

    let cost_of_delivery_label = gtk4::Label::builder()
        .label("Cost of delivery")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&cost_of_delivery_label, 0, 1, 1, 1);
    let cost_of_delivery_adjustment = gtk4::Adjustment::new(0.0, 0.0, 1000.0, 0.01, 5.0, 0.0);
    let cost_of_delivery_input = gtk4::SpinButton::builder()
        .name("cost_of_delivery")
        .hexpand(true)
        .adjustment(&cost_of_delivery_adjustment)
        .climb_rate(0.5)
        .numeric(true)
        .digits(2)
        .build();
    container.attach(&cost_of_delivery_input, 1, 1, 1, 1);

    let location_label = gtk4::Label::builder()
        .label("Location of sale")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&location_label, 0, 2, 1, 1);
    let location_input = gtk4::ComboBoxText::builder()
        .name("offsite_ads_used")
        .build();
    location_input.append(Some("local"), "Local");
    location_input.append(Some("eea"), "EEA");
    location_input.append(Some("international"), "International");
    location_input.set_active_id(Some("local"));
    container.attach(&location_input, 1, 2, 1, 1);

    let calculate = gtk4::Button::builder()
        .label("Calculate")
        .halign(gtk4::Align::End)
        .valign(gtk4::Align::Center)
        .build();
    calculate.connect_clicked(
        clone!(@strong cost_of_sale_input, @strong cost_of_delivery_input, @strong location_input =>
            move |_| {
                paypal_calculator::based_on_sale(
                    cost_of_sale_input.value(),
                    cost_of_delivery_input.value(),
                    location_input.active_id().unwrap() == "eea",
                    location_input.active_id().unwrap() == "lnternational",
                );
                cost_of_sale_input.set_value(0.0);
                cost_of_delivery_input.set_value(0.0);
                location_input.set_active_id(Some("local"));
            }
        ),
    );
    container.attach(&calculate, 1, 3, 1, 1);
    container
}

fn how_much_to_charge() -> gtk4::Grid {
    let container = gtk4::Grid::builder()
        .margin_start(5)
        .margin_end(5)
        .margin_top(5)
        .margin_bottom(5)
        .row_spacing(5)
        .column_spacing(5)
        .build();

    let minutes_label = gtk4::Label::builder()
        .label("Time taken")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&minutes_label, 0, 0, 1, 1);
    let minutes_adjustment = gtk4::Adjustment::new(0.0, 0.0, 10000.0, 1.0, 5.0, 0.0);
    let minutes_input = gtk4::SpinButton::builder()
        .name("minutes")
        .hexpand(true)
        .adjustment(&minutes_adjustment)
        .build();
    minutes_input.connect_input(|input| {
        let text = input.text();
        let split = text.split(':').collect::<Vec<&str>>();
        if split.len() != 2 {
            return None;
        }

        let hours = split.first().unwrap().parse::<f64>().unwrap();
        let minutes = split.last().unwrap().parse::<f64>().unwrap();
        Some(Ok((hours * 60.0) + minutes))
    });
    minutes_input.connect_output(|input| {
        let hours = input.adjustment().value() / 60.0;
        let minutes = (hours - hours.floor()) * 60.0;
        let text = format!("{:02}:{:02}", hours.floor(), minutes.floor());
        input.set_text(&text);
        gtk4::glib::Propagation::Stop
    });
    container.attach(&minutes_input, 1, 0, 1, 1);

    let material_costs_label = gtk4::Label::builder()
        .label("Cost of materials")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&material_costs_label, 0, 1, 1, 1);
    let material_costs_adjustment = gtk4::Adjustment::new(0.0, 0.0, 1000.0, 0.01, 5.0, 0.0);
    let material_costs_input = gtk4::SpinButton::builder()
        .name("material_costs")
        .hexpand(true)
        .adjustment(&material_costs_adjustment)
        .climb_rate(0.5)
        .numeric(true)
        .digits(2)
        .build();
    container.attach(&material_costs_input, 1, 1, 1, 1);

    let cost_of_delivery_label = gtk4::Label::builder()
        .label("Cost of delivery")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&cost_of_delivery_label, 0, 2, 1, 1);
    let cost_of_delivery_adjustment = gtk4::Adjustment::new(0.0, 0.0, 1000.0, 0.01, 5.0, 0.0);
    let cost_of_delivery_input = gtk4::SpinButton::builder()
        .name("cost_of_delivery")
        .hexpand(true)
        .adjustment(&cost_of_delivery_adjustment)
        .climb_rate(0.5)
        .numeric(true)
        .digits(2)
        .build();
    container.attach(&cost_of_delivery_input, 1, 2, 1, 1);

    let location_label = gtk4::Label::builder()
        .label("Location of sale")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&location_label, 0, 3, 1, 1);
    let location_input = gtk4::ComboBoxText::builder()
        .name("offsite_ads_used")
        .build();
    location_input.append(Some("local"), "Local");
    location_input.append(Some("eea"), "EEA");
    location_input.append(Some("international"), "International");
    location_input.set_active_id(Some("local"));
    container.attach(&location_input, 1, 3, 1, 1);

    let calculate = gtk4::Button::builder()
        .label("Calculate")
        .halign(gtk4::Align::End)
        .valign(gtk4::Align::End)
        .build();
    calculate.connect_clicked(
        clone!(@strong minutes_input, @strong material_costs_input, @strong cost_of_delivery_input, @strong location_input =>
            move |_| {
                paypal_calculator::how_much_to_charge(
                    minutes_input.value(),
                    material_costs_input.value(),
                    cost_of_delivery_input.value(),
                    location_input.active_id().unwrap() == "eea",
                    location_input.active_id().unwrap() == "international",
                );
                minutes_input.set_value(0.0);
                material_costs_input.set_value(0.0);
                cost_of_delivery_input.set_value(0.0);
                location_input.set_active_id(Some("local"));
            }
        ),
    );
    container.attach(&calculate, 1, 4, 1, 1);
    container
}
