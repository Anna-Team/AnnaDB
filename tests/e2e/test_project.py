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
            assert v["d"] == {"smth": i, "smth2": 2}

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

    def test_set_value_primitive(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                sort[asc(value|name|)],
                project{
                    s|name|:s|NEW VALUE|
                }
            ]
            """
        )
        for i, (k, v) in enumerate(resp[0]["data"].items()):
            assert set(v._value.keys()) == {"name"}
            assert v["name"] == "NEW VALUE"

    def test_set_value_map(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                sort[asc(value|name|)],
                project{
                    s|name|:m{
                        s|test|:value|smth|,
                    },
                    s|d|:m{
                        s|smth2|:keep
                    }
                }
            ]
            """
        )
        for i, (k, v) in enumerate(resp[0]["data"].items()):
            assert set(v._value.keys()) == {"name", "d"}
            assert v["name"] == {"test": "TEST"}
            assert v["d"] == {"smth2": 2}

    def test_set_value_vector(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                sort[asc(value|name|)],
                project{
                    s|name|:v[
                        value|smth|,
                    ],
                    s|l|:v[s|TEST|,keep,keep]
                }
            ]
            """
        )
        for i, (k, v) in enumerate(resp[0]["data"].items()):
            assert set(v._value.keys()) == {"name", "l"}
            assert v["name"] == ["TEST"]
            assert v["l"] == ["TEST", 8, 7]
