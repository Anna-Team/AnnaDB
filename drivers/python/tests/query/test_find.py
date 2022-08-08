from annadb.data_types.constants import root
from annadb.query.find.operators import Not, And, Or


class TestFindAll:
    def test_find_all(self, collection, primitives):
        resp = collection.find().run()
        assert len(resp.data) == primitives.meta.count


class TestFindEq:
    def test_eq_root(self, collection, primitives):
        resp = collection.find(root == 1).run()
        assert len(resp.data) == 1
        for k, v in resp.data.items():
            assert k == primitives.data[0]
            assert v == 1

        resp = collection.find(Not(root == 1)).run()
        assert len(resp.data) == primitives.meta.count - 1
        for k, v in resp.data.items():
            assert v != 1

    def test_eq(self, collection, objects):
        print(root.is_even == True)
        resp = collection.find(root.is_even == True).run()
        assert len(resp.data) == 5
        for k, v in resp.data.items():
            assert v["is_even"] == True

        resp = collection.find(Not(root.is_even == True)).run()
        assert len(resp.data) == 5
        for k, v in resp.data.items():
            assert v["is_even"] == False


class TestFindNeq:
    def test_neq_root(self, collection, primitives):
        resp = collection.find(root != 1).run()
        assert len(resp.data) == primitives.meta.count - 1
        for k, v in resp.data.items():
            assert v != 1

        resp = collection.find(Not(root != 1)).run()
        assert len(resp.data) == 1
        for k, v in resp.data.items():
            assert v == 1

    def test_neq(self, collection, objects):
        resp = collection.find(root.is_even != True).run()
        assert len(resp.data) == 5
        for k, v in resp.data.items():
            assert v["is_even"] == False


class TestFindGt:
    def test_gt_root(self, collection, primitives):
        resp = collection.find(root > 1.2).run()
        assert len(resp.data) == 4
        for k, v in resp.data.items():
            assert v > 1.2

        resp = collection.find(Not(root > 1.2)).run()
        assert len(resp.data) == primitives.meta.count - 4

    def test_gt(self, collection, objects):
        resp = collection.find(root.num > 2.5).run()
        assert len(resp.data) == 7
        for k, v in resp.data.items():
            assert v["num"] > 2


class TestFindGte:
    def test_gte_root(self, collection, primitives):
        resp = collection.find(root >= 2).run()
        assert len(resp.data) == 4
        for k, v in resp.data.items():
            assert v > 1

        resp = collection.find(Not(root >= 2)).run()
        assert len(resp.data) == primitives.meta.count - 4

    def test_gte(self, collection, objects):
        resp = collection.find(root.num >= 2).run()
        assert len(resp.data) == 8
        for k, v in resp.data.items():
            assert v["num"] >= 2


class TestFindLt:
    def test_lt_root(self, collection, primitives):
        resp = collection.find(root < 4).run()
        assert len(resp.data) == 3
        for k, v in resp.data.items():
            assert v < 4

        resp = collection.find(Not(root < 4)).run()
        assert len(resp.data) == primitives.meta.count - 3

    def test_lt(self, collection, objects):
        resp = collection.find(root.num < 6).run()
        assert len(resp.data) == 6
        for k, v in resp.data.items():
            assert v["num"] < 6


class TestFindLte:
    def test_lte_root(self, collection, primitives):
        resp = collection.find(root <= 4).run()
        assert len(resp.data) == 4
        for k, v in resp.data.items():
            assert v <= 4

        resp = collection.find(Not(root <= 4)).run()
        assert len(resp.data) == primitives.meta.count - 4

    def test_lte(self, collection, objects):
        resp = collection.find(root.num <= 6).run()
        assert len(resp.data) == 7
        for k, v in resp.data.items():
            assert v["num"] <= 6


class TestFindAnd:
    def test_and_root(self, collection, primitives):
        resp = collection.find((root > 2) & (root < 5)).run()
        assert len(resp.data) == 2
        for k, v in resp.data.items():
            assert 2 < v < 5

    def test_and(self, collection, objects):
        resp = collection.find(And(root.num > 4, root.is_even == True)).run()
        assert len(resp.data) == 2
        for k, v in resp.data.items():
            assert v["num"] > 4
            assert v["is_even"] == True


class TestFindOr:
    def test_or_root(self, collection, primitives):
        resp = collection.find((root > 4) | (root < 2)).run()
        assert len(resp.data) == 2
        for k, v in resp.data.items():
            assert v > 4 or v < 2

    def test_or(self, collection, objects):
        resp = collection.find(Or(root.num > 7, root.is_even == True)).run()
        assert len(resp.data) == 6
        for k, v in resp.data.items():
            assert v["num"] > 7 or v["is_even"] == True


class TestNot:
    def test_not_root(self, collection, primitives):
        resp = collection.find(Not(root == 4)).run()
        assert len(resp.data) == primitives.meta.count - 1
        for k, v in resp.data.items():
            assert v != 4

    def test_not(self, collection, objects):
        resp = collection.find(Not(root.num <= 6)).run()
        assert len(resp.data) == 3
        for k, v in resp.data.items():
            assert not v["num"] <= 6


class TestOther:
    def test_field_not_exists(self, collection, objects):
        resp = collection.find(root.blink.smth <= 6).run()
        assert len(resp.data) == 3
        for k, v in resp.data.items():
            assert v["blink"]["smth"] <= 6

        resp = collection.find(Not(root.blink.smth <= 6)).run()
        assert len(resp.data) == 10 - 3

    def test_two_find_queries(self, collection, objects):
        resp = collection.find(root.num > 4).find(root.is_even == True).run()
        assert len(resp.data) == 2
        for k, v in resp.data.items():
            assert v["num"] > 4
            assert v["is_even"] == True

    def test_many_parameters(self, collection, objects):
        resp = collection.find(root.num > 4, root.is_even == True).run()
        assert len(resp.data) == 2
        for k, v in resp.data.items():
            assert v["num"] > 4
            assert v["is_even"] == True

    def test_by_linked_field_primitive(self, collection):
        resp = collection.insert("bar").run()
        id = resp.data[0]
        collection.insert({"foo": id}).run()
        resp = collection.find(root.foo == "bar").run()
        assert resp.meta.count == 1

    def test_by_linked_field_container(self, collection):
        resp = collection.insert({"bar": "baz"}).run()
        id = resp.data[0]
        collection.insert({"foo": id}).run()
        resp = collection.find(root.bar == "baz").run()
        assert resp.meta.count == 1
