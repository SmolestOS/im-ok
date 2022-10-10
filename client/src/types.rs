#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum AppState {
	LoginRegister,
	Editing,
	Viewing,
	Submit,
}

impl Default for AppState {
	fn default() -> Self {
		Self::LoginRegister
	}
}

impl AppState {
	pub fn set_app_state(&mut self, state: AppState) {
		*self = state;
	}
}
