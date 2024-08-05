from __future__ import annotations

import enum
import json
import os
import typing

SchemaTypes = typing.Literal[
    "string", "integer", "float", "bool", "a_string", "a_integer", "a_float", "a_bool"
]


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

    def __init__(
        self, dbname: str, schema_name: str, schema_data: dict[str, SchemaTypes]
    ) -> None:
        self.database = dbname
        self.name = schema_name
        self.raw_configs = schema_data
        self.configs = {
            name: SchemaTypeToPyT(t.lower()) for name, t in schema_data.items()
        }

    def __repr__(self) -> str:
        return json.dumps(self.raw_configs, indent=4)

    @classmethod
    def from_str(cls, db_name: str, schema_name: str, data: str) -> Schema:
        return cls(db_name, schema_name, json.loads(data))

    def create(self) -> Schema:
        with open(
            f"data/databases/{self.database}/schemas/{self.name}.pnuts.json", "w"
        ) as schemafile:
            schemafile.write(json.dumps(self.raw_configs, indent=4))

        os.mkdir(f"data/databases/{self.database}/data/{self.name}")
        return self

    def validate(self, data: dict[str, typing.Any]) -> None:
        assert set(self.raw_configs.keys()) == set(data.keys()), "Key mismatch"

        def verify_array(t: type, arr: list[typing.Any]) -> None:
            for item in arr:
                assert isinstance(
                    item, t
                ), f"Expected type {t} for {key}, got {type(item)}"

        for key, value in data.items():
            dtype = self.configs[key]
            if dtype == SchemaTypeToPyT.STRING:
                assert isinstance(
                    value, str
                ), f"Expected type {dtype} for {key}, got {type(value)}"
            elif dtype == SchemaTypeToPyT.INTEGER:
                assert isinstance(
                    value, int
                ), f"Expected type {dtype} for {key}, got {type(value)}"
            elif dtype == SchemaTypeToPyT.FLOAT:
                assert isinstance(
                    value, float
                ), f"Expected type {dtype} for {key}, got {type(value)}"
            elif dtype == SchemaTypeToPyT.BOOL:
                assert isinstance(
                    value, bool
                ), f"Expected type {dtype} for {key}, got {type(value)}"
            elif dtype == SchemaTypeToPyT.A_STRING:
                assert verify_array(str, value)
            elif dtype == SchemaTypeToPyT.A_INTEGER:
                assert verify_array(int, value)
            elif dtype == SchemaTypeToPyT.A_FLOAT:
                assert verify_array(float, value)
            elif dtype == SchemaTypeToPyT.A_BOOL:
                assert verify_array(bool, value)

    def write(self, id: str, payload: str) -> None:
        self.validate(d := json.loads(payload))
        with open(
            f"data/databases/{self.database}/data/{self.name}/{id}.pnuts.json", "w"
        ) as datafile:
            datafile.write(json.dumps(d, indent=4))
