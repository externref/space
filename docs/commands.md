---
description: Guide through all the commands available in the database system
---

# Commands

Here's a list of all the available commands in the database

## Schema and core operations.

### `!help`

Displays all commands info.

### `!quit`

Quits the CLI. Aliases: `!q`, `:q`, `:quit`

### `!create_schema`

Creates a new [schema](getting-started.md#schema)

**Syntax**
```json
!create_schema <schema_name> <schema_JSON>;

// example:
!create_schema studentinfo {
    "name": "string",
    "age": "number"
};
```

<p align="center">
    <img src="/assets/examples/create_schema.gif" width=80%>
</p>

### `!drop_schema`

Drops an existing schema.

**Syntax**
```json
!drop_schema <schema_name>;

// example: 
!drop_schema studentinfo;
```

### `!display_schema`

Displays data under an schema.

**Syntax**
```json
!display_schema <schema_name>;

// example: 
!display_schema studeinfo;
```

<p align="center">
    <img src="/assets/examples/display_schema.gif" width=80%>
</p>


### `!load_cache`

Loads all the existing data into cache. This is optional and suggested only when large operations have to be performed.

## Data related commands

### `insert`

Adds data to an schema with respect to provided ID 

**Syntax**
```json
insert <schema_name> <id> [data_JSON];

// example: 
insert studentinfo 1; // app will ask for further data
insert studentinfo 1 {
    "name": "sarthak",
    "age": 19
};
```

<p align="center">
    <img src="/assets/examples/insert.gif" width=80%>
</p>


### `select`

Fetch data from the cache or the storage

**Syntax**
```json
select <schema_name> <id>;

// example: 
select studentinfo 1;
```

<p align="center">
    <img src="/assets/examples/select.gif" width=80%>
</p>
