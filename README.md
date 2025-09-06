# SyncYam rust SDK

[![codecov](https://codecov.io/gh/syncyam-io/syncyam-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/syncyam-io/syncyam-rs)
[![CI](https://github.com/syncyam-io/syncyam-rs/actions/workflows/coverage.yml/badge.svg)](https://github.com/syncyam-io/syncyam-rs/actions/workflows/coverage.yml)
![GitHub commit activity](https://img.shields.io/github/commit-activity/w/syncyam-io/syncyam-rs)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/syncyam-io/syncyam-rs/build-test-coverage.yml)

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
