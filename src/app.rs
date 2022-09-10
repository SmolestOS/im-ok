use bson::doc;
use mongodb::{options::ClientOptions, sync::Client};

use crate::{
	db::Night,
	models::{Craziness, Drunkness, User},
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ImOk {
	// Example stuff:
	label: String,

	// this how you opt-out of serialization of a member
	#[serde(skip)]
	value: f32,

	#[serde(skip)]
	collection: mongodb::sync::Collection<Night>,
	#[serde(skip)]
	craziness: Craziness,
	#[serde(skip)]
	other_city: String,
}

impl Default for ImOk {
	fn default() -> Self {
		let mut client_options = ClientOptions::parse(
            "mongodb+srv://pouts-os:smallest-os@im-ok.nzhnepa.mongodb.net/?retryWrites=true&w=majority",
        )
        .unwrap();
		client_options.app_name = Some("Im Ok".to_string());

		let client = Client::with_options(client_options).unwrap();

		let collection = client.database("im_ok").collection::<Night>("nights");

		Self {
			// Example stuff:
			label: "Hello World!".to_owned(),
			value: 2.7,
			collection,
			craziness: Craziness::default(),
			other_city: String::new(),
		}
	}
}

impl ImOk {
	/// Called once before the first frame.
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// This is also where you can customized the look at feel of egui using
		// `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

		// Load previous app state (if any).
		// Note that you must enable the `persistence` feature for this to work.
		if let Some(storage) = cc.storage {
			return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
		}

		Default::default()
	}
}

impl eframe::App for ImOk {
	/// Called by the frame work to save state before shutdown.
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self);
	}

	/// Called each time the UI needs repainting, which may be many times per second.
	/// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let Self { label, value, collection, craziness, other_city } = self;

		// Examples of how to create different panels and windows.
		// Pick whichever suits you.
		// Tip: a good default choice is to just keep the `CentralPanel`.
		// For inspiration and more examples, go to https://emilk.github.io/egui

		#[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			// The top panel is often a good place for a menu bar:
			egui::menu::bar(ui, |ui| {
				ui.menu_button("File", |ui| {
					if ui.button("Quit").clicked() {
						_frame.close();
					}
				});
			});
		});

		egui::SidePanel::left("side_panel").show(ctx, |ui| {
			ui.heading("Side Panel");

			ui.horizontal(|ui| {
				ui.label("Write something: ");
				ui.text_edit_singleline(label);
			});

			ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
			if ui.button("Increment").clicked() {
				let captain_marvel = Night {
					id: None,
					location: "Captain Marvel".to_owned(),
					date: bson::DateTime::now(),
				};

				let insert_res = Night::create_night(collection, captain_marvel).unwrap();
				println!("{}", insert_res.inserted_id);

				*value += 1.0;
			}

			ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
				ui.horizontal(|ui| {
					ui.spacing_mut().item_spacing.x = 0.0;
					ui.label("powered by ");
					ui.hyperlink_to("egui", "https://github.com/emilk/egui");
					ui.label(" and ");
					ui.hyperlink_to(
						"eframe",
						"https://github.com/emilk/egui/tree/master/crates/eframe",
					);
					ui.label(".");
				});
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			// The central panel the region left after adding TopPanel's and SidePanel's

			egui::ComboBox::from_label("Select user")
				.selected_text(format!("{:?}", craziness.user))
				.show_ui(ui, |ui| {
					ui.selectable_value(&mut craziness.user, User::Lostsaka, "Lostsaka");
					ui.selectable_value(&mut craziness.user, User::Gkasma, "Gkasma");
				});
			egui::ComboBox::from_label("Select level of drunkness")
				.selected_text(format!("{:?}", craziness.drunkness))
				.show_ui(ui, |ui| {
					ui.selectable_value(&mut craziness.drunkness, Drunkness::Cool, "Cool");
					ui.selectable_value(
						&mut craziness.drunkness,
						Drunkness::LittleHead,
						"LittleHead",
					);
					ui.selectable_value(&mut craziness.drunkness, Drunkness::Bream, "Bream");
					ui.selectable_value(&mut craziness.drunkness, Drunkness::Gnat, "Gnat");
					ui.selectable_value(&mut craziness.drunkness, Drunkness::Ant, "Ant");
					ui.selectable_value(&mut craziness.drunkness, Drunkness::ImOk, "ImOk");
				});

			ui.checkbox(&mut craziness.coitus, "Coitus");
			ui.checkbox(&mut craziness.drive, "Driven");
			ui.checkbox(&mut craziness.talked_2x, "Talked_2x");
			ui.radio_value(&mut craziness.location, "Athens".to_string(), "Athens");
			ui.radio_value(&mut craziness.location, "Korinthos".to_string(), "Korinthos");

			ui.radio_value(&mut craziness.location, "Other".to_string(), "Other");
			if craziness.location == *"Other".to_string() {
				ui.label("Enter your city: ");
				ui.text_edit_singleline(other_city);
			}

			egui::warn_if_debug_build(ui);
		});

		if false {
			egui::Window::new("Window").show(ctx, |ui| {
				ui.label("Windows can be moved by dragging them.");
				ui.label("They are automatically sized based on contents.");
				ui.label("You can turn on resizing and scrolling if you like.");
				ui.label("You would normally chose either panels OR windows.");
			});
		}
	}
}
