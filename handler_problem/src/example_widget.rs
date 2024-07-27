
use gtk::{gio, glib::{self, Object}};

glib::wrapper! {
  pub struct ExampleWidget(ObjectSubclass<imp::ExampleWidget>)
    @extends gtk::Widget,
    @implements gio::ActionGroup, gio::ActionMap,
                gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
                gtk::Native, gtk::Root, gtk::ShortcutManager;
}

mod imp {

use std::sync::atomic::{self, AtomicBool};

use gtk::{glib, template_callbacks, BinLayout, CompositeTemplate};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use glib::subclass::InitializingObject;

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/example/widget.ui")]
pub struct ExampleWidget {
  #[template_child]
  button : TemplateChild<gtk::Button>,
  #[template_child]
  entry : TemplateChild<gtk::Entry>,
  constructed : AtomicBool,
}

#[glib::object_subclass]
impl ObjectSubclass for ExampleWidget {
  const NAME: &'static str = "ExampleWidget";
  type Type = super::ExampleWidget;
  type ParentType = gtk::Widget;

  fn class_init(klass: &mut Self::Class) {
    klass.bind_template();
    klass.bind_template_callbacks();
    klass.set_layout_manager_type::<BinLayout>();
  }

  fn instance_init(obj: &InitializingObject<Self>) {
    obj.init_template();
  }
}

impl ObjectImpl for ExampleWidget {
  fn constructed(&self) {
    self.entry.set_text("text");

    self.constructed.store(true, atomic::Ordering::SeqCst);

    self.parent_constructed();
  }

  fn dispose(&self) {
    let obj = self.obj();
    while let Some(child) = obj.first_child() {
      child.unparent();
    }
    self.dispose_template();
  }
}

impl WidgetImpl for ExampleWidget {}

#[template_callbacks]
impl ExampleWidget {
  #[template_callback]
  fn button_clicked(&self, _button : gtk::Button) {
    println!("button_clicked/start: {:?}", self.entry.text());

    self.entry.set_text("text v2");

    println!("button_clicked/end: {:?}", self.entry.text());
  }

  #[template_callback]
  fn check_text(&self, _obj : glib::Object) {
    if !self.constructed.load(atomic::Ordering::SeqCst) { return; }

    println!("check_text/start: {:?}", self.entry.text());

    let text = self.entry.text();
    if text.as_str().is_empty() {
      panic!("editable text was empty, but shouldn't be");
    }

    println!("check_text/end: {:?}", self.entry.text());
  }
}

}

impl ExampleWidget {
  pub fn new() -> Self {
    Object::builder().build()
  }
}