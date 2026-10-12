# yaru

anime search TUI for linux.

built with Rust + Python (PyO3), uses AniList's GraphQL API.
> I watch anime through ani-cli and never wanted to leave the terminal just to look up info. So I built this.

<img width="1920" height="1080" alt="1781929773" src="https://github.com/user-attachments/assets/de09aace-e64e-4a16-aa31-59fc231c1a4a" />

## requirements

- rust (cargo)
- python 3.11+

## install

```bash
git clone https://github.com/killiam-txt/yaru.git
cd yaru
make install
```

## usage

```bash
make run
```

| key | action |
|-----|--------|
| `i` | insert mode |
| `esc` | normal mode / quit |
| `enter` | search |
| `up/down` | navigate |
| `tab` | details |
| `l` | add to watchlist |
| `w` | open watchlist |
| `s` | watch status |
| `d` | remove from watchlist |

## stack

- rust — TUI (ratatui, crossterm)
- python — AniList API client, SQLite storage (yaru_core)
- pyo3 — rust/python interop

## license

MIT
