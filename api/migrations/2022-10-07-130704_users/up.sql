CREATE TABLE users (
        id serial PRIMARY KEY,
	username VARCHAR UNIQUE NOT NULL,
	password VARCHAR NOT NULL,
	created_on TIMESTAMP NOT NULL
)
