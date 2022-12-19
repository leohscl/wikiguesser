tests_run:

up:
	docker compose -f docker/docker_web.yaml up
down:
	docker compose -f docker/docker_web.yaml down
populate:
	docker compose -f docker/docker_populate.yaml up \
		--abort-on-container-exit \
		--exit-code-from populate_db
build:
	docker compose -f docker/docker_populate.yaml build
	docker compose -f docker/docker_web.yaml build
