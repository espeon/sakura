CREATE TABLE "series" (
  "id" serial PRIMARY KEY,
  "slug" varchar NOT NULL,
  "title" varchar NOT NULL,
  "cr_id" varchar UNIQUE
);

CREATE TABLE "season" (
  "id" serial PRIMARY KEY,
  "series_id" int NOT NULL,
  "slug" varchar NOT NULL,
  "title_en" varchar,
  "title_ja" varchar,
  "title_romaji" varchar NOT NULL,
  "cr_id" varchar UNIQUE,
  "keywords" varchar,
  "anilist_id" int,
  "description" varchar,
  "synonyms" varchar,
  "episode_amt" int,
  "episode_dur" int
);

CREATE TABLE "episode" (
  "id" serial PRIMARY KEY,
  "season_id" int NOT NULL,
  "number" float NOT NULL,
  "title" varchar,
  "cr_id" varchar UNIQUE,
  "description" varchar
);

CREATE TABLE "media" (
  "id" serial PRIMARY KEY,
  "episode_id" int NOT NULL,
  "host" int NOT NULL,
  "quality" varchar NOT NULL,
  "sub_lang" varchar,
  "sub_burned" bool NOT NULL,
  "sub_url" varchar,
  "url" varchar NOT NULL,
  "time" timestamp NOT NULL
);

CREATE TABLE "genres" (
  "id" serial PRIMARY KEY,
  "name" varchar NOT NULL
);

CREATE TABLE "seasonal" (
  "id" serial PRIMARY KEY,
  "name" varchar NOT NULL
);

CREATE TABLE "season_genre" (
  "id" serial PRIMARY KEY,
  "genre" int,
  "season" int
);

CREATE TABLE "season_seasonal" (
  "id" serial PRIMARY KEY,
  "seasonal" int,
  "season" int
);

ALTER TABLE "season" ADD FOREIGN KEY ("series_id") REFERENCES "series" ("id");

ALTER TABLE "episode" ADD FOREIGN KEY ("season_id") REFERENCES "season" ("id");

ALTER TABLE "media" ADD FOREIGN KEY ("episode_id") REFERENCES "episode" ("id");

ALTER TABLE "season_genre" ADD FOREIGN KEY ("genre") REFERENCES "genres" ("id");

ALTER TABLE "season_genre" ADD FOREIGN KEY ("season") REFERENCES "season" ("id");

ALTER TABLE "season_seasonal" ADD FOREIGN KEY ("seasonal") REFERENCES "seasonal" ("id");

ALTER TABLE "season_seasonal" ADD FOREIGN KEY ("season") REFERENCES "season" ("id");
