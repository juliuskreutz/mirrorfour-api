CREATE TABLE IF NOT EXISTS users (
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    username text NOT NULL,
    password text NOT NULL,
    name text NOT NULL,
    tz text NOT NULL,
    role integer NOT NULL
);

CREATE INDEX users_username_index ON users (username);

CREATE TABLE IF NOT EXISTS timeframes (
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id integer NOT NULL,
    day integer NOT NULL,
    start time NOT NULL,
    duration integer NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE INDEX timeframes_user_id_index ON timeframes (user_id);

