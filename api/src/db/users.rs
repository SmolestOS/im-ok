use crate::models::user::{NewUserDB, User, UserJSONRequest};
use chrono::{NaiveDate, NaiveTime};
use diesel::prelude::*;

pub fn create_user(conn: &mut PgConnection, item: UserJSONRequest) -> QueryResult<usize> {
	use crate::schema::users::dsl;
	let user = NewUserDB {
		username: item.username,
		password: item.password,
	    created_on:chrono::NaiveDateTime::new(
		NaiveDate::from_ymd(2015, 1, 1),
		NaiveTime::from_hms(23, 23, 2)),
	};

	diesel::insert_into(dsl::users).values::<NewUserDB>(user).execute(conn)
}

pub fn get_user(
	conn: &mut PgConnection,
	user: UserJSONRequest,
) -> Result<User, diesel::result::Error> {
	use crate::schema::users::dsl;

	dsl::users
		.filter(dsl::username.eq(user.username).and(dsl::password.eq(user.password)))
		.first::<User>(conn)
}
