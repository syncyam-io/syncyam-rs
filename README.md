# SyncYam rust SDK

[![codecov](https://codecov.io/gh/hgroh/syncyam-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/hgroh/syncyam-rs)
[![Coverage](https://img.shields.io/badge/coverage-0%25-red.svg)](https://github.com/hgroh/syncyam-rs/actions)
[![CI](https://github.com/hgroh/syncyam-rs/actions/workflows/coverage.yml/badge.svg)](https://github.com/hgroh/syncyam-rs/actions/workflows/coverage.yml)

## For development

### Getting started

```shell
# install 
$ make install
# 
$ make enable-jeager 
```

> [!NOTE]
> To enable log output in the tests, you should run test with '--all-features' after running the follows:

```shell
$ make enable-jaeger
$ cargo test --all-features 
```

You can find the traces in the jaeger UI: http://localhost:16686/

### Code Coverage

Code coverage is measured using cargo-tarpaulin:

```shell
# run tarpaulin
$ make tarpaulin

# local update coverage badge
$ make update-coverage-badge
```

### Before pull request
