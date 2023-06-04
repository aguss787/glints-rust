# Sample RUST Graphql API

## Setup Environment
```bash
docker compose up -d
cp .env.example .env
```

## Run migration
```bash
cargo install diesel_cli
diesel migration run
```

## Run the project
```bash
cargo run
```

## Build for prod
```bash
cargo build --release
```

### Sample GraphQL Query

```graphql
# playground at localhost:8080 if the feature is enabled
{
  hubbers {
    code
    id
    name
    computedString
  }
}
```