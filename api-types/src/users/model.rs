use crate::schema::users;
use chrono::{NaiveDate, NaiveTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct User {
	pub id: i32,
	pub username: String,
	pub password: String,
	pub created_on: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct UserJSONRequest {
	pub username: String,
	pub password: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUserDB {
	pub username: String,
	pub password: String,
	pub created_on: chrono::NaiveDateTime,
}

impl User {
	pub fn create_user(conn: &mut PgConnection, item: UserJSONRequest) -> QueryResult<usize> {
		use crate::schema::users::dsl::*;
		let user = NewUserDB {
			created_on: chrono::NaiveDateTime::new(
				NaiveDate::from_ymd(2015, 1, 1),
				NaiveTime::from_hms(23, 23, 2),
			),

			username: item.username,
			password: item.password,
		};

		diesel::insert_into(users).values(&user).execute(conn)
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
}
