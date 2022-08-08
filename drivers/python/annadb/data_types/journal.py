from tyson import deserialize

from annadb.console import SimpleConsole, HTMLConsole
from annadb.data_types.pair import Pair
from annadb.data_types.types_map import build_primitive, factory


class Journal:
    def __init__(self, data):
        self._value = data

    def __getitem__(self, item):
        return self._value[item]

    @classmethod
    def deserialize(cls, query: str):
        res = deserialize(query)
        return cls(
            [Pair(build_primitive(o[0]), factory(o[1])) for o in res.data]
        )

    def pretty(self, topic, console):
        console = SimpleConsole(topic, console)
        for pair in self._value:
            console.new_line()
            pair.key.pretty(console)
            console.add_text(":")
            pair.value.pretty(console)
            console.add_text(";")
        console.print()

    def to_html(self):
        console = HTMLConsole()
        for pair in self._value:
            pair.key.to_html(console)
            console.add_text(":")
            pair.value.to_html(console)
            console.add_text(";")
            console.new_line()
        return console.out()
