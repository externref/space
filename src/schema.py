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

import enum
import json
import os
import pathlib
import string
import typing

from rich.console import Console

from src.cache import Cache
from src.common import display_syntax

SchemaTypes = typing.Literal["string", "integer", "float", "bool", "a_string", "a_integer", "a_float", "a_bool"]


class SchemaTypeToPyT(enum.Enum):
    STRING = "string"
    INTEGER = "integer"
    FLOAT = "float"
    BOOL = "bool"
    A_STRING = "a_string"
    A_INTEGER = "a_integer"
    A_FLOAT = "a_float"
    A_BOOL = "a_bool"


class Schema:
    database: str
    name: str
    raw_configs: dict[str, SchemaTypes]
    configs: dict[str, SchemaTypeToPyT]
    cache: Cache
    console = Console()

    def __init__(self, dbname: str, schema_name: str, schema_data: dict[str, SchemaTypes]) -> None:
        self.database = dbname
        self.name = schema_name
        self.raw_configs = schema_data
        self.cache = Cache(self)
        self.configs = {name: SchemaTypeToPyT(t.lower()) for name, t in schema_data.items()}

    def __repr__(self) -> str:
        return json.dumps(self.raw_configs, indent=4)

    @classmethod
    def from_str(cls, db_name: str, schema_name: str, data: str) -> Schema:
        return cls(db_name, schema_name, json.loads(data))

    @property
    def storage_path(self) -> pathlib.Path:
        return pathlib.Path(f"data/databases/{self.database}/data/{self.name}")

    def create(self) -> Schema:
        with open(f"data/databases/{self.database}/schemas/{self.name}.pnuts.json", "w") as schemafile:
            schemafile.write(json.dumps(self.raw_configs, indent=4))

        os.mkdir(f"data/databases/{self.database}/data/{self.name}")
        self.console.print(f"[green]Schema [bold]{self.name}[/bold] created")
        return self

    def validate(self, data: dict[str, typing.Any]) -> None:
        assert set(self.raw_configs.keys()) == set(data.keys()), "Key mismatch"

        def verify_array(t: type, arr: list[typing.Any]) -> None:
            for item in arr:
                assert isinstance(item, t), f"Expected type {t} for {key}, got {type(item)}"

        for key, value in data.items():
            dtype = self.configs[key]
            if dtype == SchemaTypeToPyT.STRING:
                assert isinstance(value, str), f"Expected type {dtype} for {key}, got {type(value)}"
            elif dtype == SchemaTypeToPyT.INTEGER:
                assert isinstance(value, int), f"Expected type {dtype} for {key}, got {type(value)}"
            elif dtype == SchemaTypeToPyT.FLOAT:
                assert isinstance(value, float), f"Expected type {dtype} for {key}, got {type(value)}"
            elif dtype == SchemaTypeToPyT.BOOL:
                assert isinstance(value, bool), f"Expected type {dtype} for {key}, got {type(value)}"
            elif dtype == SchemaTypeToPyT.A_STRING:
                assert verify_array(str, value)
            elif dtype == SchemaTypeToPyT.A_INTEGER:
                assert verify_array(int, value)
            elif dtype == SchemaTypeToPyT.A_FLOAT:
                assert verify_array(float, value)
            elif dtype == SchemaTypeToPyT.A_BOOL:
                assert verify_array(bool, value)

    def write(self, id: str, payload: str) -> None:
        if set(id) & set(string.ascii_letters + "1234567890-_") != set(id):
            self.console.print(display_syntax("insert", "insert <schema_name> <id_a-z_1-9_-> [data_JSON]"))
            return
        try:
            self.validate(d := json.loads(payload))
        except AssertionError as e:
            self.console.print(f"[red]{e}")
            return
        with open(f"data/databases/{self.database}/data/{self.name}/{id}.pnuts.json", "w") as datafile:
            json.dump(d, datafile, indent=4)
        self.console.print(f"[green]Written data for [bold]{id}[/bold] in [bold]{self.name}[/bold]")

    def read(self, id: str) -> dict[str, typing.Any]:
        with open(f"data/databases/{self.database}/data/{self.name}/{id}.pnuts.json") as file:
            return json.load(file)

    def display_dict(self) -> dict[str, list[typing.Any]]:
        data: dict[str, list[typing.Any]] = {"id": []}
        data.update({f"\033[1m{col}\033[0m": [] for col in self.configs.keys()})
        for file in self.storage_path.glob("*.pnuts.json"):
            data["id"].append(file.name.split(".")[0])
            for key, value in json.loads(file.read_text()).items():
                data[f"\033[1m{key}\033[0m"].append(value)
        return data
