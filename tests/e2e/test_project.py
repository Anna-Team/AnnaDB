class TestProject:
    def test_keep(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                sort[asc(value|name|)],
                project{
                    s|name|:keep,
                    s|d|:keep,
                }
            ]
            """
        )
        for i, (k, v) in enumerate(resp[0]["data"].items()):
            assert set(v._value.keys()) == {"name", "d"}
            assert v["name"] == f"test_{i}"
            assert v["d"] == {"smth": i}

    def test_set_by_path(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                sort[asc(value|name|)],
                project{
                    s|name|:value|smth|
                }
            ]
            """
        )
        for i, (k, v) in enumerate(resp[0]["data"].items()):
            assert set(v._value.keys()) == {"name"}
            assert v["name"] == "TEST"
