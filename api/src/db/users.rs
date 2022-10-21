use crate::models::user::{NewUserDB, User, UserJSONRequest};
use diesel::prelude::*;

pub fn create_user(conn: &mut PgConnection, item: UserJSONRequest) -> QueryResult<usize> {
	use crate::schema::users::dsl;
	let user = NewUserDB {
		username: item.username,
		password: item.password,
		created_on: chrono::offset::Local::now().naive_local(),
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
