# ⭕ Philippine Holiday API

[![🦀 Rust Build and Test](https://github.com/surelle-ha/PhilippineHolidayAPI/actions/workflows/build-test.yml/badge.svg)](https://github.com/surelle-ha/PhilippineHolidayAPI/actions/workflows/build-test.yml)
[![📦 Dependency Check](https://github.com/surelle-ha/PhilippineHolidayAPI/actions/workflows/deps-check.yml/badge.svg)](https://github.com/surelle-ha/PhilippineHolidayAPI/actions/workflows/deps-check.yml)

Originally built using Python and Flask, this project has been completely rewritten in Rust using the Axum framework for
improved performance and reliability.

This API provides information about Philippine holidays, including regular holidays, special non-working days, and more.
It is designed to be fast, efficient, and easy to use.

> 🧪 **Note**: This project is currently in beta. While it is functional, there may be bugs or incomplete features. Use at your own risk. Please report any issues or feature requests on the GitHub repository. Contributions are welcome!

> View Change Log [here](./CHANGELOG.md).

## API Endpoints

- `GET /health`: Health check endpoint to verify the API is running.
- `POST /holiday/{year}`: Scrape and store holidays for the specified year. This may take some time as it fetches data
  from external sources.
- `GET /holiday/{year}`: Retrieve stored holidays for the specified year.
- `DELETE /holiday/{year}`: Delete stored holidays for the specified year.
- `PUT /holiday/{year}`: Update holidays for the specified year. This will re-scrape the data and update the stored
  information.

## To Do List

- [ ] Fix Dockerfile to use multi-stage builds for smaller image sizes
- [ ] Add [`Render`](https://dashboard.render.com/blueprint/new) integration
- [ ] Disable direct push to `main` branch
- [ ] Microservice Event Emitter integration (Port: 1212)
- [ ] Implement rate limiting
- [ ] Add more comprehensive error handling and logging
- [ ] Write unit and integration tests for all endpoints
- [ ] Set up CI/CD pipeline for automated testing and deployment
- [ ] Add response schema pattern on documentations
- [ ] Improve API documentation with examples
- [ ] Implement Caching for frequently accessed data

## Development Instructions

This project uses `cargo-make` to streamline development tasks. Below are the common commands you can use.

### Development setup

```
cargo make dev-setup              # Complete dev environment setup
cargo make dev                    # Start dev environment
cargo make watch                  # Watch and auto-rebuild
```

### Building

```
cargo make build                  # Debug build
cargo make bin-build             # Release build
cargo make release-build         # Release + Docker image
```

### Testing

```
cargo make test                   # Run tests
cargo make test-watch            # Watch tests
cargo make ci                    # Run all CI checks
```

### Docker

```
cargo make docker-build          # Build Docker image
cargo make docker-run            # Run container
cargo make docker-compose-up     # Start with compose
cargo make docker-compose-down   # Stop compose
cargo make docker-full           # Full Docker workflow
cargo make deploy-local          # Clean deploy locally
```

### API Testing

```
cargo make api-test-health
```

## Technical Information

Here are some technical details about the project:

### Allowed Advisories
- **Critical:** "RUSTSEC-2020-0071"
- **Warning:** "RUSTSEC-2024-0436", "RUSTSEC-2025-0057"