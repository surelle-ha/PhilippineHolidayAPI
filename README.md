# 📅 Philippine Holiday API

[![🦀 Rust Build and Test](https://github.com/surelle-ha/PhilippineHolidayAPI/actions/workflows/build-test.yml/badge.svg)](https://github.com/surelle-ha/PhilippineHolidayAPI/actions/workflows/build-test.yml)
[![📦 Dependency Check](https://github.com/surelle-ha/PhilippineHolidayAPI/actions/workflows/deps-check.yml/badge.svg)](https://github.com/surelle-ha/PhilippineHolidayAPI/actions/workflows/deps-check.yml)

Originally built using Python and Flask, this project has been completely rewritten in Rust using the Axum framework for
improved performance and reliability.

This API provides information about Philippine holidays, including regular holidays, special non-working days, and more.
The data is scraped from official government sources to ensure accuracy and up-to-date information.

> 💠 This project is **not funded by any government and is entirely maintained by volunteers**. It is open-source and free to use. If you find it useful, please consider starring the repository or contributing to its development!

> 🧪 **Note**: This project is currently in beta. While it is functional, there may be bugs or incomplete features. Use at your own risk. Please report any issues or feature requests on the GitHub repository. Contributions are welcome! View Change Log [here](./CHANGELOG.md).

## ⭕ API Endpoints

- `GET /health`: Health check endpoint to verify the API is running.
- `POST /holiday/{year}`: Scrape and store holidays for the specified year. This may take some time as it fetches data
  from external sources.
- `GET /holiday/{year}`: Retrieve stored holidays for the specified year.
- `DELETE /holiday/{year}`: Delete stored holidays for the specified year.
- `PUT /holiday/{year}`: Update holidays for the specified year. This will re-scrape the data and update the stored
  information.

## ⭕ Custom Integrations 

- **NestJS**: A NestJS module is available for easy integration into NestJS applications. Check out the [ph-government-api](https://github.com/surelle-ha/ph-government-api).
- **Express.js**: An Express.js middleware is available for easy integration into Express.js applications. Check out the [ph-government-api](https://github.com/surelle-ha/ph-government-api)
- **Event Emitter**: Integration with Microservice Event Emitter for event-driven architectures. Check out the [microservice-event-emitter](https://github.com/surelle-ha/ph-government-api)

## ⭕ To Do List

- [x] Fix Dockerfile to use multi-stage builds for smaller image sizes
- [x] Add [`Render`](https://dashboard.render.com/blueprint/new) integration
- [ ] Use SSE for long-running tasks like generating holidays
- [ ] Disable direct push to `main` branch
- [ ] Microservice Event Emitter integration (Port: 1212)
- [ ] Implement rate limiting
- [ ] Add more comprehensive error handling and logging
- [ ] Write unit and integration tests for all endpoints
- [ ] Set up CI/CD pipeline for automated testing and deployment
- [ ] Add response schema pattern on documentations
- [ ] Improve API documentation with examples
- [ ] Implement Caching for frequently accessed data

## ⭕ Development Instructions

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

## ⭕ Technical Information

Here are some technical details about the project:

### Allowed Advisories
- **Critical:** "RUSTSEC-2020-0071"
- **Warning:** "RUSTSEC-2024-0436", "RUSTSEC-2025-0057"

### Known Issues

- Long response times when scraping holidays for the first time due to external data fetching.
- Returning empty results on Render Web Services. - Render Docker untested yet.

### Public Deployment

For general use, this service is publicly available, and the API is deployed at the following URL:
> ❗ **Note**: This is a free service and may have usage limitations. For production use, consider deploying your own instance, or you may sponsor the project.
- [https://philippineholidayapi.onrender.com/](https://philippineholidayapi.onrender.com/)