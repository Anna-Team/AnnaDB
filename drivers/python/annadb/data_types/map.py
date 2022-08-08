from collections import OrderedDict
from typing import List, Optional

from annadb.constants import STORAGE_MAP
from annadb.data_types.pair import Pair
from annadb.data_types.tyson import TysonItem


class MapBase(TysonItem):
    prefix_color = "red"
    instance_type = "map"

    def __eq__(self, other):
        if isinstance(other, MapBase):
            return self.prefix == other.prefix and self._value == other._value
        return False

    def __len__(self):
        return len(self._value)


class MapUnique(MapBase):
    def __init__(self, data: Optional[dict] = None):
        if data is None:
            self._value = OrderedDict()
        else:
            self._value = data

    def __setitem__(self, key, value):
        self._value[key] = value

    def __getitem__(self, item):
        return self._value[item]

    def values(self):
        return list(self._value.values())

    def pretty(self, console):
        console.add_text(f"{self.pretty_prefix}{{")
        console.plus_tab()
        for k, v in self.items():
            console.new_line()
            k.pretty(console)
            console.add_text(":")
            v.pretty(console)
            console.add_text(",")
        console.minus_tab()
        console.new_line()
        console.add_text("}")

    def to_html(self, console):
        console.add_text(f"{self.html_prefix}{{")
        console.plus_tab()
        for k, v in self.items():
            console.new_line()
            k.to_html(console)
            console.add_text(":")
            v.to_html(console)
            console.add_text(",")
        console.minus_tab()
        console.new_line()
        console.add_text("}")

    def items(self):
        return self._value.items()


class MapNotUnique(MapBase):
    def __init__(self, data: Optional[List[Pair]] = None):
        if data is None:
            self._value: List[Pair] = []
        else:
            self._value = data

    def __setitem__(self, key, value):
        self._value.append(Pair(key, value))

    def __getitem__(self, item):
        return self._value[item]

    def pretty(self, console):
        console.add_text(f"{self.pretty_prefix}{{")
        console.plus_tab()
        for item in self._value:
            console.new_line()
            item.key.pretty(console)
            console.add_text(":")
            item.value.pretty(console)
            console.add_text(",")
        console.minus_tab()
        console.new_line()
        console.add_text("}")

    def items(self):
        return self._value


class Map(MapUnique):
    prefix = STORAGE_MAP

    def __eq__(self, other):
        if isinstance(other, dict):
            return self._value == other
        return super(Map, self).__eq__(other)
