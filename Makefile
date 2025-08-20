.PHONY: install
install:
	cargo install cargo-tarpaulin

.PHONY: lint
lint:
	cargo +nightly fmt --check
	cargo check --all-features --tests
	cargo clippy --tests --all-features  -- -D warnings

.PHONY: tarpaulin
tarpaulin:
	SYNCYAM_RS_OTEL_ENABLED=true cargo tarpaulin -o html --all-features --engine llvm --output-dir ./coverage
	open coverage/tarpaulin-report.html

.PHONY: enable-jeager
enable-jeager:
	export SYNCYAM_RS_OTEL_ENABLED=true
	docker run --rm -d --name jaeger \
      -e COLLECTOR_ZIPKIN_HOST_PORT=:9411 -e COLLECTOR_OTLP_ENABLED=true -p 16686:16686 \
      -p 14268:14268 -p 4317:4317 \
      -p 4318:4318 \
      -p 5778:5778 \
      -p 9411:9411 \
      cr.jaegertracing.io/jaegertracing/jaeger:latest

doc:
	cargo doc --open