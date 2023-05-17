-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NULL
);


CREATE TABLE articles (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_on TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP
);

-- add users
INSERT INTO users (first_name, last_name) VALUES ('John', 'Doe');
INSERT INTO users (first_name, last_name) VALUES ('Abhi', 'Roy');
INSERT INTO users (first_name, last_name) VALUES ('Sukhi', 'Singh');


-- add articles for `John`
INSERT INTO articles (title, content, created_by) VALUES ('Hello World', 'This is my first article', 1);
INSERT INTO articles (title, content, created_by) VALUES ('Pollution in City', 'We need to clean the city by our own will. The city provides greenery back to us which help us thrive our mental & physical health', 3);

INSERT INTO articles (title, content, created_by) VALUES ('Say No to Smoking', 'It is a campaign that underscores the detrimental health effects of smoking, highlighting its role in causing life-threatening diseases such as lung cancer and heart disease. The campaign also underscores the addictive nature of nicotine and provides comprehensive strategies for quitting, thus promoting a healthier, smoke-free lifestyle.', 3);
