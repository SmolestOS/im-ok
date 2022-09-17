#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
	// Log to stdout (if you run with `RUST_LOG=debug`).
	use image::GenericImageView;
	tracing_subscriber::fmt::init();
	dotenv::dotenv().ok();

	let mut native_options = eframe::NativeOptions::default();

	let icon = image::open("assets/favicon.ico").expect("peos");
	let (icon_width, icon_height) = icon.dimensions();
	native_options.min_window_size = Some(egui::Vec2::new(840.0, 620.0));
	native_options.max_window_size = Some(egui::Vec2::new(840.0, 620.0));
	native_options.resizable = false;
	native_options.icon_data =
		Some(eframe::IconData { rgba: icon.into_bytes(), width: icon_width, height: icon_height });

	eframe::run_native("im ok", native_options, Box::new(|cc| Box::new(im_ok::ImOk::new(cc))));
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
	// Make sure panics are logged using `console.error`.
	console_error_panic_hook::set_once();

	// Redirect tracing to console.log and friends:
	tracing_wasm::set_as_global_default();

	let web_options = eframe::WebOptions::default();
	eframe::start_web(
		"the_canvas_id", // hardcode it
		web_options,
		Box::new(|cc| Box::new(im_ok::ImOk::new(cc))),
	)
	.expect("failed to start eframe");
}
