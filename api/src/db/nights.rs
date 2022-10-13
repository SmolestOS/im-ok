use crate::models::night::{NewNightDB, Night, NightJSONRequest, NightWithUser};
use chrono::Local;
use diesel::prelude::*;

pub fn create_night(conn: &mut PgConnection, item: NightJSONRequest) -> QueryResult<usize> {
	use crate::schema::nights::dsl;
	let night = NewNightDB {
		user_id: item.user_id,
		drunkness: item.drunkness,
		coitus: item.coitus,
		drive: item.drive,
		talked_2x: item.talked_2x,
		location: item.location,
		description: item.description,
		created_at: Local::now().date_naive(),
	};

	diesel::insert_into(dsl::nights).values::<NewNightDB>(night).execute(conn)
}

pub fn get_all_nights(conn: &mut PgConnection) -> Result<Vec<Night>, diesel::result::Error> {
	use crate::schema::nights::dsl;

	dsl::nights.load::<Night>(conn)
}

pub fn get_all_nights_with_user(
	conn: &mut PgConnection,
) -> Result<Vec<NightWithUser>, diesel::result::Error> {
	use crate::schema::{nights, users};
	nights::dsl::nights
		.inner_join(users::table.on(nights::dsl::user_id.eq(users::dsl::id)))
		.select((
			nights::dsl::id,
			nights::dsl::user_id,
			users::dsl::username,
			nights::dsl::drunkness,
			nights::dsl::coitus,
			nights::dsl::drive,
			nights::dsl::talked_2x,
			nights::dsl::location,
			nights::dsl::description,
			nights::dsl::created_at,
		))
		.load::<NightWithUser>(conn)
}

pub fn get_night(conn: &mut PgConnection, item_id: i32) -> Result<Night, diesel::result::Error> {
	use crate::schema::nights::dsl;
	dsl::nights.filter(dsl::id.eq(item_id)).first::<Night>(conn)
}

pub fn delete_night(conn: &mut PgConnection, item_id: i32) -> Result<usize, diesel::result::Error> {
	use crate::schema::nights::dsl;
	diesel::delete(dsl::nights.filter(dsl::id.eq(item_id))).execute(conn)
}

pub fn edit_night(
	conn: &mut PgConnection,
	item_id: i32,
	updated_night: NightJSONRequest,
) -> Result<usize, diesel::result::Error> {
	use crate::schema::nights::dsl;
	diesel::update(dsl::nights)
		.filter(dsl::id.eq(item_id))
		.set::<NightJSONRequest>(updated_night)
		.execute(conn)
}
