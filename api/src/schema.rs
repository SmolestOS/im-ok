// @generated automatically by Diesel CLI.

pub mod sql_types {
	#[derive(diesel::sql_types::SqlType)]
	#[diesel(postgres_type(name = "drunkness"))]
	pub struct Drunkness;
}

diesel::table! {
	use diesel::sql_types::*;
	use super::sql_types::Drunkness;

	nights (id) {
		id -> Int4,
		user_id -> Int4,
		drunkness -> Drunkness,
		coitus -> Bool,
		drive -> Bool,
		talked_2x -> Bool,
		location -> Varchar,
		description -> Varchar,
		created_at -> Date,
	}
}

diesel::table! {
	users (id) {
		id -> Int4,
		username -> Varchar,
		password -> Varchar,
		created_on -> Timestamp,
	}
}

diesel::joinable!(nights -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(nights, users,);
