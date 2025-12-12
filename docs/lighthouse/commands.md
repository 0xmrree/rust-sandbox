### run single test
cargo nextest run -p lighthouse telemetry_sample_rate_config

### run tests in package
cargo nextest run -p lighthouse

### run all tests with runner parallelism
make test-release

