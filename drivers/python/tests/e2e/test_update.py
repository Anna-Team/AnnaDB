from annadb.dump import to_str


class TestUpdateInc:
    def test_root(self, conn, primitives):
        conn.send_query(
            """
            collection|test|:q[
                find[
                    eq{root:n|1|}
                ],
                update[
                    inc{root:n|2|}
                ]
            ];"""
        )
        resp = conn.send_query(
            f"""collection|test|:q[
                get[
                    {to_str(primitives["data"][0])}
                ]
            ];
            """
        )
        for k, v in resp[0]["data"].items():
            assert k == primitives["data"][0]
            assert v == 3

    def test_object(self, conn, objects):
        conn.send_query(
            """
            collection|test|:q[
                find[
                    eq{value|is_even|:b|true|}
                ],
                update[
                    inc{value|num|:n|100|}
                ]
            ];"""
        )
        resp = conn.send_query(
            """collection|test|:find[
                    eq{value|is_even|:b|true|}
                ]
            """
        )
        for k, v in resp[0]["data"].items():
            assert v["num"] >= 100


class TestUpdateSet:
    def test_root(self, conn, primitives):
        conn.send_query(
            """
            collection|test|:q[
                find[
                    eq{root:n|1|}
                ],
                update[
                    set{root:n|100|}
                ]
            ];"""
        )
        resp = conn.send_query(
            f"""collection|test|:q[
                get[
                    {to_str(primitives["data"][0])}
                ]
            ];
            """
        )
        for k, v in resp[0]["data"].items():
            assert k == primitives["data"][0]
            assert v == 100

    def test_object(self, conn, objects):
        conn.send_query(
            """
            collection|test|:q[
                find[
                    eq{value|is_even|:b|true|}
                ],
                update[
                    set{value|num|:n|100|}
                ]
            ];"""
        )
        resp = conn.send_query(
            """collection|test|:find[
                    eq{value|is_even|:b|true|}
                ]
            """
        )
        for k, v in resp[0]["data"].items():
            assert v["num"] == 100


class TestUpdateOther:
    def test_field_not_exists_set(self, conn, objects):
        conn.send_query(
            """
            collection|test|:q[
                find[],
                update[
                    set{value|blink2.a|:n|100|}
                ]
            ];
            """
        )
        conn.send_query(
            """
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

    def test_field_not_exists_inc(self, conn, objects):
        conn.send_query(
            """
            collection|test|:q[
                find[],
                update[
                    inc{value|blink2.a|:n|100|}
                ]
            ];
            """
        )
        resp = conn.send_query(
            """collection|test|:find[]
            """
        )
        for k, v in resp[0]["data"].items():
            if "a" in v["blink2"]._value:
                assert v["blink2"]["a"] == 200

    def test_double_update(self, conn, objects):
        conn.send_query(
            """
            collection|test|:q[
                find[
                    eq{value|is_even|:b|true|}
                ],
                update[
                    set{value|blink|:n|100|},
                    set{value|num|:n|200|},
                ]
            ];"""
        )
        resp = conn.send_query(
            """collection|test|:find[
                    eq{value|is_even|:b|true|}
                ]
            """
        )
        for k, v in resp[0]["data"].items():
            assert v["blink"] == 100
            assert v["num"] == 200

    def test_by_linked_field_primitive(self, conn):
        resp = conn.send_query(
            """
            collection|test|:q[
                insert[
                    s|bar|
                ]
            ]
            """
        )
        id = resp[0]["data"][0]

        conn.send_query(
            f"""
            collection|test|:q[
                insert[
                    m{{
                        s|foo|: {to_str(id)}
                    }}
                ]
            ]
            """
        )

        resp = conn.send_query(
            """
        collection|test|:q[
            find[
                eq{value|foo|:s|bar|},
            ],
            update[
                set{value|foo|:s|baz|},
            ]
        ]
        """
        )
        assert resp[0]["meta"]["count"] == 1

        resp = conn.send_query(
            f"""
                collection|test|:get[{to_str(id)}]
                """
        )
        for k, v in resp[0]["data"].items():
            assert v == "bar"

    def test_by_linked_field_container(self, conn):
        resp = conn.send_query(
            """
            collection|test|:q[
                insert[
                    m{s|bar|:s|baz|}
                ]
            ]
            """
        )
        id = resp[0]["data"][0]

        conn.send_query(
            f"""
                    collection|test|:q[
                        insert[
                            m{{
                                s|foo|: {to_str(id)}
                            }}
                        ]
                    ]
                    """
        )

        resp = conn.send_query(
            """
                collection|test|:q[
                    find[
                        eq{value|foo.bar|:s|baz|},
                    ],
                    update[
                        set{value|foo.bar|:s|bazzz|},
                    ]
                ]
                """
        )
        assert resp[0]["meta"]["count"] == 1

        resp = conn.send_query(
            f"""
                        collection|test|:get[{to_str(id)}]
                        """
        )
        for k, v in resp[0]["data"].items():
            assert v["bar"] == "bazzz"

    def test_update_updated(self, conn):
        conn.send_query(
            """
            collection|test|:q[
                insert[
                    m{
                        s|bar|:s|baz|,
                        s|bar_2|:s|baz_2|,
                    }
                ]
            ]
            """
        )
        conn.send_query(
            """
                        collection|test|:q[
                            find[],
                            update[
                                set{value|bar|:s|bazzz|},
                            ]
                        ],
                        collection|test|:q[
                            find[],
                            update[
                                set{value|bar_2|:s|bazzz_2|},
                            ]
                        ]
                        """
        )
        resp = conn.send_query("collection|test|:find[]")
        for k, v in resp[0]["data"].items():
            assert v["bar"] == "bazzz"
            assert v["bar_2"] == "bazzz_2"
