def test_multi_collection(conn):
    query_1 = """
    collection|test|: insert[
                          s|foo|,
                      ];
    collection|test2|: insert[
                n|100|,
            ]
    """
    conn.send_query(query_1)

    query_2 = """
    collection|test|: find[];
    collection|test2|: find[];
    """
    resp = conn.send_query(query_2)
    for i in resp[0]["data"]._value.values():
        assert i == "foo"
    for i in resp[1]["data"]._value.values():
        assert i == 100
