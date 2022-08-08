from annadb.data_types.tyson import TysonItem


class ModifierBase(TysonItem):
    prefix_color = "green"
    instance_type = "modifier"

    def __init__(self, value):
        self.value = value

    def pretty(self, console):
        console.add_text(f"{self.pretty_prefix}(")
        self.value.pretty(console)
        console.add_text(")")

    def to_html(self, console):
        console.add_text(f"{self.html_prefix}(")
        self.value.to_html(console)
        console.add_text(")")
