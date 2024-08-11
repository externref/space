from __future__ import annotations

import json
import os
import typing

if typing.TYPE_CHECKING:
    from src.schema import Schema


class Cache:
    schema: Schema
    storage: dict[str, dict[str, typing.Any]]

    def __init__(self, inst: Schema) -> None:
        self.schema = inst
        self.storage = {}

    def load(self) -> None:
        for item in os.listdir(self.schema.storage_path):
            with open(self.schema.storage_path / item) as file:
                self.storage[item] = json.load(file)
