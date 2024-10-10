default:
    just --list --unsorted

db-host := env_var_or_default('DB_HOST', "localhost")
db-port := env_var_or_default('DB_PORT', "5432")
db-user := env_var_or_default('DB_USER', "postgres")
db-password := env_var_or_default('DB_PASSWORD', "admin")
db-name := env_var_or_default('DB_NAME', "brindexer")
export DATABASE_URL := "postgres://" + db-user + ":" + db-password + "@" + db-host + ":" + db-port + "/" + db-name



docker-name := env_var_or_default('DOCKER_NAME', "brindexer-postgres")
test-db-port := env_var_or_default('TEST_DB_PORT', "9433")


start-postgres:
    # we run it in --rm mode, so all data will be deleted after stopping
    docker run -p {{db-port}}:5432 --name {{docker-name}} -e POSTGRES_PASSWORD={{db-password}} -e POSTGRES_USER={{db-user}} --rm -d postgres -N 500
    sleep 3
    # wait for postgres to start, but only if db_name is not empty
    $SHELL -c '[[ -z "{{db-name}}" ]] || docker exec -it {{docker-name}} psql -U postgres -c "create database {{db-name}};"'

stop-postgres:
    docker kill {{docker-name}}

test *args:
    cargo test {{args}} -- --include-ignored

test-with-db *args:
    -just db-port="{{test-db-port}}" db-name="" docker-name="{{docker-name}}-test" start-postgres
    just db-port="{{test-db-port}}" db-name=""                                    test {{args}}

stop-test-postgres:
    just docker-name="{{docker-name}}-test" stop-postgres

run:
    BRINDEXER__DATABASE__CONNECT__URL={{DATABASE_URL}} \
    cargo run --bin server

env-run:
    dotenv -e .env -e .env.example -- just run

init-pg:
    cargo run --bin initialize_pg


