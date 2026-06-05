# template-server-rust
A template for a RESTful API server written in Rust that can be forked for other projects

## Technology 
The server is written in Rust using Axum.

## Architecture
The architecture is generally built around vertical feature slices. Each feature folder owns transport, domain logic, dtos, entities, errors, etc. Concrete repos, however, are stored in the `repos` module as they may be accessed by many different features. Currently, features (e.g. `auth`) depend on concrete repo implementations (e.g. `user_repo`). This is fine for a small project, but when scaling out, it may be worth adding an interface/trait layer that lives in the feature module and then inject the concrete implementation in DI (`app_state`). It's also likely that `repos` will evolve into a more general `infrastructure` module which would contain other concrete implementation like message queues. 

The structure of a feature slice is roughly: 
- `Routers` map routes to handlers 
- `Handlers` accept and return dtos. They map dtos to domain entities and pass the entity to a service to orchestrate business logic. Handlers also validate requests as much as they can be without requiring additional data (e.g. can validate that a string is of a certain length but not whether a username is already in use)
- `DTOs` define the shape of data coming into and out of the API
- `Services` orchestrate logic on domain entities. For example, when registering a user, the `auth_service` asks the repo if the username is in use, calls a function to hash the password, calls the repo to persist the user, and calls another function to generate a token. Services should not know about how things are done, they're just in charge of making sure the right things happen in the right order. Importantly, mapping `DTOs` to `Models` and back is done outside of the service as services should not know about DTOs.
- `Models` are the data structures that `Services` deal in. They should have as few concrete dependencies as possible.

As mentioned, repos currently sit outside of feature slices and are concrete (e.g. they specifically use psql). In the future we could abstract this so domain services don't know about concrete repos. 

## Deployment 
The server can be run locally via Docker. `Dockerfile` contains instructions to build the image and run the server container, and `docker-compose.yml` can be used to run the container.

For development, it's recommended to run the rust binaries locally using `cargo run --bin server` (e.g. for the server) or `cargo watch...` as Docker is very slow at rebuilding containers. However, the option to run the rust binaries in Docker is always there. Note: when running rust binaries locally, ensure db connection strings etc. refer to `localhost` rather than the service name (e.g. `db`) as the service name is only visible to services on the internal docker network. Docker exposes all its services on localhost using the given port mappings. 

## Environment variables 
Env variables are stored locally in `.env`. `.env.example` is a copy of `.env` that is stored in source control and gives example values of env variables.

## Logging 
We use `trace` and `tower_http` to log