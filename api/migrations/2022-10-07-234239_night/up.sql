CREATE TYPE Drunkness AS ENUM (
       'cool', 'little_head', 'bream', 'gnat', 'ant', 'im_ok'
);

CREATE TABLE nights (
       id serial PRIMARY KEY,
       user_id int NOT NULL,
       drunkness Drunkness NOT NULL,
       coitus boolean NOT NULL,
       drive boolean NOT NULL,
       talked_2x boolean NOT NULL,
       location VARCHAR NOT NULL,
       description VARCHAR NOT NULL,
       created_at DATE NOT NULL
);

ALTER TABLE nights
ADD CONSTRAINT FK_nights_users FOREIGN KEY (user_id)
REFERENCES users (id);
