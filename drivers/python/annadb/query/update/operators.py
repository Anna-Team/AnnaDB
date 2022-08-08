from annadb.constants import SET_OPERATOR, INC_OPERATOR
from annadb.data_types.map import MapUnique


class Set(MapUnique):
    """
    Set({"key": "value"})
    """

    prefix = SET_OPERATOR


class Inc(MapUnique):
    prefix = INC_OPERATOR
