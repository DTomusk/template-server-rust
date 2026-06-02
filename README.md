# template-server-rust
A template for a RESTful API server written in Rust that can be forked for other projects

## Technology 
The server is written in Rust using Axum.

## Architecture
-

## Deployment 
The server can be run locally via Docker. `Dockerfile` contains instructions to build the image and run the server container, and `docker-compose.yml` can be used to run the container.

## Environment variables 
Env variables are stored locally in `.env`. `.env.example` is a copy of `.env` that is stored in source control and gives example values of env variables.

## Logging 
We use `trace` and `tower_http` to log