from __future__ import annotations

import json
import os
import pathlib

from src.schema import Schema


class Instance:
    database_name: str
    schemas: dict[str, Schema]

    def __init__(self, db_name: str) -> None:
        self.database_name = db_name
        if not (
            (dpath := pathlib.Path(f"data/databases/{db_name}")).exists()
            and dpath.is_dir()
        ):
            os.mkdir(f"data/databases/{db_name}")
            os.mkdir(f"data/databases/{self.database_name}/data/")
            os.mkdir(f"data/databases/{db_name}/schemas")
        self.schemas = {
            fname.split(".")[0]: Schema.from_str(
                db_name,
                fname.split(".")[0],
                open(f"data/databases/{db_name}/schemas/{fname}").read(),
            )
            for fname in os.listdir(f"data/databases/{db_name}/schemas")
        }

    def drop_schema(self, name: str) -> None:
        assert self.schemas.get(name), f"No schema named {name}"
        self.schemas.pop(name)
        os.remove(f"data/database/{self.database_name}/schemas/{name}.pnuts.json")
        os.rmdir(f"data/database/{self.database_name}/data/{name}")

    def add_schema(self, name: str, data: str) -> None:
        schema = Schema(self.database_name, name, json.loads(data))
        self.schemas[name] = schema.create()
