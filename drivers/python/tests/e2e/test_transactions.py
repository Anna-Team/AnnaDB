from annadb.data_types.primitive import Deleted


class TestTransactions:
    def test_find(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:find[
                    gt{value|num|:n|4|},
                ],
            collection|test|:find[
                    eq{value|is_even|:b|true|},
                ]
            """
        )
        assert len(resp[0]["data"]) == 5
        assert len(resp[1]["data"]) == 5
        for k, v in resp[0]["data"].items():
            assert v["num"] > 1
        for k, v in resp[1]["data"].items():
            assert v["is_even"] == True

    def test_update(self, conn, objects):
        conn.send_query(
            """
            collection|test|:q[
                find[],
                update[
                    set{value|blink2.a|:n|100|}
                ]
            ];
            collection|test|:q[
                find[],
                update[
                    set{value|blink2.e|:n|1000|}
                ]
            ];
            """
        )
        resp = conn.send_query(
            """collection|test|:find[]
            """
        )
        for k, v in resp[0]["data"].items():
            assert v["blink2"]["a"] == 100
            assert v["blink2"]["e"] == 1000

    def test_update_and_find(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                update[
                    set{value|blink2.a|:n|100|}
                ]
            ];
            collection|test|:q[
                find[],
                update[
                    set{value|blink2.e|:n|1000|}
                ]
            ];
            collection|test|:find[];
            """
        )
        for k, v in resp[2]["data"].items():
            assert v["blink2"]["a"] == 100
            assert v["blink2"]["e"] == 1000

    def test_insert(self, conn):
        conn.send_query(
            """
            collection|test|: insert[
                m{
                    s|foo|:s|bar|,
                    s|vec|:v[
                        n|101|,
                        n|102|,
                        n|103|,
                    ]
                },
            ];
            collection|test|: insert[
                m{
                    s|foo|:s|bar|,
                    s|vec|:v[
                        n|101|,
                        n|102|,
                        n|103|,
                    ]
                },
            ];
            """
        )
        resp = conn.send_query(
            """collection|test|:find[]
            """
        )
        assert len(resp[0]["data"]) == 2
        for k, v in resp[0]["data"].items():
            assert v["vec"][0] == 101
            assert v["foo"] == "bar"

    def test_insert_find(self, conn):
        resp = conn.send_query(
            """
            collection|test|: insert[
                m{
                    s|foo|:s|bar|,
                    s|vec|:v[
                        n|101|,
                        n|102|,
                        n|103|,
                    ]
                },
            ];
            collection|test|: insert[
                m{
                    s|foo|:s|bar|,
                    s|vec|:v[
                        n|101|,
                        n|102|,
                        n|103|,
                    ]
                },
            ];
            collection|test|:find[];
            """
        )
        assert len(resp[2]["data"]) == 2
        for k, v in resp[2]["data"].items():
            assert v["vec"][0] == 101
            assert v["foo"] == "bar"

    def test_insert_delete_collection_insert_find(self, conn):
        resp = conn.send_query(
            """
            collection|test|: insert[
                m{
                    s|foo|:s|bar|,
                    s|vec|:v[
                        n|101|,
                        n|102|,
                        n|103|,
                    ]
                },
            ];
            collection|test|: delete;
            collection|test|: insert[
                m{
                    s|foo|:s|bar|,
                    s|vec|:v[
                        n|101|,
                        n|102|,
                        n|103|,
                    ]
                },
            ];
            collection|test|:find[];
            """
        )
        assert len(resp[3]["data"]) == 1
        for k, v in resp[3]["data"].items():
            assert v["vec"][0] == 101
            assert v["foo"] == "bar"

    def test_insert_delete_insert_find(self, conn):
        resp = conn.send_query(
            """
            collection|test|: insert[
                m{
                    s|foo|:s|bar|,
                    s|vec|:v[
                        n|101|,
                        n|102|,
                        n|103|,
                    ]
                },
            ];
            collection|test|: q[
                find[],
                delete
            ];
            collection|test|: insert[
                m{
                    s|foo|:s|bar|,
                    s|vec|:v[
                        n|101|,
                        n|102|,
                        n|103|,
                    ]
                },
            ];
            collection|test|:find[];
            """
        )
        assert len(resp[3]["data"]) == 2  # TODO clean deleted
        for k, v in resp[3]["data"].items():
            if v != Deleted():
                assert v["vec"][0] == 101
                assert v["foo"] == "bar"
