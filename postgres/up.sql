CREATE TYPE STATUS AS ENUM('active', 'banned', 'deleted');

CREATE TABLE Institutes(
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE Classes(
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    institute_id INT NOT NULL REFERENCES Institutes(id)
);

CREATE TABLE Users(
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(64) NOT NULL,
    last_name VARCHAR(64) NOT NULL,
    tg_name VARCHAR(32) UNIQUE NOT NULL,
    phone_number VARCHAR(16) UNIQUE,
    status STATUS NOT NULL DEFAULT 'active',
    class_id INT NOT NULL REFERENCES Classes(id)
);

CREATE OR REPLACE FUNCTION chk_class()
RETURNS trigger AS $$
BEGIN
    IF NEW.name NOT SIMILAR TO '[а-яА-Я]{2}БО-\d{2}-\d{2}' THEN
        RAISE EXCEPTION '% is not valid class name', NEW.name;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER foo BEFORE INSERT OR UPDATE ON Classes FOR EACH ROW EXECUTE FUNCTION chk_class();

CREATE OR REPLACE FUNCTION chk_user()
RETURNS trigger AS $$
BEGIN
    -- i could be wrong with regex
    IF NEW.phone_number NOT SIMILAR TO '\+7\d{10}' THEN
        RAISE EXCEPTION '% is not valid phone number', NEW.phone_number;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER foo BEFORE INSERT OR UPDATE ON Users FOR EACH ROW EXECUTE FUNCTION chk_user();
