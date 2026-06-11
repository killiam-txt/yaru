import pytest
from unittest.mock import patch, MagicMock
from yaru_core.api import search_anime, get_anime_details
from yaru_core.db import init_db, add_anime, remove_anime, get_list, log_search, get_search_history


MOCK_SEARCH_RESPONSE = {
    "data": {
        "Page": {
            "pageInfo": {"total": 1, "currentPage": 1, "hasNextPage": False},
            "media": [
                {
                    "id": 1,
                    "title": {"romaji": "Cowboy Bebop", "english": "Cowboy Bebop"},
                    "episodes": 26,
                    "averageScore": 86,
                    "status": "FINISHED",
                    "genres": ["Action", "Drama"],
                    "description": "A ragtag group of bounty hunters.",
                    "seasonYear": 1998,
                }
            ],
        }
    }
}

MOCK_DETAILS_RESPONSE = {
    "data": {
        "Media": {
            "id": 1,
            "title": {"romaji": "Cowboy Bebop", "english": "Cowboy Bebop"},
            "episodes": 26,
            "averageScore": 86,
            "status": "FINISHED",
            "genres": ["Action", "Drama"],
            "description": "A ragtag group of bounty hunters.",
            "seasonYear": 1998,
            "studios": {"nodes": [{"name": "Sunrise"}]},
            "trailer": {"id": "abc123", "site": "youtube"},
        }
    }
}

MOCK_ANIME = {
    "id": 1,
    "title": {"romaji": "Cowboy Bebop", "english": "Cowboy Bebop"},
    "episodes": 26,
    "averageScore": 86,
    "status": "FINISHED",
    "genres": ["Action", "Drama"],
    "seasonYear": 1998,
}


def test_search_anime_returns_results():
    with patch("yaru_core.api.requests.post") as mock_post:
        mock_post.return_value = MagicMock(
            json=lambda: MOCK_SEARCH_RESPONSE,
            raise_for_status=lambda: None,
        )
        result = search_anime("Cowboy Bebop")
        assert result is not None
        assert len(result["media"]) == 1
        assert result["media"][0]["title"]["english"] == "Cowboy Bebop"


def test_search_anime_returns_none_on_failure():
    with patch("yaru_core.api.requests.post") as mock_post:
        mock_post.side_effect = Exception("network error")
        result = search_anime("anything")
        assert result is None


def test_get_anime_details_returns_media():
    with patch("yaru_core.api.requests.post") as mock_post:
        mock_post.return_value = MagicMock(
            json=lambda: MOCK_DETAILS_RESPONSE,
            raise_for_status=lambda: None,
        )
        result = get_anime_details(1)
        assert result is not None
        assert result["id"] == 1
        assert result["title"]["english"] == "Cowboy Bebop"


def test_get_anime_details_returns_none_on_failure():
    with patch("yaru_core.api.requests.post") as mock_post:
        mock_post.side_effect = Exception("network error")
        result = get_anime_details(999)
        assert result is None


def test_init_db_creates_tables(tmp_path, monkeypatch):
    monkeypatch.setattr("yaru_core.db.DB_PATH", str(tmp_path / "test.db"))
    init_db()


def test_add_and_get_anime(tmp_path, monkeypatch):
    monkeypatch.setattr("yaru_core.db.DB_PATH", str(tmp_path / "test.db"))
    init_db()
    result = add_anime(MOCK_ANIME)
    assert result is True
    anime_list = get_list()
    assert len(anime_list) == 1
    assert anime_list[0]["title"] == "Cowboy Bebop"


def test_add_anime_duplicate(tmp_path, monkeypatch):
    monkeypatch.setattr("yaru_core.db.DB_PATH", str(tmp_path / "test.db"))
    init_db()
    add_anime(MOCK_ANIME)
    result = add_anime(MOCK_ANIME)
    assert result is False


def test_remove_anime(tmp_path, monkeypatch):
    monkeypatch.setattr("yaru_core.db.DB_PATH", str(tmp_path / "test.db"))
    init_db()
    add_anime(MOCK_ANIME)
    result = remove_anime(1)
    assert result is True
    assert get_list() == []


def test_log_and_get_search_history(tmp_path, monkeypatch):
    monkeypatch.setattr("yaru_core.db.DB_PATH", str(tmp_path / "test.db"))
    init_db()
    log_search("Cowboy Bebop")
    log_search("Evangelion")
    history = get_search_history()
    assert len(history) == 2
    assert history[0]["query"] == "Evangelion"