use diesel::{
	r2d2::{ConnectionManager, Pool},
	PgConnection,
};

pub async fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

	let manager = ConnectionManager::<PgConnection>::new(&database_url);
	Pool::builder().build(manager).expect("Failed to create pool")
}
