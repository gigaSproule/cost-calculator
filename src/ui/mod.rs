use gtk4::prelude::*;

mod etsy_ui;
mod options_ui;
mod paypal_ui;
mod shopify_ui;
mod stripe_ui;

pub(crate) fn load_ui() {
    let application = gtk4::Application::builder()
        .application_id("com.benjaminsproule.jackie-cost-calculator")
        .build();
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::builder()
        .application(application)
        .title("Main Menu")
        .default_width(350)
        .default_height(70)
        .build();

    let container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    window.set_child(Some(&container));

    let stack = gtk4::Stack::new();
    stack.set_transition_type(gtk4::StackTransitionType::SlideLeftRight);
    stack.set_transition_duration(200);

    let side_stack = gtk4::StackSidebar::new();
    side_stack.set_stack(&stack);

    let etsy_options = etsy_ui::etsy_options();
    stack.add_titled(&etsy_options, Option::<&str>::None, "Etsy");

    let paypal_title = paypal_ui::paypal_options();
    stack.add_titled(&paypal_title, Option::<&str>::None, "PayPal");

    let shopify_title = shopify_ui::shopify_options();
    stack.add_titled(&shopify_title, Option::<&str>::None, "Shopify");

    let stripe_title = stripe_ui::stripe_options();
    stack.add_titled(&stripe_title, Option::<&str>::None, "Stripe");

    let options_title = options_ui::options();
    stack.add_titled(&options_title, Option::<&str>::None, "Options");

    container.append(&side_stack);
    container.append(&stack);

    window.present();
}
