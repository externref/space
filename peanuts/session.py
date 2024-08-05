from __future__ import annotations

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

    def run_session(self) -> None:
        try:
            self.console.log(
                f"[cyan] Welcome to [bold]peanuts 🥜[/bold], Connected to database: [bold]{self.dbname}[/bold][/cyan]"
            )
            while True:
                cmd = self.console.input("[bold][green]#[/green][/bold] ")
                vals = cmd.split(" ")
                if vals[0] == "!add_schema":
                    self.app.add_schema(vals[1], " ".join(vals[2:]))
                elif vals[0] == "!drop_schema":
                    self.app.drop_schema(vals[1])
                elif vals[0] == "insert":
                    assert self.app.schemas.get(
                        vals[1]
                    ), f"No schema named {vals[1]} found"
                    self.app.schemas[vals[1]].write(vals[2], " ".join(vals[3:]))
                elif vals[0] == "select":
                    assert self.app.schemas.get(
                        vals[1]
                    ), f"No schema named {vals[1]} found"
                    data = self.app.schemas[vals[1]].read(vals[2])
                    print(
                        tabulate.tabulate(
                            ([k, v] for k, v in data.items()), tablefmt="rounded_grid"
                        )
                    )
                elif vals[0] in ["!q", "!quit", ":q", ":quit"]:
                    raise KeyboardInterrupt
        except KeyboardInterrupt:
            self.console.log("[green]Thanks for using [bold]peanuts 🥜[/bold][/green]")
