import requests
from typing import Optional

ANILIST_ENDPOINT = "https://graphql.anilist.co"

SEARCH_QUERY = """
query ($search: String, $page: Int, $per_page: Int) {
    Page (page: $page, perPage: $per_page) {
        pageInfo {
            total
            currentPage
            hasNextPage
        }
        media (search: $search, type: ANIME) {
            id
            title {
                romaji
                english
            }
            episodes
            averageScore
            status
            genres
            description (asHtml: false)
            seasonYear
        }
    }
}
"""

DETAILS_QUERY = """
query ($id: Int) {
    Media (id: $id, type: ANIME) {
        id
        title {
            romaji
            english
        }
        episodes
        averageScore
        status
        genres
        description (asHtml: false)
        seasonYear
        studios {
            nodes {
                name
            }
        }
        trailer {
            id
            site
        }
    }
}
"""


def _post(query: str, variables: dict) -> dict:
    response = requests.post(
        ANILIST_ENDPOINT,
        json={"query": query, "variables": variables},
        timeout=10,
    )
    response.raise_for_status()
    return response.json()


def search_anime(search: str, page: int = 1, per_page: int = 10) -> Optional[dict]:
    variables = {
        "search": search,
        "page": page,
        "per_page": per_page,
    }
    try:
        data = _post(SEARCH_QUERY, variables)
        return data["data"]["Page"]
    except (requests.RequestException, KeyError) as e:
        print(f"[error] search failed: {e}")
        return None


def get_anime_details(anime_id: int) -> Optional[dict]:
    variables = {"id": anime_id}
    try:
        data = _post(DETAILS_QUERY, variables)
        return data["data"]["Media"]
    except (requests.RequestException, KeyError) as e:
        print(f"[error] fetch details failed: {e}")
        return None