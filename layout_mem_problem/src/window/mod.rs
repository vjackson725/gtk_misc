
mod imp;

use gtk::{glib, gio, Application};
use gtk::prelude::*;

use glib::Object;

//

glib::wrapper! {
  pub struct VjjTestWindow(ObjectSubclass<imp::VjjTestWindow>)
    @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl VjjTestWindow {
  pub fn new(app: &Application) -> Self {
    Object::builder()
      .property("application", app)
      .build()
  }
}