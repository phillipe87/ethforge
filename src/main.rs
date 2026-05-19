fn main() -> eframe::Result<()> {

  // Title bar
  let window_title = "ethforge";

  // Window appearance and behavior
  let window_options = eframe::NativeOptions{
    viewport: egui::ViewportBuilder::default()
      .with_title("ethforge")
      .with_inner_size([960.0, 700.0]) // starting with widthxheight in pixels
      .with_min_inner_size([800.0, 600.0]), // can't resize smaller than this
    ..Default::default()
  };

  let app_creator = Box::new(|_cc: &eframe::CreationContext| -> Box<dyn eframe::App> {
    Box::new(App::new())
  });

  // Hand control to egui
  eframe::run_native(window_title, window_options, app_creator)
}

// Application state struct
struct App {
  // status message at the bottom of the window
  status: String,

  // name of currently selected network interface
  selected_iface: String,
}

impl App {
  fn new() -> Self {
    Self {
      status: String::from("Ready"),
      selected_iface: String::from(""),
    }
  }
}

impl eframe::App for App {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

    // top panel toolbar
    egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
      ui.horizontal(|ui| {
        ui.heading("ethforge");

        ui.separator();

        if ui.button("💾 Save").clicked() {
          self.status = "Save clicked.".to_string();
        }

        if ui.button("📂 Load").clicked() {
          self.status = "Load clicked.".to_string();
        }
      });
    });

    // bottom panel status bar
    egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
      ui.label(&self.status);
    });

    // left side panel
    egui::SidePanel::left("iface_panel")
      .min_width(160.0)
      .show(ctx, |ui| {
        ui.heading("Interface");
        ui.separator();
        // interface list
        let ifaces = vec!["eth0", "enp3s0", "lo"];

        for iface in ifaces.iter() {
          let is_selected = self.selected_iface == *iface;
          if ui.selectable_label(is_selected, *iface).clicked() {
            self.selected_iface = iface.to_string();
            self.status = format!("Interface: {}", iface);
          }
        }
      });

    // central panel: two columns
    egui::CentralPanel::default().show(ctx, |ui| {

      ui.columns(2, |cols| {
        // left column: packet builder
        cols[0].heading("Packet Builder");
        cols[0].separator();
        cols[0].label("Fields go here.");

        // right column: hex preview
        cols[1].heading("Hex preview");
        cols[1].separator();
        cols[1].label("Hex dump goes here.");
      });
    });
  }
}
