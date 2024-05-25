use gtk::prelude::*;

pub fn apply_css() {
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(include_bytes!("style.css"))
        .expect("Failed to load CSS");
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
