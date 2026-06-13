.PHONY: run build test

VENV := .venv/bin

run:
	$(VENV)/python -c "import yaru_core" 2>/dev/null; VIRTUAL_ENV=$(PWD)/.venv PATH=$(PWD)/$(VENV):$(PATH) cargo run

build:
	VIRTUAL_ENV=$(PWD)/.venv PATH=$(PWD)/$(VENV):$(PATH) cargo build

test:
	$(VENV)/pytest tests/ -v