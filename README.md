# yaru

anime search CLI for linux with a TUI interface.

built with Rust + Python (PyO3), uses AniList's GraphQL API.
> I watch anime through ani-cli and never wanted to leave the terminal just to look up info. So I built this.

![](https://i.ibb.co/hFRmffZh/1781929773.png)

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
| i | insert mode |
| esc | normal mode / quit |
| enter | search |
| up/down | navigate |
| tab | details |
| l | add to watchlist |
| w | open watchlist |
| s | watch status |
| d | remove from watchlist |

## stack

- rust — TUI (ratatui, crossterm)
- python — AniList API client, SQLite storage (yaru_core)
- pyo3 — rust/python interop

## license

MIT