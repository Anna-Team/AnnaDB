from annadb.data_types.map import Map
from annadb.data_types.primitive import Number, String, Bool, Null
from annadb.data_types.vector import Vector
from annadb.dump import to_tyson, to_str
from annadb.query.find.operators import Not


class TestPrimitives:
    def test_number(self):
        assert to_tyson(1) == Number(1)
        assert to_str(Number(1)) == "n|1|"

        assert to_tyson(1.1) == Number(1.1)
        assert to_str(Number(1.1)) == "n|1.1|"

    def test_str(self):
        assert to_tyson("test") == String("test")
        assert to_str(String("test")) == "s|test|"

        assert (
            to_str(String("smth | !@#$%^&*() 12345 \\"))
            == "s|smth \\| !@#$%^&*() 12345 \\\\|"
        )

        assert (
            to_str(
                String(
                    """
        1
        2
        3
        4
        """
                )
            )
            == "s|\n        1\n        2\n        3\n        4\n        |"
        )

        assert (
            String.from_serialized("smth \\| !@#$%^&*() 12345 \\\\")
            == "smth | !@#$%^&*() 12345 \\"
        )

    def test_bool(self):
        assert to_tyson(True) == Bool(True)
        assert to_str(Bool(True)) == "b|true|"

    def test_null(self):
        assert to_tyson(None) == Null()
        assert to_str(Null()) == "null"


class TestContainer:
    def test_modifier(self):
        assert to_str(Not(String("test"))) == "not(s|test|)"

    def test_vector(self):
        assert (
            to_str(Vector(String("test"), String("test")))
            == "v[s|test|,s|test|,]"
        )

    def test_map(self):
        assert (
            to_str(Map({String("test"): String("test")}))
            == "m{s|test|:s|test|,}"
        )
