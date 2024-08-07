from __future__ import annotations

import readline  # type: ignore # noqa: F401

import tabulate
from rich.console import Console

from src.main import Instance


class Session:
    dbname: str
    app: Instance
    console = Console()

    def __init__(self, dbname: str) -> None:
        self.dbname = dbname
        self.app = Instance(dbname)

    def run_command(self, action: str, restargs: list[str]) -> None:
        if action == "!add_schema":
            self.app.add_schema(restargs[0], " ".join(restargs[1:]))
        elif action == "!drop_schema":
            self.app.drop_schema(restargs[0])
        elif action == "insert":
            assert self.app.schemas.get(
                restargs[0]
            ), f"No schema named {restargs[0]} found"
            self.app.schemas[restargs[0]].write(restargs[1], " ".join(restargs[2:]))
        elif action == "select":
            assert self.app.schemas.get(
                restargs[0]
            ), f"No schema named {restargs[0]} found"
            data = self.app.schemas[restargs[0]].read(restargs[1])
            print(
                tabulate.tabulate(
                    ([k, v] for k, v in data.items()), tablefmt="rounded_grid"
                )
            )
        elif action in ["!q", "!quit", ":q", ":quit"]:
            raise KeyboardInterrupt

    def run_session(self) -> None:
        try:
            self.console.log(
                f"[cyan] Welcome to [bold]peanuts 🥜[/bold], Connected to database: [bold]{self.dbname}[/bold][/cyan]"
            )
            while True:
                cmd = self.console.input("[bold][green]#[/green][/bold] ")
                vals = cmd.split(" ")
                self.run_command(vals[0], vals[1:])

        except KeyboardInterrupt:
            self.console.log("[green]Thanks for using [bold]peanuts 🥜[/bold][/green]")
