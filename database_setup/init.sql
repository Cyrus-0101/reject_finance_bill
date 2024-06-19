-- Create the table with the specified columns
CREATE TABLE UserDetails (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    phone_number VARCHAR(15),
    location VARCHAR(100),
    police_station VARCHAR(100)
);
