//! app.rs - Main application window and widget layout
//!
//! This module contains all FLTK widget creation, layout constants,
//! and widget callbacks. Called from `main.rs` via [`run`].

use crate::iface;
use crate::packet;

use fltk::{
  app,
  browser::HoldBrowser,
  enums::*,
  frame::Frame,
  group::{Group, Scroll},
  input::Input,
  menu::{Choice, MenuBar},
  prelude::*,
  window::Window
};

// ------------------------------------
// LAYOUT CONSTANTS
// ------------------------------------
// Dimensions
const WIN_W: i32 = 960;
const WIN_H: i32 = 700;

const MENU_H:      i32 = 30;                           // menu bar height
const STATUS_H:    i32 = 25;                           // status bar height
const IFACE_W:     i32 = 160;                          // interface panel width
const PREVIEW_W:   i32 = 280;                          // hex preview panel width
const BUILDER_W:   i32 = WIN_W - IFACE_W - PREVIEW_W;  // builder takes remaining width
const IFACE_HDR_H: i32 = 25;                           // Height of interface panel heading

// Positions
const CONTENT_Y: i32 = MENU_H;                          // content area starts below menu
const CONTENT_H: i32 = WIN_H - MENU_H - STATUS_H;       // content area height
const BUILDER_X: i32 = IFACE_W;                         // builder starts after iface panel
const PREVIEW_X: i32 = WIN_W - PREVIEW_W;               // preview flush to right edge
const STATUS_Y:  i32 = WIN_H - STATUS_H;                // status bar at bottom

// Builder panel internal layout
const FIELD_H: i32 = 25;  // height of each input row
const LABEL_W: i32 = 80;  // width of field labels
const INPUT_W: i32 = 160; // width of input fields
const MARGIN:  i32 = 10;  // left margin inside builder panel
const HDR_H:   i32 = 25;  // section header height

/// Entry point for the GUI, called from `main()`.
/// Builds the window,
/// Populates all widgets,
/// Wires callbacks,
/// Runs the FLTK event loop.
pub fn run() {

  // create app
  let app = app::App::default();

  // enumerate the NICs on the host
  let interfaces = iface::list_interfaces();

  // initialize Ethernet packet configuration
  let config = packet::PacketConfig::default();

  let mut window = Window::new(0, 0, WIN_W, WIN_H, "ethforge");
  window.make_resizable(true);


  //-----------------------------------
  // MENU BAR
  //-----------------------------------
  let mut menu = MenuBar::new(0,0,WIN_W, WIN_H, "");
  menu.add_choice("File/Save");
  menu.add_choice("File/Load");


  //-----------------------------------
  // ETHERNET INTERFACE PANEL
  //-----------------------------------
  // group container
  let mut iface_group = Group::new(0, CONTENT_Y, IFACE_W, CONTENT_H, "");
  iface_group.set_frame(FrameType::DownBox);

  // panel heading
  Frame::new(0, CONTENT_Y,IFACE_W, 25, "Interface")
    .set_align(Align::Center);

  // create scrollable list that keeps the last clicked item highlighted
  let mut iface_browser = HoldBrowser::new(
    0,
    CONTENT_Y + IFACE_HDR_H,
    IFACE_W,
    CONTENT_H - IFACE_HDR_H,
    "",
  );

  // populate browser with interface names.
  for iface in &interfaces {
    iface_browser.add(&iface.name);
  }

  // pre-select the first interface on the list
  if !interfaces.is_empty() {
    iface_browser.select(1);
  }

  iface_group.end();


  //-----------------------------------
  // PACKET BUILDER PANEL
  //-----------------------------------
  let mut builder_scroll = Scroll::new(BUILDER_X, CONTENT_Y,BUILDER_W,CONTENT_H, "");

  builder_scroll.set_frame(FrameType::DownBox);

  // section header
  Frame::new(BUILDER_X, CONTENT_Y, BUILDER_W, 25, "Ethernet Header")
    .set_align(Align::Center);

  // Update vertical position after each field is added.
  let mut field_y = CONTENT_Y + HDR_H + 5;


  // First field: Source MAC Address
  Frame::new(BUILDER_X + MARGIN, field_y, LABEL_W, FIELD_H, "Src MAC")
    .set_align(Align::Left | Align::Inside);

  let mut src_mac_input = Input::new(
    BUILDER_X + MARGIN + LABEL_W, field_y, INPUT_W, FIELD_H, "",);

  src_mac_input.set_value(&config.src_mac);

  field_y += FIELD_H + 5;


  // Second field: Destination MAC Address
  Frame::new(BUILDER_X + MARGIN, field_y, LABEL_W, FIELD_H, "Dst MAC")
    .set_align(Align::Left | Align::Inside);

  let mut dst_mac_input = Input::new(
    BUILDER_X + MARGIN + LABEL_W, field_y, INPUT_W, FIELD_H, "",);

  dst_mac_input.set_value(&config.dst_mac);

  field_y += FIELD_H + 5;


  // Third field: EtherType
  Frame::new(BUILDER_X + MARGIN, field_y, LABEL_W, FIELD_H, "EtherType")
    .set_align(Align::Left | Align::Inside);

  let mut ethertype_choice = Choice::new(
    BUILDER_X + MARGIN + LABEL_W, field_y, INPUT_W, FIELD_H, "",);

  for et in packet::EtherType::all() {
    ethertype_choice.add_choice(et.label());
  }

  ethertype_choice.set_value(0);

  field_y += FIELD_H + 5;


  //-----------------------------------
  // SEND BUTTON
  //-----------------------------------
  field_y += 10;

  let mut send_button = fltk::button::Button::new(
    BUILDER_X + MARGIN, field_y, 80, 30, "Send");

  builder_scroll.end();


  //-----------------------------------
  // HEX PANEL PREVIEW PANEL
  //-----------------------------------
  let mut preview_group = Group::new(PREVIEW_X, CONTENT_Y, PREVIEW_W, CONTENT_H, "");
  preview_group.set_frame(FrameType::DownBox);

  Frame::new(PREVIEW_X, CONTENT_Y, PREVIEW_W, 25, "Hex Preview")
    .set_align(Align::Center);

  let mut hex_display = fltk::text::TextDisplay::new(
    PREVIEW_X +2,
    CONTENT_Y + HDR_H,
    PREVIEW_W - 4,
    CONTENT_H - HDR_H,
    ""
  );

  let hex_buf =  fltk::text::TextBuffer::default();

  hex_display.set_buffer(hex_buf.clone());
  hex_display.set_text_font(Font::Courier);
  hex_display.set_text_size(11);

  preview_group.end();

  //-----------------------------------
  // STATUS BAR
  //-----------------------------------
  let mut status = Frame::new(0, STATUS_Y, WIN_W, STATUS_H, "Ready");
  status.set_frame(FrameType::DownBox);
  status.set_align(Align::Left | Align::Inside);


  window.end();
  window.resizable(&window);
  window.show();


  //-----------------------------------
  // INTERFACE SELECTION CALLBACK
  //-----------------------------------
  // When user clicks a NIC on the list, update the status bar with the MAC
  // and IP of the selected interface.
  let interfaces_cb = interfaces.clone();
  let mut status_iface = status.clone();
  let mut src_mac_cb = src_mac_input.clone();

  iface_browser.set_callback(move |cb| {
    let idx = cb.value() - 1; // grab index of currently selected iface. rebase to 0.

    if idx >= 0 {
      let iface = &interfaces_cb[idx as usize]; // grab interface

      src_mac_cb.set_value(&iface.mac); // set to selected iface's mac

      status_iface.set_label(&format!( // update status bar
        " {} - {} - {}",
        iface.name, iface.mac, iface.ipv4
      ))
    }
  });

  //-----------------------------------
  // SEND BUTTON CALLBACK
  //-----------------------------------
  let mut status_send = status.clone();
  let mut hex_buf_cb  = hex_buf.clone();

  send_button.set_callback(move |_| {
    let cfg = packet::PacketConfig {
      src_mac   : src_mac_input.value(),
      dst_mac   : dst_mac_input.value(),
      ether_type: packet::EtherType::all()[ethertype_choice.value() as usize].clone()
    };

    match packet::build_packet(&cfg) {
      // success case
      Ok(bytes) => {
        hex_buf_cb.set_text(&hex_dump(&bytes));
        status_send.set_label("Packet built successfully.");
      }
      // error case
      Err(e) => {
        status_send.set_label(&format!("Error: {}", e));
      }
    }
  });

  app.run().unwrap();
}

fn hex_dump(bytes: &[u8]) -> String {
  let mut out = String::new();

  //for (i, chunk) in bytes.chunks(16).enumerate()
  out
}
