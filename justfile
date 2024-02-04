alias r := run
alias b := build

default:
	@echo "Run any of the supported front ends"
	@just --list

run FRONTEND:
	cargo run -- {{ FRONTEND }}

build:
	cargo build
