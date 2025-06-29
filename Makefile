run:
	@cargo run

watch:
	@cargo watch -x run

# Migration
migrate-up:
	@cargo run -p migration -- up

migrate-down:
	@cargo run -p migration -- down

migrate-status:
	@cargo run -p migration -- status

migrate-fresh:
	@cargo run -p migration -- fresh

migrate-refresh:
	@cargo run -p migration -- refresh

migrate-reset:
	@cargo run -p migration -- reset

migrate-generate:
	@sea-orm-cli migrate generate $(name) -d ./migration 

# Migration by step
migrate-up-one:
	@cargo run -p migration -- up -n 1

migrate-down-one:
	@cargo run -p migration -- down -n 1

migrate-up-n:
	@cargo run -p migration -- up -n $(n)

migrate-down-n:
	@cargo run -p migration -- down -n $(n)

entity-generate:
	@sea-orm-cli generate entity \
		-o entity/src/

# Seeder
seeder-run:
	@cargo run -p seeder -- up

seeder-down:
	@cargo run -p seeder -- down

seeder-status:
	@cargo run -p seeder -- status

seeder-fresh:
	@cargo run -p seeder -- fresh

seeder-refresh:
	@cargo run -p seeder -- refresh

seeder-reset:
	@cargo run -p seeder -- reset

seeder-generate:
	@sea-orm-cli migrate generate "$(name)_seeder" -d ./seeder

# Seeder by step
seeder-up-one:
	@cargo run -p seeder -- up -n 1

seeder-down-one:
	@cargo run -p seeder -- down -n 1

seeder-up-n:
	@cargo run -p seeder -- up -n $(n)

seeder-down-n:
	@cargo run -p seeder -- down -n $(n)