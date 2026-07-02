.PHONY: run build test install

VENV := .venv/bin

install:
	python -m venv .venv
	$(VENV)/pip install -e .
	VIRTUAL_ENV=$(PWD)/.venv PATH=$(PWD)/$(VENV):$(PATH) PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo build --release

run:
	$(VENV)/python -c "import yaru_core" 2>/dev/null; VIRTUAL_ENV=$(PWD)/.venv PATH=$(PWD)/$(VENV):$(PATH) PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo run

build:
	VIRTUAL_ENV=$(PWD)/.venv PATH=$(PWD)/$(VENV):$(PATH) PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo build

test:
	$(VENV)/pytest tests/ -v