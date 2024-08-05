from __future__ import annotations

from src.main import Instance


class Session:
    dbname: str
    app: Instance

    def __init__(self, dbname: str) -> None:
        self.dbname = dbname
        self.app = Instance(dbname)

    def run_session(self) -> None:
        while True:
            cmd = input("> ")
            if cmd.startswith("add_schema "):
                vals = cmd.split()

                self.app.add_schema(vals[1], "".join(vals[2:]))
