"""
AnnaDB Python Client — embedded, no server needed.

Usage:
    from annadb import AnnaDB

    db = AnnaDB.open("warehouse")       # persistent
    db = AnnaDB.open(":memory:")        # temporary

    link = db.remember("facts", "User lives in Paris", key=("name", "paris"))
    docs = db.recall("facts", "where does user live?", k=5)
    edge = db.relate(from_link, to_link, "knows")
    neighbors = db.neighbors(link)
    traversed = db.traverse(link, max_depth=3)
    path = db.path(from_link, to_link, max_depth=5)
    db.forget(link)

    # Raw TySON
    result = db.exec('find s|collection|facts|')

    # Vector index (required for recall)
    db.create_vector_index("facts", "embedding", 1536, 16, 200, "cosine")

    # With OpenAI embedding provider (set env vars)
    # EMBEDDING_PROVIDER=openai EMBEDDING_MODEL=text-embedding-3-small
    # OPENAI_API_KEY=sk-...
"""

from ._annadb import AnnaDB
