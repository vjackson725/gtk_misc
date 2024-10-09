
use glib_build_tools;

fn main() {
  glib_build_tools::compile_resources(
      &["res"],
      "res/resources.gresource.xml",
      "test.gresource",
  );
}