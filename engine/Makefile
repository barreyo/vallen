
PROJECT_NAME 	:= "vallen-engine"
CARGO_VERSION	?= $(shell awk -F '[ -]*' 'NR==3{print $$NF;exit}' Cargo.toml)
GIT_VERSION		?= $(shell echo `git describe --long --always --match=version-* --dirty` | sed s/version-//g)
FULL_VERSION	?= $(CARGO_VERSION)-$(GIT_VERSION)

# Formatting variables
DISPLAY_BOLD 	:= "\033[01m"
DISPLAY_RESET 	:= "\033[0;0m"

.PHONY: version test lint

version:
	@echo $(FULL_VERSION)

test:
	@echo $(DISPLAY_BOLD) "--- Testing $(FULL_VERSION)" $(DISPLAY_RESET)
	@cargo test

lint: test
