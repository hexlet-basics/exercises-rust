-include /opt/basics/common/common.mk

ifeq ($(wildcard /opt/basics/common/common.mk),)
test:
	@ ./bin/test.sh
endif

compose-setup: compose-build

compose:
	docker compose up

compose-build:
	docker compose build

code-lint:
	cargo fmt --all --check
	cargo clippy --all-targets -- -D warnings

code-format:
	cargo fmt --all

compose-bash:
	docker compose run --rm exercises bash

compose-test:
	docker compose run --rm exercises make test

compose-code-lint:
	docker compose run --rm exercises make code-lint

compose-description-lint:
	docker compose run --rm exercises make description-lint

compose-schema-validate:
	docker compose run --rm exercises make schema-validate

ci-check:
	docker compose --file docker-compose.yml build
	docker compose --file docker-compose.yml up --abort-on-container-exit

.PHONY: test code-lint code-format
