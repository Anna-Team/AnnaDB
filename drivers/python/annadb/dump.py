from annadb.exceptions import ConvertError
from annadb.data_types.map import Map, MapBase, MapUnique, MapNotUnique
from annadb.data_types.modifier import ModifierBase
from annadb.data_types.primitive import (
    Number,
    String,
    Bool,
    Null,
    PrimitiveBase,
)
from annadb.data_types.tyson import TysonItem
from annadb.data_types.vector import Vector, VectorBase
from annadb.query.query_set import QuerySet

NoneType = type(None)

TYPE_TO_TYSON = {
    int: lambda o: Number(o),
    float: lambda o: Number(o),
    str: lambda o: String(o),
    bool: lambda o: Bool(o),
    NoneType: lambda o: Null(o),
}


def to_tyson(item):
    if isinstance(item, list):
        return Vector([to_tyson(i) for i in item])
    if isinstance(item, dict):
        return Map({to_tyson(k): to_tyson(v) for k, v in item.items()})
    if isinstance(item, tuple(TYPE_TO_TYSON.keys())):
        return TYPE_TO_TYSON[type(item)](item)
    if isinstance(item, TysonItem):
        if isinstance(item, VectorBase):
            item._value = [to_tyson(o) for o in item._value]
        if isinstance(item, MapBase):
            item.items = [
                (to_tyson(k), to_tyson(v)) for k, v in item.items.items()
            ]
        if isinstance(item, ModifierBase):
            item.value = to_tyson(item.value)
        return item
    raise ConvertError(f"Type {type(item)} is not TySON serializable")


def to_str(data):
    # Extra types
    if isinstance(data, QuerySet):
        res = []
        for query in data.queries:
            res.append(to_str(query))
        serialized_queries = "q[" + ",".join(res) + "]"
        return f"collection|{data.collection}|:{serialized_queries}"

    # TySON types
    if isinstance(data, String):
        data.value = data.value.replace("\\", "\\\\").replace("|", "\\|")
        return f"{data.prefix}|{data.value}|"

    if isinstance(data, Bool):
        if data.value is True:
            return f"{data.prefix}|true|"
        else:
            return f"{data.prefix}|false|"

    if isinstance(data, PrimitiveBase):
        if data.value:
            return f"{data.prefix}|{data.value}|"
        else:
            return f"{data.prefix}"

    if isinstance(data, ModifierBase):
        return f"{data.prefix}({to_str(data.value)})"

    if isinstance(data, VectorBase):
        res = f"{data.prefix}["
        for item in data._value:
            res += f"{to_str(item)},"
        res += "]"
        return res

    if isinstance(data, MapUnique):
        res = f"{data.prefix}{{"
        for k, v in data.items():
            res += f"{to_str(k)}:{to_str(v)},"
        res += "}"
        return res

    if isinstance(data, MapNotUnique):
        res = f"{data.prefix}{{"
        for item in data.items():
            res += f"{to_str(item.key)}:{to_str(item.value)},"
        res += "}"
        return res

    # Native types
    if data is None:
        return "null"
    if isinstance(data, bool):
        if data:
            return "b|true|"
        return "b|false|"
    if isinstance(data, int):
        return f"n|{data}|"
    if isinstance(data, float):
        return f"n|{data}|"
    if isinstance(data, str):
        data = data.replace("\\", "\\\\").replace("|", "\\|")
        return f"s|{data}|"
    if isinstance(data, list):
        res = "v["
        for item in data:
            res += f"{to_str(item)},"
        res += "]"
        return res
    if isinstance(data, dict):
        res = "m{"
        for k, v in data.items():
            res += f"{to_str(k)}:{to_str(v)},"
        res += "}"
        return res
