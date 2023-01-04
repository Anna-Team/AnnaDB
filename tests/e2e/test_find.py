from annadb.dump import to_str


class TestFindAll:
    def test_find_all(self, conn, primitives):
        resp = conn.send_query(
            """
            collection|test|:find[]
            """
        )
        assert len(resp[0]["data"]) == primitives["meta"]["count"]


class TestFindEq:
    def test_eq_root(self, conn, primitives):
        resp = conn.send_query(
            """
            collection|test|:find[
                eq{root:n|1|}
            ]
            """
        )
        assert len(resp[0]["data"]) == 1
        for k, v in resp[0]["data"].items():
            assert k == primitives["data"][0]
            assert v == 1

        resp = conn.send_query(
            """
            collection|test|:find[
                not(eq{root:n|1|})
            ]
            """
        )
        assert len(resp[0]["data"]) == primitives["meta"]["count"] - 1
        for k, v in resp[0]["data"].items():
            assert v != 1

    def test_eq(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:find[
                eq{value|is_even|:b|true|}
            ]
            """
        )
        assert len(resp[0]["data"]) == 5
        for k, v in resp[0]["data"].items():
            assert v["is_even"] == True

        resp = conn.send_query(
            """
            collection|test|:find[
                not(eq{value|is_even|:b|true|})
            ]
            """
        )
        assert len(resp[0]["data"]) == 5
        for k, v in resp[0]["data"].items():
            assert v["is_even"] == False


class TestFindNeq:
    def test_neq_root(self, conn, primitives):
        resp = conn.send_query(
            """
            collection|test|:find[
                neq{root:n|1|}
            ]
            """
        )
        print(resp[0]["data"])
        assert len(resp[0]["data"]) == primitives["meta"]["count"] - 1
        for k, v in resp[0]["data"].items():
            assert v != 1

        resp = conn.send_query(
            """
            collection|test|:find[
                not(neq{root:n|1|})
            ]
            """
        )
        assert len(resp[0]["data"]) == 1
        for k, v in resp[0]["data"].items():
            assert v == 1

    def test_neq(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:find[
                neq{value|is_even|:b|true|}
            ]
            """
        )
        assert len(resp[0]["data"]) == 5
        for k, v in resp[0]["data"].items():
            assert v["is_even"] == False


class TestFindGt:
    def test_gt_root(self, conn, primitives):
        resp = conn.send_query(
            """
            collection|test|:find[
                gt{root:n|1.2|}
            ]
            """
        )
        assert len(resp[0]["data"]) == 4
        for k, v in resp[0]["data"].items():
            assert v > 1.2

        resp = conn.send_query(
            """
            collection|test|:find[
                not(gt{root:n|1.2|})
            ]
            """
        )
        assert len(resp[0]["data"]) == primitives["meta"]["count"] - 4

    def test_gt(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:find[
                gt{value|num|:n|2.5|}
            ]
            """
        )
        assert len(resp[0]["data"]) == 7
        for k, v in resp[0]["data"].items():
            assert v["num"] > 2


class TestFindGte:
    def test_gte_root(self, conn, primitives):
        resp = conn.send_query(
            """
            collection|test|:find[
                gte{root:n|2|}
            ]
            """
        )
        assert len(resp[0]["data"]) == 4
        for k, v in resp[0]["data"].items():
            assert v > 1

        resp = conn.send_query(
            """
            collection|test|:find[
                not(gte{root:n|2|})
            ]
            """
        )
        assert len(resp[0]["data"]) == primitives["meta"]["count"] - 4

    def test_gte(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:find[
                gte{value|num|:n|2|}
            ]
            """
        )
        assert len(resp[0]["data"]) == 8
        for k, v in resp[0]["data"].items():
            assert v["num"] >= 2


class TestFindLt:
    def test_lt_root(self, conn, primitives):
        resp = conn.send_query(
            """
            collection|test|:find[
                lt{root:n|4|}
            ]
            """
        )
        assert len(resp[0]["data"]) == 3
        for k, v in resp[0]["data"].items():
            assert v < 4

        resp = conn.send_query(
            """
            collection|test|:find[
                not(lt{root:n|4|})
            ]
            """
        )
        assert len(resp[0]["data"]) == primitives["meta"]["count"] - 3

    def test_lt(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:find[
                lt{value|num|:n|6|}
            ]
            """
        )
        print(resp[0]["data"])
        assert len(resp[0]["data"]) == 6
        for k, v in resp[0]["data"].items():
            assert v["num"] < 6


class TestFindLte:
    def test_lte_root(self, conn, primitives):
        resp = conn.send_query(
            """
            collection|test|:find[
                lte{root:n|4|}
            ]
            """
        )
        assert len(resp[0]["data"]) == 4
        for k, v in resp[0]["data"].items():
            assert v <= 4

        resp = conn.send_query(
            """
            collection|test|:find[
                not(lte{root:n|4|})
            ]
            """
        )
        assert len(resp[0]["data"]) == primitives["meta"]["count"] - 4

    def test_lte(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:find[
                lte{value|num|:n|6|}
            ]
            """
        )
        assert len(resp[0]["data"]) == 7
        for k, v in resp[0]["data"].items():
            assert v["num"] <= 6


class TestFindAnd:
    def test_and_root(self, conn, primitives):
        resp = conn.send_query(
            """
            collection|test|:find[
                and[
                    gt{root:n|2|},
                    lt{root:n|5|}
                ]
            ]
            """
        )
        assert len(resp[0]["data"]) == 2
        for k, v in resp[0]["data"].items():
            assert 2 < v < 5

    def test_and(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:find[
                and[
                    gt{value|num|:n|4|},
                    eq{value|is_even|:b|true|},
                ]
            ]
            """
        )
        assert len(resp[0]["data"]) == 2
        for k, v in resp[0]["data"].items():
            assert v["num"] > 4
            assert v["is_even"] == True


class TestFindOr:
    def test_or_root(self, conn, primitives):
        resp = conn.send_query(
            """
            collection|test|:find[
                or[
                    gt{root:n|4|},
                    lt{root:n|2|}
                ]
            ]
            """
        )
        assert len(resp[0]["data"]) == 2
        for k, v in resp[0]["data"].items():
            assert v > 4 or v < 2

    def test_or(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:find[
                or[
                    gt{value|num|:n|7|},
                    eq{value|is_even|:b|true|},
                ]
            ]
            """
        )
        assert len(resp[0]["data"]) == 6
        for k, v in resp[0]["data"].items():
            assert v["num"] > 7 or v["is_even"] == True


class TestNot:
    def test_not_root(self, conn, primitives):
        resp = conn.send_query(
            """
            collection|test|:find[
                not(eq{root:n|4|})
            ]
            """
        )
        assert len(resp[0]["data"]) == primitives["meta"]["count"] - 1
        for k, v in resp[0]["data"].items():
            assert v != 4

    def test_not(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:find[
                not(lte{value|num|:n|6|})
            ]
            """
        )
        assert len(resp[0]["data"]) == 3
        for k, v in resp[0]["data"].items():
            assert not v["num"] <= 6


class TestOther:
    def test_field_not_exists(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:find[
                lte{value|blink.smth|:n|6|}
            ]
            """
        )
        assert len(resp[0]["data"]) == 3
        for k, v in resp[0]["data"].items():
            assert v["blink"]["smth"] <= 6

        resp = conn.send_query(
            """
            collection|test|:find[
                not(lte{value|blink.smth|:n|6|})
            ]
            """
        )
        assert len(resp[0]["data"]) == 10 - 3

    def test_two_find_queries(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:q[
                find[
                    gt{value|num|:n|4|},
                ],
                find[
                    eq{value|is_even|:b|true|},
                ]
            ]
            """
        )
        assert len(resp[0]["data"]) == 2
        for k, v in resp[0]["data"].items():
            assert v["num"] > 4
            assert v["is_even"] == True

    def test_many_parameters(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:q[
                find[
                    gt{value|num|:n|4|},
                    eq{value|is_even|:b|true|},
                ]
            ]
            """
        )
        assert len(resp[0]["data"]) == 2
        for k, v in resp[0]["data"].items():
            assert v["num"] > 4
            assert v["is_even"] == True

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
        collection|test|:find[
            eq{value|foo|:s|bar|}
        ]
        """
        )
        assert resp[0]["meta"]["count"] == 1

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
        collection|test|:find[
            eq{value|foo.bar|:s|baz|}
        ]
        """
        )
        assert resp[0]["meta"]["count"] == 1
