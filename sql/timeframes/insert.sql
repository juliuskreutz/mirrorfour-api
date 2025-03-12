INSERT INTO timeframes (user_id, day, START, duration)
    VALUES ($1, $2, $3, $4)
RETURNING
    *;

