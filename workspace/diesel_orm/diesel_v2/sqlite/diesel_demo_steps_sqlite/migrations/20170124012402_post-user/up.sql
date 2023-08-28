CREATE TABLE posts (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 0
);

CREATE TABLE users (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  hair_color TEXT,
  created_at TIMESTAMP  NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP  NULL DEFAULT CURRENT_TIMESTAMP
);
