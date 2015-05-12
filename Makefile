.PHONY: run

run:
	cargo run

server:
	cargo run --bin carcasian-server -- --host 127.0.0.1 --port 8991

client:
	cargo run --bin carcasian-client -- --host 127.0.0.1 --port 8991
