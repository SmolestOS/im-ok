use crate::models::user::{NewUserDB, User, UserJSONRequest};
use diesel::prelude::*;

pub fn create_user(conn: &mut PgConnection, item: UserJSONRequest) -> QueryResult<i32> {
	use crate::schema::users::dsl;
	let user = NewUserDB {
		username: item.username,
		password: item.password,
		created_on: chrono::offset::Local::now().naive_local(),
	};

	diesel::insert_into(dsl::users)
		.values::<NewUserDB>(user)
		.returning(dsl::id)
		.get_result(conn)
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
pub fn delete_user(conn: &mut PgConnection, item_id: i32) -> Result<usize, diesel::result::Error> {
	use crate::schema::users::dsl;
	diesel::delete(dsl::users.filter(dsl::id.eq(item_id))).execute(conn)
}
