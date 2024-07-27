
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

use gtk::{glib, gio, template_callbacks, BinLayout, CompositeTemplate};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use glib::subclass::InitializingObject;

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/example/widget.ui")]
pub struct ExampleWidget {
  #[template_child]
  button : TemplateChild<gtk::Button>,
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
  fn button_clicked(&self, button : gtk::Button) {
    let root = {
      button.root()
        .expect("must have root")
        .dynamic_cast::<gtk::Window>()
        .expect("root must be a Window")
    };
    let dialog = {
      gtk::FileChooserNative::builder()
        .title("File Chooser")
        .transient_for(&root)
        .action(gtk::FileChooserAction::Open)
        .accept_label("_Open")
        .cancel_label("_Cancel")
        .modal(true)
        .build()
    };

    let cwd = gio::File::for_path(".");
    let _ = dialog.set_current_folder(Some(&cwd));
    dialog.show();
  }
}

}

impl ExampleWidget {
  pub fn new() -> Self {
    Object::builder().build()
  }
}