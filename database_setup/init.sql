-- Create the table with the specified columns
CREATE TABLE UserDetails (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50),
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    location VARCHAR(100) NOT NULL,
    police_station VARCHAR(100) NOT NULL
);
