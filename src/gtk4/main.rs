use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let app = Application::new(Some("foo.bar.barfoo"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Super desktop app"));
    let btn = Button::with_label("the label - click here");
    btn.set_margin_top(15);
    btn.set_margin_bottom(15);
    btn.set_margin_start(15);
    btn.set_margin_end(15);

    btn.connect_clicked(move |btn| {
        btn.set_label("that works");
    });

    window.set_child(Some(&btn));
    window.present();
}

