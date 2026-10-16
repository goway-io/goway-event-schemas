BIN := node_modules/.bin
DOCS := events realtime
export ASYNCAPI_METRICS_CONFIG_PATH := $(CURDIR)/build/.asyncapi-analytics
export SUPPRESS_NO_CONFIG_WARNING := 1

tools:
	bun install --frozen-lockfile
	cargo install cargo-typify --version 0.8.0 --locked --root .tools
	cd packages/python && uv sync
	cd packages/dart && dart pub get

bundle:
	python3 scripts/bundle.py
	echo '{"analyticsEnabled":"false","infoMessageShown":"true","userID":"goway"}' > build/.asyncapi-analytics

check: bundle
	for doc in $(DOCS); do $(BIN)/asyncapi validate asyncapi/$$doc.yaml || exit 1; done
	for doc in $(DOCS); do $(BIN)/spectral lint asyncapi/$$doc.yaml --ruleset .spectral.yaml --fail-severity warn || exit 1; done
	uv run --quiet scripts/check_examples.py

breaking:
	python3 scripts/check_breaking.py

generate: bundle
	./scripts/generate.sh

drift: generate
	@stale=$$(git status --porcelain --untracked-files=all -- packages); \
	if [ -n "$$stale" ]; then \
		echo "generated code differs from the commit; run make generate and commit packages/:"; \
		echo "$$stale"; git --no-pager diff --stat -- packages; exit 1; \
	fi

test:
	cd packages/python && uv run ruff check src tests && uv run ruff format --check src tests && uv run pytest -q
	cd packages/rust && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test -q
	$(BIN)/tsc -p packages/ts
	cd packages/dart && dart analyze --fatal-infos && dart test

docs: bundle
	for doc in $(DOCS); do $(BIN)/asyncapi generate fromTemplate asyncapi/$$doc.yaml ./node_modules/@asyncapi/html-template -o build/docs/$$doc --param singleFile=true --force-write || exit 1; done
