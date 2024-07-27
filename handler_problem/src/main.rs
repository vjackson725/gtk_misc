
mod example_widget;

use example_widget::ExampleWidget;

use gtk::{gio, glib, Application, ApplicationWindow};
use gtk::prelude::*;

fn main() -> glib::ExitCode {
  gio::resources_register_include!("example.gresource")
    .expect("Failed to register resources.");

  let _ = gtk::init();

  let widget = ExampleWidget::new();

  let app = Application::builder().build();
  app.connect_activate(move |app| {
    let window = {
      ApplicationWindow::builder()
        .application(app)
        .build()
    };
    window.set_child(Some(&widget));
    window.present();
  });
  app.run()
}
