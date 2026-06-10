import sqlite3
import os
from typing import Optional

DB_PATH = os.path.expanduser("~/.local/share/yaru/yaru.db")


def _connect() -> sqlite3.Connection:
    os.makedirs(os.path.dirname(DB_PATH), exist_ok=True)
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def init_db() -> None:
    conn = _connect()
    cursor = conn.cursor()
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS anime_list (
            id          INTEGER PRIMARY KEY,
            title       TEXT NOT NULL,
            episodes    INTEGER,
            score       REAL,
            status      TEXT,
            genres      TEXT,
            year        INTEGER,
            watch_status TEXT DEFAULT 'plan_to_watch',
            added_at    TEXT DEFAULT (datetime('now'))
        )
    """)
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS search_history (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            query       TEXT NOT NULL,
            searched_at TEXT DEFAULT (datetime('now'))
        )
    """)
    conn.commit()
    conn.close()


def add_anime(anime: dict, watch_status: str = "plan_to_watch") -> bool:
    conn = _connect()
    cursor = conn.cursor()
    try:
        title = anime["title"]["english"] or anime["title"]["romaji"]
        genres = ", ".join(anime.get("genres", []))
        cursor.execute("""
            INSERT OR IGNORE INTO anime_list
            (id, title, episodes, score, status, genres, year, watch_status)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        """, (
            anime["id"],
            title,
            anime.get("episodes"),
            anime.get("averageScore"),
            anime.get("status"),
            genres,
            anime.get("seasonYear"),
            watch_status,
        ))
        conn.commit()
        return cursor.rowcount > 0
    except sqlite3.Error as e:
        print(f"[error] db insert failed: {e}")
        return False
    finally:
        conn.close()


def remove_anime(anime_id: int) -> bool:
    conn = _connect()
    cursor = conn.cursor()
    try:
        cursor.execute("DELETE FROM anime_list WHERE id = ?", (anime_id,))
        conn.commit()
        return cursor.rowcount > 0
    except sqlite3.Error as e:
        print(f"[error] db delete failed: {e}")
        return False
    finally:
        conn.close()


def get_list(watch_status: Optional[str] = None) -> list:
    conn = _connect()
    cursor = conn.cursor()
    try:
        if watch_status:
            cursor.execute(
                "SELECT * FROM anime_list WHERE watch_status = ? ORDER BY added_at DESC",
                (watch_status,)
            )
        else:
            cursor.execute("SELECT * FROM anime_list ORDER BY added_at DESC")
        return [dict(row) for row in cursor.fetchall()]
    except sqlite3.Error as e:
        print(f"[error] db fetch failed: {e}")
        return []
    finally:
        conn.close()


def log_search(query: str) -> None:
    conn = _connect()
    cursor = conn.cursor()
    try:
        cursor.execute(
            "INSERT INTO search_history (query) VALUES (?)",
            (query,)
        )
        conn.commit()
    except sqlite3.Error as e:
        print(f"[error] log search failed: {e}")
    finally:
        conn.close()


def get_search_history(limit: int = 20) -> list:
    conn = _connect()
    cursor = conn.cursor()
    try:
        cursor.execute(
            "SELECT query, searched_at FROM search_history ORDER BY searched_at DESC LIMIT ?",
            (limit,)
        )
        return [dict(row) for row in cursor.fetchall()]
    except sqlite3.Error as e:
        print(f"[error] fetch history failed: {e}")
        return []
    finally:
        conn.close()