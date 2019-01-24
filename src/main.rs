#[macro_use]
extern crate rust_embed;
extern crate sciter;

#[derive(RustEmbed)]
#[folder = "src/assets/"]
struct Asset;


use sciter::Window;

fn main() {
  let mut frame: Window = sciter::Window::new();

  let index_html = Asset::get("minimal.htm").unwrap();

  frame.load_html(index_html.as_ref(), None);
  frame.run_app();
}
