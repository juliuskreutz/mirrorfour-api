INSERT INTO users (username, PASSWORD, name, tz, ROLE)
    VALUES ($1, $2, $3, $4, $5)
RETURNING
    *;

