# SyncYam rust SDK

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

### Before pull request
