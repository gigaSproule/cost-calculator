use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use gtk4::{glib::clone, prelude::*, Align};

use crate::{
    calculator::shopify_calculator, calculator::Material, store::config::get_config,
    store::materials::get_materials,
};

pub(crate) fn shopify_options() -> gtk4::Box {
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

    let international_amex_label = gtk4::Label::builder()
        .label("International/AmEx?")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&international_amex_label, 0, 2, 1, 1);
    let international_amex_input = gtk4::CheckButton::builder()
        .name("international_amex")
        .build();
    container.attach(&international_amex_input, 1, 2, 1, 1);

    let calculate = gtk4::Button::builder()
        .label("Calculate")
        .halign(gtk4::Align::End)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&calculate, 1, 3, 1, 1);

    let sale_label = gtk4::Label::builder()
        .label("Sale")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&sale_label, 0, 5, 1, 1);
    let sale_value = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&sale_value, 1, 5, 1, 1);

    let delivery_costs_label = gtk4::Label::builder()
        .label("Delivery costs")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&delivery_costs_label, 0, 6, 1, 1);
    let delivery_costs_value = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&delivery_costs_value, 1, 6, 1, 1);

    let transaction_cost_label = gtk4::Label::builder()
        .label("Transaction cost")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&transaction_cost_label, 0, 7, 1, 1);
    let transaction_cost_value = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&transaction_cost_value, 1, 7, 1, 1);

    let payment_processing_cost_label = gtk4::Label::builder()
        .label("Payment processing cost")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&payment_processing_cost_label, 0, 8, 1, 1);
    let payment_processing_cost_value = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&payment_processing_cost_value, 1, 8, 1, 1);

    let offsite_ads_cost_label = gtk4::Label::builder()
        .label("Offsite ads cost")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&offsite_ads_cost_label, 0, 9, 1, 1);
    let offsite_ads_cost_value = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&offsite_ads_cost_value, 1, 9, 1, 1);

    let regulatory_operating_fee_label = gtk4::Label::builder()
        .label("Regulatory operating fee")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&regulatory_operating_fee_label, 0, 10, 1, 1);
    let regulatory_operating_fee_value = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&regulatory_operating_fee_value, 1, 10, 1, 1);

    let vat_paid_by_buyer_label = gtk4::Label::builder()
        .label("VAT paid by buyer")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&vat_paid_by_buyer_label, 0, 11, 1, 1);
    let vat_paid_by_buyer_value = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&vat_paid_by_buyer_value, 1, 11, 1, 1);

    let vat_on_seller_fees_label = gtk4::Label::builder()
        .label("VAT on seller fees")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&vat_on_seller_fees_label, 0, 12, 1, 1);
    let vat_on_seller_fees_value = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&vat_on_seller_fees_value, 1, 12, 1, 1);

    let total_fees_label = gtk4::Label::builder()
        .label("Total fees")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&total_fees_label, 0, 13, 1, 1);
    let total_fees_value = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&total_fees_value, 1, 13, 1, 1);

    let total_fees_with_vat_label = gtk4::Label::builder()
        .label("Total fees with VAT")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&total_fees_with_vat_label, 0, 14, 1, 1);
    let total_fees_with_vat_value = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&total_fees_with_vat_value, 1, 14, 1, 1);

    let tax_label = gtk4::Label::builder()
        .label("Tax")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&tax_label, 0, 15, 1, 1);
    let tax_value = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&tax_value, 1, 15, 1, 1);

    let revenue_label = gtk4::Label::builder()
        .label("Revenue")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&revenue_label, 0, 16, 1, 1);
    let revenue_value = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&revenue_value, 1, 16, 1, 1);

    let percentage_kept_label = gtk4::Label::builder()
        .label("Percentage kept")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&percentage_kept_label, 0, 17, 1, 1);
    let percentage_kept_value = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&percentage_kept_value, 1, 17, 1, 1);

    let max_working_hours_label = gtk4::Label::builder()
        .label("Max working hours")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&max_working_hours_label, 0, 18, 1, 1);
    let max_working_hours_value = gtk4::Label::builder()
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&max_working_hours_value, 1, 18, 1, 1);

    calculate.connect_clicked(
        clone!(@strong cost_of_sale_input, @strong cost_of_delivery_input, @strong international_amex_input, @strong sale_value,
            @strong delivery_costs_value, @strong transaction_cost_value, @strong payment_processing_cost_value, @strong offsite_ads_cost_value,
            @strong regulatory_operating_fee_value, @strong vat_paid_by_buyer_value, @strong vat_on_seller_fees_value, @strong total_fees_value,
            @strong total_fees_with_vat_value, @strong tax_value, @strong revenue_value, @strong percentage_kept_value, @strong max_working_hours_value =>
            move |_| {
                let config=get_config();
                let sale_breakdown = shopify_calculator::based_on_sale(
                    cost_of_sale_input.value(),
                    cost_of_delivery_input.value(),
                    international_amex_input.is_active(),
                );
                cost_of_sale_input.set_value(0.0);
                cost_of_delivery_input.set_value(0.0);
                international_amex_input.set_active(false);
                sale_value.set_text(&format!("{}{:.2}", config.currency, sale_breakdown.sale));
                delivery_costs_value.set_text(&format!("{}{:.2}", config.currency, sale_breakdown.delivery_costs));
                transaction_cost_value.set_text(&format!("{}{:.2}", config.currency, sale_breakdown.transaction_cost));
                payment_processing_cost_value.set_text(&format!("{}{:.2}", config.currency, sale_breakdown.payment_processing_cost));
                offsite_ads_cost_value.set_text(&format!("{}{:.2}", config.currency, sale_breakdown.offsite_ads_cost));
                regulatory_operating_fee_value.set_text(&format!("{}{:.2}", config.currency, sale_breakdown.regulatory_operating_fee));
                vat_paid_by_buyer_value.set_text(&format!("{}{:.2}", config.currency, sale_breakdown.vat_paid_by_buyer));
                vat_on_seller_fees_value.set_text(&format!("{}{:.2}", config.currency, sale_breakdown.vat_on_seller_fees));
                total_fees_value.set_text(&format!("{}{:.2}", config.currency, sale_breakdown.total_fees));
                total_fees_with_vat_value.set_text(&format!("{}{:.2}", config.currency, sale_breakdown.total_fees_with_vat));
                tax_value.set_text(&format!("{}{:.2}", config.currency, sale_breakdown.tax));
                revenue_value.set_text(&format!("{}{:.2}", config.currency, sale_breakdown.revenue));
                percentage_kept_value.set_text(&format!("{:.2}%", sale_breakdown.percentage_kept));
                max_working_hours_value.set_text(&format!("{}:{:02}", sale_breakdown.max_working_hours as i64, ((sale_breakdown.max_working_hours - ((sale_breakdown.max_working_hours as i64) as f64)) * 60.0) as i64));
            }
        ),
    );
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
    let material_costs_entries: Arc<Mutex<Vec<(gtk4::Label, gtk4::SpinButton, f64)>>> =
        Arc::new(Mutex::new(vec![]));

    let materials = get_materials();
    for material in materials {
        let material_costs_label = gtk4::Label::builder()
            .label(&material.name)
            .halign(Align::Start)
            .build();
        let mut rows = material_costs_container_rows.borrow_mut();
        material_costs_container.attach(&material_costs_label, 0, *rows, 1, 1);
        let material_costs_adjustment = gtk4::Adjustment::new(0.0, 0.0, 100.0, 1.0, 5.0, 0.0);
        let material_costs_input = gtk4::SpinButton::builder()
            .name("material_costs")
            .hexpand(true)
            .halign(Align::End)
            .adjustment(&material_costs_adjustment)
            .climb_rate(1.0)
            .numeric(true)
            .digits(0)
            .build();
        material_costs_container.attach(&material_costs_input, 1, *rows, 1, 1);
        let mut material_entries = material_costs_entries.lock().unwrap();
        material_entries.push((material_costs_label, material_costs_input, material.value));
        *rows = *rows + 1;
    }

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

    let international_amex_label = gtk4::Label::builder()
        .label("International/AmEx")
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .build();
    container.attach(&international_amex_label, 0, 3, 1, 1);
    let international_amex_input = gtk4::CheckButton::builder()
        .name("international_amex")
        .build();
    container.attach(&international_amex_input, 1, 3, 1, 1);

    let calculate = gtk4::Button::builder()
        .label("Calculate")
        .halign(gtk4::Align::End)
        .valign(gtk4::Align::End)
        .build();
    container.attach(&calculate, 1, 4, 1, 1);

    let answer_label = gtk4::Label::builder().halign(gtk4::Align::Start).build();
    container.attach(&answer_label, 0, 5, 2, 1);

    let material_costs_entries_calculate = material_costs_entries.clone();
    calculate.connect_clicked(
        clone!(@strong minutes_input, @strong material_costs_container, @strong material_costs_container_rows, @strong cost_of_delivery_input, @strong international_amex_input, @strong answer_label =>
            move |_| {
                let config = get_config();
                let material_entries = material_costs_entries_calculate.lock().unwrap();
                let materials: Vec<Material> = material_entries.iter()
                    .filter(|(_, entry, _)| {
                        entry.value() > 0.0
                    })
                    .map(|(label, spin_button, value)| {
                        Material { name: label.text().to_string(), value: spin_button.value() * value }
                    }).collect();
                let charge_amount = shopify_calculator::how_much_to_charge(
                    minutes_input.value(),
                    materials,
                    cost_of_delivery_input.value(),
                    international_amex_input.is_active(),
                );
                minutes_input.set_value(0.0);
                material_entries.iter().for_each(|(_, entry, _)| {
                    entry.set_value(0.0);
                });
                cost_of_delivery_input.set_value(0.0);
                international_amex_input.set_active(false);
                answer_label.set_text(&format!("Charge: {}{:.2} (with VAT {}{:.2})", config.currency, charge_amount.total_to_charge, config.currency, charge_amount.with_vat));
            }
        ),
    );
    container
}
