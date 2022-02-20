-- public.teacher definition
-- Drop table
-- DROP TABLE public.teacher;
CREATE TABLE public.teacher (
  id serial4 NOT NULL,
  "name" varchar(100) NOT NULL,
  picture_url varchar(200) NOT NULL,
  profile varchar(2000) NOT NULL,
  CONSTRAINT teacher_pkey PRIMARY KEY (id)
);