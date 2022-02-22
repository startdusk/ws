run:
	@cargo run -p webservice --bin teacher-service | cargo run -p webapp --bin svr
