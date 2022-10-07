use crate::{
	datepicker::DatePicker,
	models::{
		night::{Craziness, Drunkness, Night, User},
		user,
	},
	types::AppState,
};
use bson::doc;
use chrono::Datelike;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ImOk {
	// Example stuff:
	// this how you opt-out of serialization of a member
	#[serde(skip)]
	craziness: Craziness,
	other_city: String,
	night_entries: Vec<Night>,
	selected_night: Option<Night>,
	appstate: AppState,

	username: String,
	password: String,
}

impl Default for ImOk {
	fn default() -> Self {
		let mut night_entries = Vec::<Night>::new();
		for i in Night::get_all_nights()
			.unwrap()
			.into_json::<crate::types::ResponseNights>()
			.unwrap()
			.data
			.unwrap()
			.iter()
		{
			night_entries.push(i.clone());
		}

		Self {
			craziness: Craziness::default(),
			other_city: String::new(),
			night_entries: night_entries.clone(),
			selected_night: None,
			appstate: AppState::default(),
			username: String::from("username"),
			password: String::from("password"),
		}
	}
}

impl ImOk {
	pub fn new_with_state(state: AppState) -> Self {
		println!("peos\n!");
		let mut night_entries = Vec::<Night>::new();
		for i in Night::get_all_nights()
			.unwrap()
			.into_json::<crate::types::ResponseNights>()
			.unwrap()
			.data
			.unwrap()
			.iter()
		{
			night_entries.push(i.clone());
		}

		Self {
			craziness: Craziness::default(),
			other_city: String::new(),
			night_entries: night_entries.clone(),
			selected_night: None,
			appstate: state,
			username: String::from("username"),
			password: String::from("password"),
		}
	}

	/// Called once before the first frame.
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// This is also where you can customize the look at feel of egui using
		// `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

		// Load previous app state (if any).
		// Note that you must enable the `persistence` feature for this to work.

		let mut logged_in: bool = false;

		if let Some(storage) = cc.storage {
			match eframe::get_value::<String>(storage, "TOKEN") {
				Some(resp) =>
					if resp.is_empty() {
						println!("{:?}", resp);
						logged_in = false;
					} else {
						println!("{:?}", resp);
						logged_in = true;
					},
				None => {
					logged_in = false;
				},
			}
		}

		match logged_in {
			true => Self::new_with_state(AppState::Submit),
			false => Default::default(),
		}
	}

	/// Helper function for updating the `night_entries`
	pub fn refresh(night_entries: &mut Vec<Night>) {
		night_entries.clear();
		for i in Night::get_all_nights()
			.unwrap()
			.into_json::<crate::types::ResponseNights>()
			.unwrap()
			.data
			.unwrap()
			.iter()
		{
			night_entries.push(i.clone());
		}
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
			craziness,
			other_city,
			night_entries,
			selected_night,
			appstate,
			username,
			password,
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
					if ui.button("Logout").clicked() {
						appstate.set_app_state(AppState::LoginRegister);
						eframe::set_value::<String>(
							_frame.storage_mut().unwrap(),
							"TOKEN",
							&"".to_string(),
						);
					}

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
					for i in night_entries.iter() {
						if i.craziness.user == User::Lostsaka {
							let response = ui.add(egui::SelectableLabel::new(
								false,
								format!(
									"{} {}/{}/{}",
									i.craziness.date.weekday(),
									i.craziness.date.day(),
									i.craziness.date.month(),
									i.craziness.date.year()
								),
							));
							if response.clicked() {
								*selected_night = Some(i.clone());
								*appstate = AppState::Viewing;
							}
							response.context_menu(|ui| {
								if ui.button("Edit").clicked() {
									*appstate = AppState::Editing;
									*selected_night = Some(i.clone());
									ui.close_menu();
								}
								if ui.button("Delete").clicked() {
									Night::delete_night(i.id.unwrap()).unwrap();
									ui.close_menu();
								}
							});
						}
					}
				});

				egui::CollapsingHeader::new("Gkasma").show(ui, |ui| {
					for i in night_entries.iter() {
						if i.craziness.user == User::Gkasma {
							let response = ui.add(egui::SelectableLabel::new(
								false,
								format!(
									"{} {}/{}/{}",
									i.craziness.date.weekday(),
									i.craziness.date.day(),
									i.craziness.date.month(),
									i.craziness.date.year()
								),
							));
							if response.clicked() {
								*selected_night = Some(i.clone());
								*appstate = AppState::Viewing;
							}
							response.context_menu(|ui| {
								if ui.button("Edit").clicked() {
									*appstate = AppState::Editing;
									*selected_night = Some(i.clone());
									ui.close_menu();
								}
								if ui.button("Delete").clicked() {
									Night::delete_night(i.id.unwrap()).unwrap();
									ui.close_menu();
								}
							});
						}
					}
				});

				if ui.add(egui::Button::new("Refresh")).clicked() {
					Self::refresh(night_entries);
				}
			});
		});
		egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
			if *appstate == AppState::Viewing && ui.button("Exit viewing mode").clicked() {
				*appstate = AppState::Submit;
			}
			if *appstate == AppState::Editing && ui.button("Exit edit mode").clicked() {
				*appstate = AppState::Submit;
			}
		});

		match appstate {
			AppState::LoginRegister => {
				egui::CentralPanel::default().show(ctx, |ui| {
					ui.vertical_centered_justified(|ui| {
						ui.set_max_width(250.0);
						ui.heading("Login");

						ui.add_space(20.0);
						ui.text_edit_singleline(username);

						ui.add_space(5.0);
						ui.text_edit_singleline(password);

						ui.add_space(20.0);
						if ui.add(egui::Button::new("Login")).clicked() {
							// login API call
						}
						ui.add_space(10.0);
						if ui.add(egui::Button::new("Register")).clicked() {
							// Register API call
							let user = user::User {
								id: None,
								username: username.to_string(),
								password: password.to_string(),
							};
							match user::User::register(user) {
								Ok(resp) => {
									println!("{:?}", resp);
									eframe::set_value::<String>(
										_frame.storage_mut().unwrap(),
										"TOKEN",
										&"kavlaki".to_string(),
									);
									appstate.set_app_state(AppState::Submit)
								},
								Err(err) => {
									println!("{:?}", err);
								},
							}
						}
					});
				});
			},

			AppState::Editing => {
				egui::CentralPanel::default().show(ctx, |ui| {
					// The central panel the region left after adding TopPanel's and SidePanel's
					ui.heading("Users");
					egui::ComboBox::from_id_source("my-box")
						.selected_text(format!(
							"{:?}",
							selected_night.as_ref().unwrap().craziness.user
						))
						.show_ui(ui, |ui| {
							ui.selectable_value(
								&mut selected_night.as_mut().unwrap().craziness.user,
								User::Lostsaka,
								"Lostsaka",
							);
							ui.selectable_value(
								&mut selected_night.as_mut().unwrap().craziness.user,
								User::Gkasma,
								"Gkasma",
							);
						});
					ui.separator();
					ui.heading("Drunk levels");
					egui::ComboBox::from_id_source("my-box2")
						.selected_text(format!(
							"{:?}",
							selected_night.as_mut().unwrap().craziness.drunkness
						))
						.show_ui(ui, |ui| {
							ui.selectable_value(
								&mut selected_night.as_mut().unwrap().craziness.drunkness,
								Drunkness::Cool,
								"Cool",
							);
							ui.selectable_value(
								&mut selected_night.as_mut().unwrap().craziness.drunkness,
								Drunkness::LittleHead,
								"LittleHead",
							);
							ui.selectable_value(
								&mut selected_night.as_mut().unwrap().craziness.drunkness,
								Drunkness::Bream,
								"Bream",
							);
							ui.selectable_value(
								&mut selected_night.as_mut().unwrap().craziness.drunkness,
								Drunkness::Gnat,
								"Gnat",
							);
							ui.selectable_value(
								&mut selected_night.as_mut().unwrap().craziness.drunkness,
								Drunkness::Ant,
								"Ant",
							);
							ui.selectable_value(
								&mut selected_night.as_mut().unwrap().craziness.drunkness,
								Drunkness::ImOk,
								"ImOk",
							);
						});

					ui.separator();
					ui.heading("City");
					ui.radio_value(
						&mut selected_night.as_mut().unwrap().craziness.location,
						"Athens".to_string(),
						"Athens",
					);
					ui.radio_value(
						&mut selected_night.as_mut().unwrap().craziness.location,
						"Korinthos".to_string(),
						"Korinthos",
					);
					ui.radio_value(
						&mut selected_night.as_mut().unwrap().craziness.location,
						"Other".to_string(),
						"Other",
					);

					if selected_night.as_mut().unwrap().craziness.location == *"Other".to_string() {
						ui.label("Enter your city: ");
						ui.text_edit_singleline(other_city);
					}

					ui.separator();
					ui.heading("Night Activities");
					ui.checkbox(&mut selected_night.as_mut().unwrap().craziness.coitus, "Coitus");
					ui.checkbox(&mut selected_night.as_mut().unwrap().craziness.drive, "Driven");
					ui.checkbox(
						&mut selected_night.as_mut().unwrap().craziness.talked_2x,
						"Talked_2x",
					);

					ui.separator();
					ui.text_edit_multiline(
						&mut selected_night.as_mut().unwrap().craziness.description,
					);

					ui.separator();
					ui.heading("Date");
					ui.add(DatePicker::new(
						"date_picker",
						&mut selected_night.as_mut().unwrap().craziness.date,
					));

					// Update entry to database
					ui.separator();
					if ui.add(egui::Button::new("Save")).clicked() {
						// if `other_city` is not empty, replace
						// `craziness.location` with the other city
						// or else the location on the database will be "Other". - @charmitro
						if other_city.is_empty() {
							let night = Night {
								id: Some(selected_night.as_ref().unwrap().id.unwrap()),
								craziness: selected_night.as_ref().unwrap().craziness.clone(),
							};
							Night::edit_night(night.id.unwrap(), night.craziness).unwrap();
						} else {
							let night = Night {
								id: Some(selected_night.as_ref().unwrap().id.unwrap()),
								craziness: Craziness {
									location: other_city.to_string(),
									..selected_night.as_ref().unwrap().craziness.clone()
								},
							};
							Night::edit_night(night.id.unwrap(), night.craziness).unwrap();
						};
					}
				});
			},

			AppState::Viewing => {
				egui::CentralPanel::default().show(ctx, |ui| {
					// The central panel the region left after adding TopPanel's and SidePanel's
					ui.heading(format!("{:?}", selected_night.as_ref().unwrap().craziness.user));

					ui.separator();

					ui.heading(format!(
						"Drunk level: {:?}",
						selected_night.as_ref().unwrap().craziness.drunkness
					));

					ui.separator();

					ui.heading(format!(
						"City: {}",
						selected_night.as_ref().unwrap().craziness.location
					));

					ui.separator();

					ui.heading("Night Activities");
					ui.add_enabled(
						false,
						egui::Checkbox::new(
							&mut selected_night.as_ref().unwrap().craziness.coitus.clone(),
							"Coitus",
						),
					);
					ui.add_enabled(
						false,
						egui::Checkbox::new(
							&mut selected_night.as_ref().unwrap().craziness.drive.clone(),
							"Driven",
						),
					);
					ui.add_enabled(
						false,
						egui::Checkbox::new(
							&mut selected_night.as_ref().unwrap().craziness.talked_2x.clone(),
							"Talked_2x",
						),
					);

					ui.separator();
					ui.heading("Description");
					ui.add_enabled(
						false,
						egui::TextEdit::multiline(
							&mut selected_night.as_ref().unwrap().craziness.description.clone(),
						),
					);

					ui.separator();
					ui.heading("Date");
					ui.add(DatePicker::new(
						"date_picker",
						&mut selected_night.as_ref().unwrap().craziness.date.clone(),
					));
				});
			},
			AppState::Submit => {
				egui::CentralPanel::default().show(ctx, |ui| {
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
							ui.selectable_value(
								&mut craziness.drunkness,
								Drunkness::Bream,
								"Bream",
							);
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
						ui.text_edit_singleline(other_city);
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
							Night::create_night(night).unwrap();
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
							Night::create_night(night).unwrap();
						};
					}
				});
			},
		}

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
