# MIT License

# Copyright (c) 2024 sarthak

# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:

# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.

# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

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
        if not ((dpath := pathlib.Path(f"data/databases/{db_name}")).exists() and dpath.is_dir()):
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
        if not self.schemas.get(name):
            Schema.console.print(f"[red]No schema named {name}")
            return
        self.schemas.pop(name)
        os.remove(f"data/databases/{self.database_name}/schemas/{name}.pnuts.json")
        os.rmdir(f"data/databases/{self.database_name}/data/{name}")
        Schema.console.print(f"Schema {name} dropped.")

    def add_schema(self, name: str, data: str) -> None:
        schema = Schema(self.database_name, name, json.loads(data))
        self.schemas[name] = schema.create()
