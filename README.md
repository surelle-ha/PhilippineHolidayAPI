# Philippine Holiday API

Originally built using Python and Flask, this project has been completely rewritten in Rust using the Axum framework for
improved performance and reliability.

This API provides information about Philippine holidays, including regular holidays, special non-working days, and more.
It is designed to be fast, efficient, and easy to use.

## API Endpoints

- `GET /health`: Health check endpoint to verify the API is running.
- `POST /holiday/{year}`: Scrape and store holidays for the specified year. This may take some time as it fetches data
  from external sources.
- `GET /holiday/{year}`: Retrieve stored holidays for the specified year.
- `DELETE /holiday/{year}`: Delete stored holidays for the specified year.
- `PUT /holiday/{year}`: Update holidays for the specified year. This will re-scrape the data and update the stored
  information.

## To Do List

- [ ] Microservice Event Emitter integration (Port: 1212)
- [ ] Fix Dockerfile to use multi-stage builds for smaller image sizes
- [ ] Implement rate limiting
- [ ] Add more comprehensive error handling and logging
- [ ] Write unit and integration tests for all endpoints
- [ ] Set up CI/CD pipeline for automated testing and deployment
- [ ] Add response schema pattern on documentations
- [ ] Improve API documentation with examples
- [ ] Implement Caching for frequently accessed data

## Development setup

```
cargo make dev-setup              # Complete dev environment setup
cargo make dev                    # Start dev environment
cargo make watch                  # Watch and auto-rebuild
```

## Building

```
cargo make build                  # Debug build
cargo make bin-build             # Release build
cargo make release-build         # Release + Docker image
```

## Testing

```
cargo make test                   # Run tests
cargo make test-watch            # Watch tests
cargo make ci                    # Run all CI checks
```

## Docker

```
cargo make docker-build          # Build Docker image
cargo make docker-run            # Run container
cargo make docker-compose-up     # Start with compose
cargo make docker-compose-down   # Stop compose
cargo make docker-full           # Full Docker workflow
cargo make deploy-local          # Clean deploy locally
```

## API Testing

```
cargo make api-test-health
```