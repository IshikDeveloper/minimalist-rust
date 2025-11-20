# Makefile for Minimalist Browser

.PHONY: all build release clean setup test run

# Detect OS
UNAME := $(shell uname)
ifeq ($(UNAME), Linux)
	SETUP_SCRIPT = ./setup.sh
	BROWSER_BIN = target/release/minimalist-browser
endif
ifeq ($(UNAME), Darwin)
	SETUP_SCRIPT = ./setup.sh
	BROWSER_BIN = target/release/minimalist-browser
endif
ifdef COMSPEC
	SETUP_SCRIPT = powershell -ExecutionPolicy Bypass -File setup.ps1
	BROWSER_BIN = target\release\minimalist-browser.exe
endif

all: setup build

setup:
	@echo "Setting up browser environment..."
	@$(SETUP_SCRIPT)

build:
	@echo "Building browser (debug mode)..."
	@cargo build

release:
	@echo "Building browser (release mode)..."
	@cargo build --release --profile minimal 2>/dev/null || cargo build --release

clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf browser_data/cache/*

test:
	@echo "Running tests..."
	@cargo test
	@./test_browser.sh

run: release
	@echo "Starting Minimalist Browser..."
	@$(BROWSER_BIN)

install: release
	@echo "Installing browser..."
	@cargo install --path .

uninstall:
	@echo "Uninstalling browser..."
	@cargo uninstall minimalist-browser

# Development commands
dev:
	@cargo watch -x run

fmt:
	@cargo fmt

lint:
	@cargo clippy -- -D warnings

# Memory profiling (Linux only)
profile:
	@valgrind --tool=massif $(BROWSER_BIN)
	@ms_print massif.out.*

help:
	@echo "Minimalist Browser - Build Commands"
	@echo "===================================="
	@echo "make setup    - Download dependencies and set up environment"
	@echo "make build    - Build debug version"
	@echo "make release  - Build optimized release version"
	@echo "make run      - Build and run the browser"
	@echo "make test     - Run test suite"
	@echo "make clean    - Clean build artifacts"
	@echo "make install  - Install browser system-wide"
	@echo "make help     - Show this help message"