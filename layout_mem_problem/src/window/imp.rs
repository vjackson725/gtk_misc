
use gtk::{CompositeTemplate, ConstraintLayout, Constraint, ConstraintAttribute,
  ConstraintRelation, Widget };
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use glib::subclass::InitializingObject;


#[derive(CompositeTemplate, Default)]
#[template(resource = "/vjj/GtkTest/window.ui")]
pub struct VjjTestWindow {}

#[glib::object_subclass]
impl ObjectSubclass for VjjTestWindow {
  const NAME: &'static str = "VjjTestWindow";
  type Type = super::VjjTestWindow;
  type ParentType = gtk::ApplicationWindow;

  fn class_init(klass: &mut Self::Class) {
    klass.bind_template();
  }

  fn instance_init(obj: &InitializingObject<Self>) {
    obj.init_template();
  }
}

impl ObjectImpl for VjjTestWindow {
  fn constructed(&self) {
    let objref = self.obj();
    let obj = objref.as_ref();

    let layout = ConstraintLayout::new();
    obj.set_layout_manager(Some(layout));

    self.parent_constructed();
  }

  // fn dispose(&self) {
  //   let obj = self.obj();

  //   let mmanager  : Option<ConstraintLayout> = {
  //     obj.layout_manager().and_then(|m| m.downcast().ok())
  //   };
  //   obj.set_layout_manager(None as Option<ConstraintLayout>);
  //   if let Some(manager) = mmanager {
  //     manager.remove_all_constraints();
  //   }

  //   while let Some(child) = obj.first_child() {
  //     child.unparent();
  //   }
  // }
}

impl WidgetImpl for VjjTestWindow {
  fn realize(&self) {
    self.parent_realize();

    let obj = self.obj();
    let win_obj : &gtk::Window = obj.upcast_ref();
    win_obj.set_default_size(800, 600);

    let objref = self.obj();
    let obj = objref.as_ref();

    let layout : ConstraintLayout = {
      obj.layout_manager().expect("must have layout manager, by construction")
        .downcast().expect("must be a ConstraintLayout")
    };

    let main_child : Widget = {
      match obj.first_child() {
        None => {
          eprintln!("WARN: no first child");
          return;
        }
        Some(w) => w,
      }
    };

    // Safety constraints
    layout.add_constraint(Constraint::new(
      Some(&main_child), ConstraintAttribute::Right,
      ConstraintRelation::Le,
      Some(obj), ConstraintAttribute::Right,
      1.0, 0.0, 1000,
    ));
    layout.add_constraint(Constraint::new(
      Some(&main_child), ConstraintAttribute::Top,
      ConstraintRelation::Ge,
      Some(obj), ConstraintAttribute::Top,
      1.0, 0.0, 1000,
    ));
    layout.add_constraint(Constraint::new(
      Some(&main_child), ConstraintAttribute::Bottom,
      ConstraintRelation::Le,
      Some(obj), ConstraintAttribute::Bottom,
      1.0, 0.0, 1000,
    ));
    layout.add_constraint(Constraint::new(
      Some(&main_child), ConstraintAttribute::Top,
      ConstraintRelation::Ge,
      Some(obj), ConstraintAttribute::Top,
      1.0, 0.0, 1000,
    ));
  }
}

impl WindowImpl for VjjTestWindow {}
impl ApplicationWindowImpl for VjjTestWindow {}
