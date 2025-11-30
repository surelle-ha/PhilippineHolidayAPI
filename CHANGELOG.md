# Changelog

All notable changes to this project will be documented in this file.  
This project adheres to [Semantic Versioning](https://semver.org/).

---

## [1.0.2-beta] - 2025-11-18

**Highlights / Improvements**:

- **Massive performance improvements** in memory operations:
    - `put_value` latency reduced from ~140 µs → ~4.8 µs
    - `get_value` latency reduced from ~10 µs → ~4.1 µs
- **Throughput** increased dramatically:
    - Put Ops/sec: ~122,000
    - Get Ops/sec: ~127,000
- Background autosave logic optimized:
    - Removed synchronous disk writes from individual `put/delete` calls.
    - Background saver handles persistence asynchronously without blocking operations.
- Outliers reduced or statistically insignificant (`p > 0.05`).
- Maintains encrypted persistence for collections with AES-GCM + base64 encoding.

---

### Benchmark Table

| **Version** | **Date** | **Put Ops/sec** | **Get Ops/sec** | **Put Latency (µs)** | **Get Latency (µs)** | **Notes** |
|-------------|----------|----------------|----------------|--------------------|--------------------|-----------|
| 1.0.1-beta  | 2025-11-17 | ~7,000          | ~100,000        | 136–145               | 9–10                  | Initial benchmark with 100 measurements. Found some outliers (7% put, 8% get). |
| 1.0.2-beta  | 2025-11-18 | ~122,000        | ~127,000        | 4.56–5.06             | 4.11–4.17             | Huge performance jump due to removal of autosave from every put/get. Outliers: 7–8% mild/high. |

---

## [1.0.1-beta] - 2025-09-01

### Added
- Initial public release of Aegisr KV Database
- Basic in-memory key-value store
- Optional persistence for durability
- Basic client SDKs for multiple languages

### Changed
- Personal Termenu CLI to Crate:Clap

### Fixed
- Initial bug fixes from beta testing

### Deprecated
- None

### Removed
- None

---

## [1.0.0-beta] - 2025-11-10

- Project started internally (pre-release)