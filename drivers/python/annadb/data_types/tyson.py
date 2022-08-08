from abc import abstractmethod, ABC


class TysonItem(ABC):

    prefix = "undefined"
    prefix_color = "cyan"
    instance_type = ""

    @property
    def pretty_prefix(self):
        return f"[{self.prefix_color}]{self.prefix}[/{self.prefix_color}]"

    @property
    def html_prefix(self):
        return (
            f'<span class="prefix_{self.instance_type}">{self.prefix}</span>'
        )

    @abstractmethod
    def pretty(self, console):
        raise NotImplementedError
