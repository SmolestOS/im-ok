use std::collections::BTreeMap;

use crate::{
	datepicker::DatePicker,
	db::Night,
	models::{Craziness, Drunkness, User},
};
use bson::{doc, oid::ObjectId};
use chrono::Datelike;
use mongodb::{error::Error, options::ClientOptions, sync::Client};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ImOk {
	// Example stuff:
	// this how you opt-out of serialization of a member
	#[serde(skip)]
	nights_collection: mongodb::sync::Collection<Night>,
	#[serde(skip)]
	craziness: Craziness,
	#[serde(skip)]
	other_city: String,
	#[serde(skip)]
	night_entries: BTreeMap<ObjectId, Craziness>,

	#[serde(skip)]
	selected_night: Option<(ObjectId, Craziness)>,
	#[serde(skip)]
	editing: bool,
}

impl Default for ImOk {
	fn default() -> Self {
		let mut client_options = ClientOptions::parse(
			std::env::var("MONGO_URI").expect("MONGO_URI environment variable not set."),
		)
		.unwrap();
		client_options.app_name = Some("Im Ok".to_string());

		let client = Client::with_options(client_options).unwrap();

		#[cfg(debug_assertions)]
		let mut collection = client.database("im_ok").collection::<Night>("nights");
		#[cfg(not(debug_assertions))]
		let mut collection = client.database("im_ok_prod").collection::<Night>("nights");

		let mut night_entries = BTreeMap::<ObjectId, Craziness>::new();
		for i in Night::get_all_nights(&mut collection).unwrap() {
			night_entries.insert(i.as_ref().unwrap().id.unwrap(), i.unwrap().craziness);
		}

		Self {
			// Example stuff:
			nights_collection: collection,
			craziness: Craziness::default(),
			other_city: String::new(),
			night_entries,
			selected_night: None,
			editing: false,
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

	/// Helper function for updating the `night_entries`
	pub fn refresh(
		night_entries: &mut BTreeMap<ObjectId, Craziness>,
		mut collection: mongodb::sync::Collection<Night>,
	) {
		for i in Night::get_all_nights(&mut collection).unwrap() {
			night_entries.insert(i.as_ref().unwrap().id.unwrap(), i.unwrap().craziness);
		}
	}

	pub fn delete_entry(
		night_entries: &mut BTreeMap<ObjectId, Craziness>,
		mut collection: mongodb::sync::Collection<Night>,
		id: ObjectId,
	) {
		Night::delete_night(&mut collection, id)
			.map(|_| {
				night_entries.remove(&id).unwrap();
				Self::refresh(night_entries, collection.clone());
				Ok::<(), Error>(())
			})
			.unwrap()
			.unwrap();
	}

	fn draw_central_panel(
		ctx: &egui::Context,
		collection: mongodb::sync::Collection<Night>,
		craziness: &mut Craziness,
		other_city: &String,
		editing: bool,
	) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.set_enabled(editing);

			// The central panel the region left after adding TopPanel's and SidePanel's
			ui.heading("Users");
			egui::ComboBox::from_id_source("my-box")
				.selected_text(format!("{:?}", craziness.user))
				.show_ui(ui, |ui| {
					ui.selectable_value(&mut craziness.user, User::Lostsaka, "Lostsaka");
					ui.selectable_value(&mut craziness.user, User::Gkasma, "Gkasma");
				});
			ui.separator();
			ui.heading("Drunk levels");
			egui::ComboBox::from_id_source("my-box2")
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

			ui.separator();
			ui.heading("City");
			ui.radio_value(&mut craziness.location, "Athens".to_string(), "Athens");
			ui.radio_value(&mut craziness.location, "Korinthos".to_string(), "Korinthos");
			ui.radio_value(&mut craziness.location, "Other".to_string(), "Other");

			if craziness.location == *"Other".to_string() {
				ui.label("Enter your city: ");
				ui.text_edit_singleline(&mut other_city.to_string());
			}

			ui.separator();
			ui.heading("Night Activities");
			ui.checkbox(&mut craziness.coitus, "Coitus");
			ui.checkbox(&mut craziness.drive, "Driven");
			ui.checkbox(&mut craziness.talked_2x, "Talked_2x");

			ui.separator();
			ui.text_edit_multiline(&mut craziness.description);

			ui.separator();
			ui.heading("Date");
			ui.add(DatePicker::new("date_picker", &mut craziness.date));

			// Submit entry to database
			ui.separator();
			if ui.add(egui::Button::new("Submit")).clicked() {
				// if `other_city` is not empty, replace
				// `craziness.location` with the other city
				// or else the location on the database will be "Other". - @charmitro
				if other_city.is_empty() {
					let night = Night { id: None, craziness: craziness.clone() };
					Night::create_night(&mut collection.clone(), night).unwrap();
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
							description: craziness.description.clone(),
							date: craziness.date,
						},
					};
					Night::create_night(&mut collection.clone(), night).unwrap();
				};
			}
		});
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
		let Self {
			nights_collection: collection,
			craziness,
			other_city,
			night_entries,
			selected_night,
			editing,
		} = self;

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
				ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
					egui::global_dark_light_mode_switch(ui);
				});
			});
		});

		egui::SidePanel::left("side_panel").min_width(120.0).show(ctx, |ui| {
			egui::ScrollArea::both().show(ui, |ui| {
				egui::CollapsingHeader::new("Lostsaka").show(ui, |ui| {
					for i in night_entries.clone().iter() {
						if i.1.user == User::Lostsaka {
							let response = ui.add(egui::SelectableLabel::new(
								false,
								format!(
									"{} {}/{}/{}",
									i.1.date.weekday(),
									i.1.date.day(),
									i.1.date.month(),
									i.1.date.year()
								),
							));
							if response.clicked() {
								*selected_night = Some((*i.0, i.1.clone()));
							}
							response.context_menu(|ui| {
								if ui.button("Edit").clicked() {
									*editing = true;
								}
								if ui.button("Delete").clicked() {
									Self::delete_entry(night_entries, collection.clone(), *i.0);
									ui.close_menu();
								}
							});
						}
					}
				});
				egui::CollapsingHeader::new("Gkasma").show(ui, |ui| {
					for i in night_entries.clone().iter() {
						if i.1.user == User::Gkasma {
							let response = ui.add(egui::SelectableLabel::new(
								false,
								format!(
									"{} {}/{}/{}",
									i.1.date.weekday(),
									i.1.date.day(),
									i.1.date.month(),
									i.1.date.year()
								),
							));
							if response.clicked() {
								*selected_night = Some((*i.0, i.1.clone()));
							}
							response.context_menu(|ui| {
								if ui.button("Edit").clicked() {
									*editing = true;
								}
								if ui.button("Delete").clicked() {
									Self::delete_entry(night_entries, collection.clone(), *i.0);
									ui.close_menu();
								}
							});
						}
					}
				});

				if ui.add(egui::Button::new("Refresh")).clicked() {
					Self::refresh(night_entries, collection.clone());
				}
			});
		});

		// VIEWING MODE
		// if selected_night.is_some() {
		// 	egui::CentralPanel::default().show(ctx, |ui| {
		// 		// The central panel the region left after adding TopPanel's and SidePanel's
		// 		ui.heading(format!("{:?}", selected_night.as_ref().unwrap().1.user));

		// 		ui.separator();

		// 		ui.heading(format!(
		// 			"Drunk level: {:?}",
		// 			selected_night.as_ref().unwrap().1.drunkness
		// 		));

		// 		ui.separator();

		// 		ui.heading(format!("City: {}", selected_night.as_ref().unwrap().1.location));

		// 		ui.separator();

		// 		ui.heading("Night Activities");
		// 		ui.add_enabled(
		// 			false,
		// 			Checkbox::new(&mut selected_night.as_ref().unwrap().1.coitus.clone(), "Coitus"),
		// 		);
		// 		ui.add_enabled(
		// 			false,
		// 			Checkbox::new(&mut selected_night.as_ref().unwrap().1.drive.clone(), "Driven"),
		// 		);
		// 		ui.add_enabled(
		// 			false,
		// 			Checkbox::new(
		// 				&mut selected_night.as_ref().unwrap().1.talked_2x.clone(),
		// 				"Talked_2x",
		// 			),
		// 		);

		// 		ui.separator();
		// 		ui.heading("Description");
		// 		ui.add_enabled(
		// 			false,
		// 			TextEdit::multiline(
		// 				&mut selected_night.as_ref().unwrap().1.description.clone(),
		// 			),
		// 		);

		// 		ui.separator();
		// 		ui.heading("Date");
		// 		ui.add(DatePicker::new(
		// 			"date_picker",
		// 			&mut selected_night.as_ref().unwrap().1.date.clone(),
		// 		));

		// 		// Submit entry to database
		// 		if ui.add(egui::Button::new("Exit viewing mode")).clicked() {
		// 			*selected_night = None;
		// 		}
		// 	});
		// }

		Self::draw_central_panel(ctx, collection.clone(), craziness, other_city, *editing);

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
