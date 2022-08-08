class SimpleConsole:
    def __init__(self, title, console):
        self.title = title
        self.s = ""
        self.tabs = 0
        self.console = console

    def plus_tab(self):
        self.tabs += 1

    def minus_tab(self):
        self.tabs -= 1

    def add_text(self, text):
        self.s += text

    def new_line(self):
        self.s += "\n" + self.tabs * "\t"

    def print(self):
        # with self.console.pager():
        self.console.rule(self.title, style="cyan")
        self.console.print(self.s)
        self.console.print("")
        self.console.rule(style="blue")


class HTMLConsole:
    def __init__(self):
        self.s = ""
        self.tabs = 0

    def plus_tab(self):
        self.tabs += 1

    def minus_tab(self):
        self.tabs -= 1

    def add_text(self, text):
        self.s += text

    def new_line(self):
        self.s += "\n" + self.tabs * "\t"

    def out(self):
        self.s = "<pre><code>" + self.s + "</code></pre>"
        return self.s
