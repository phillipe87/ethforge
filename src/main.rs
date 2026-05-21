//mod packet;


use fltk::{app, prelude::*,
           widget::*,
           window::Window,
           browser::CheckBrowser,
           menu::Choice,
           frame::Frame
          };

fn main() {

  // create app
  let app = app::App::default();

  let mut window = Window::new(100, 100, 400, 300, "ethforge");

  Frame::new(20, 20, 200, 30, "EtherType pool:");

  // CheckBrowser — x, y, width, height, label
  let mut browser = CheckBrowser::new(20, 55, 200, 150, "");

  // add() takes a label and a checked state (true/false)
  browser.add("IPv4  (0x0800)", false);
  browser.add("ARP   (0x0806)", false);
  browser.add("IPv6  (0x86DD)", false);
  browser.add("VLAN  (0x8100)", false);

  // check the first item by default
  browser.set_checked(1);

  window.end();
  window.make_resizable(true);
  window.resizable(&browser);
  window.show();

  // run
  app.run().unwrap();

}
