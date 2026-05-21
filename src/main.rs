/*
  main.rs
 */

//mod packet;


use fltk::{
  app,
  prelude::*,
  enums::*,
  frame::Frame,
  group::{Group, Scroll},
  menu::MenuBar,
  window::Window,
};



// Layout constants
const WIN_W: i32 = 960;
const WIN_H: i32 = 700;

const MENU_H:   i32 = 30;   // menu bar height
const STATUS_H: i32 = 25;   // status bar height
const IFACE_W:  i32 = 160;  // interface panel width
const PREVIEW_W: i32 = 280; // hex preview panel width

// Derived constants calculated from the above
const CONTENT_Y: i32 = MENU_H;                          // content area starts below menu
const CONTENT_H: i32 = WIN_H - MENU_H - STATUS_H;       // content area height
const BUILDER_X: i32 = IFACE_W;                         // builder starts after iface panel
const BUILDER_W: i32 = WIN_W - IFACE_W - PREVIEW_W;     // builder takes remaining width
const PREVIEW_X: i32 = WIN_W - PREVIEW_W;               // preview flush to right edge
const STATUS_Y:  i32 = WIN_H - STATUS_H;                // status bar at bottom

fn main() {

  // create app
  let app = app::App::default();

  let mut window = Window::new(0, 0, WIN_W, WIN_H, "ethforge");

  window.make_resizable(true);


  // menu bar _________________________________________________________________
  let mut menu = MenuBar::new(0,0,WIN_W, WIN_H, "");
  menu.add_choice("File/Save");
  menu.add_choice("File/Load");

  // interface panel __________________________________________________________
  // group container
  let mut iface_group = Group::new(0, CONTENT_Y, IFACE_W, CONTENT_H, "");
  iface_group.set_frame(FrameType::DownBox);


  Frame::new(0, CONTENT_Y,IFACE_W, 25, "Interface")
    .set_align(Align::Center);

  iface_group.end();

  // packet builder panel _____________________________________________________
  let mut builder_scroll = Scroll::new(BUILDER_X, CONTENT_Y,BUILDER_W,CONTENT_H, "");
  builder_scroll.set_frame(FrameType::DownBox);

  Frame::new(BUILDER_X, CONTENT_Y, BUILDER_W, 25, "Packet Builder")
    .set_align(Align::Center);

  builder_scroll.end();

  // hex preview panel ________________________________________________________
  let mut preview_group = Group::new(PREVIEW_X, CONTENT_Y, PREVIEW_W, CONTENT_H, "");
  preview_group.set_frame(FrameType::DownBox);

  Frame::new(PREVIEW_X, CONTENT_Y, PREVIEW_W, 25, "Hex Preview")
    .set_align(Align::Center);

  // status bar _______________________________________________________________
  let mut status = Frame::new(0, STATUS_Y, WIN_W, STATUS_H, "Ready");
  status.set_frame(FrameType::DownBox);
  status.set_align(Align::Left | Align::Inside);

  window.end();
  window.resizable(&window);
  window.show();

  // run
  app.run().unwrap();

}
