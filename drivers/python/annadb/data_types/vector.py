from typing import List, Optional

from annadb.constants import STORAGE_VECTOR
from annadb.data_types.tyson import TysonItem


class VectorBase(TysonItem):
    prefix_color = "blue"
    instance_type = "vector"

    def __init__(self, *args, value: Optional[List[TysonItem]] = None):
        if args:
            self._value = list(args)
        elif value:
            self._value = value
        else:
            self._value = []

    def __getitem__(self, item):
        return self._value[item]

    def __eq__(self, other):
        if isinstance(other, list):
            return self._value == other
        if isinstance(other, VectorBase):
            return self._value == other._value and self.prefix == other.prefix
        return False

    def pretty(self, console):
        console.add_text(f"{self.pretty_prefix}[")
        console.plus_tab()
        for item in self._value:
            console.new_line()
            item.pretty(console)
            console.add_text(",")
        console.minus_tab()
        console.new_line()
        console.add_text("]")

    def to_html(self, console):
        console.add_text(f"{self.html_prefix}[")
        console.plus_tab()
        for item in self._value:
            console.new_line()
            item.to_html(console)
            console.add_text(",")
        console.minus_tab()
        console.new_line()
        console.add_text("]")


class Vector(VectorBase):
    prefix = STORAGE_VECTOR
