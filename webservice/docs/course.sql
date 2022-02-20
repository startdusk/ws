-- public.course definition
-- Drop table
-- DROP TABLE public.course;
CREATE TABLE public.course (
  id serial4 NOT NULL,
  teacher_id int4 NOT NULL,
  "name" varchar NOT NULL,
  "time" timestamp NULL DEFAULT now(),
  description varchar NULL,
  duration varchar NULL,
  format varchar NULL,
  "structure" varchar NULL,
  price int4 NULL,
  "language" varchar NULL,
  "level" varchar NULL,
  CONSTRAINT course_pkey PRIMARY KEY (id)
);