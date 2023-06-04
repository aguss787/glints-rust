CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "hubbers" (
   id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
   code VARCHAR(32) UNIQUE NOT NULL,
   name VARCHAR(128) NOT NULL
);

INSERT INTO hubbers(code, name) VALUES ('GLID-EX-1', 'CAT');
INSERT INTO hubbers(code, name) VALUES ('GLID-EX-2', 'DOG');
