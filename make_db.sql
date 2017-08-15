CREATE TABLE roles(
	id INTEGER PRIMARY KEY,
	name TEXT NOT NULL,
	permissions INTEGER
);

CREATE TABLE users(
	id INTEGER PRIMARY KEY,
	email TEXT NOT NULL,
	username TEXT NOT NULL,
	password_hash TEXT NOT NULL,
	role_id INTEGER NOT NULL,
	FOREIGN KEY(role_id) REFERENCES role(id)
);
