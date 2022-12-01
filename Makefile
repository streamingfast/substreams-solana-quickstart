.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams-solana-tutorial.yaml sol_basic_mapper -s 12292922 -t +10

.PHONY: codegen
codegen:
	substreams protogen ./substreams-solana-tutorial.yaml --exclude-paths="sf/substreams,google"
	