use crate::{
	db::Night,
	models::{Craziness, Drunkness, User},
};
use bson::doc;
use mongodb::{options::ClientOptions, sync::Client};

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
	nights_collection: mongodb::sync::Collection<Night>,
	#[serde(skip)]
	craziness: Craziness,
	#[serde(skip)]
	other_city: String,
	#[serde(skip)]
	vec1: Vec<Night>,
	#[serde(skip)]
	vec2: Vec<Night>,
}

impl Default for ImOk {
	fn default() -> Self {
		let mut client_options = ClientOptions::parse(
			"mongodb://pouts-os:smallest-os@localhost:27017/?retryWrites=true&w=majority",
		)
		.unwrap();
		client_options.app_name = Some("Im Ok".to_string());

		let client = Client::with_options(client_options).unwrap();

		let collection = client.database("im_ok").collection::<Night>("nights");

		Self {
			// Example stuff:
			label: "Hello World!".to_owned(),
			value: 2.7,
			nights_collection: collection,
			craziness: Craziness::default(),
			other_city: String::new(),
			vec1: Vec::new(),
			vec2: Vec::new(),
		}
	}
}

impl ImOk {
	/// Called once before the first frame.
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// This is also where you can customize the look at feel of egui using
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
		let Self { label, value, nights_collection: collection, craziness, other_city, vec1, vec2 } =
			self;

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
		let mut vec1: Vec<Night> = Vec::new();
		let mut vec2: Vec<Night> = Vec::new();

		egui::SidePanel::left("side_panel").show(ctx, |ui| {
			ui.heading("Side Panel");

			egui::CollapsingHeader::new("Lostsaka").show(ui, |ui| {
				for i in Night::get_all_nights(collection).unwrap() {
					//println!("{:?}", i.unwrap().craziness.location);
					vec1.push(i.unwrap());
				}
				ui.label(format!("{:?}", vec1))
			});
			egui::CollapsingHeader::new("Gkasma").show(ui, |ui| {
				ui.label("Body");
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

			// Submit entry to database
			if ui.add(egui::Button::new("Submit")).clicked() {
				// if `other_city` is not empty, replace
				// `craziness.location` with the other city
				// or else the location on the database will be "Other". - @charmitro
				if other_city.is_empty() {
					let night = Night { id: None, craziness: craziness.clone() };
					Night::create_night(collection, night).unwrap();
				} else {
					let night = Night {
						id: None,
						craziness: Craziness {
							user: craziness.user,
							drunkness: craziness.drunkness,
							coitus: craziness.coitus,
							drive: craziness.drive,
							talked_2x: craziness.talked_2x,
							location: other_city.to_string(),
						},
					};
					Night::create_night(collection, night).unwrap();
				};
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
