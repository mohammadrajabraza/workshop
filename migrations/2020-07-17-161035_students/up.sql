CREATE TABLE "students"
(
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    department VARCHAR NOT NULL,
    is_graduated BOOLEAN NOT NULL,
    age INT NOT NULL
)