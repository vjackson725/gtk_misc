
use gtk::prelude::*;
use gtk::{gio, glib};

mod window;

use window::VjjTestWindow;

//

const APP_ID: &str = "vjj.GtkTest";

fn main() -> glib::ExitCode {
  gio::resources_register_include!("test.gresource")
    .expect("Failed to register resources.");

  let app = gtk::Application::builder().application_id(APP_ID).build();
  app.connect_activate(|app| {
    let window = VjjTestWindow::new(app);
    window.present();
  });

  app.run()
}
