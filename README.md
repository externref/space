# peanuts 🥜

<p align="center">
<img src="https://img.shields.io/github/license/externref/peanuts?style=flat-square">
<img src="https://img.shields.io/badge/code%20style-ruff-000000.svg?style=flat-square">
<img src="https://img.shields.io/badge/%20type_checker-pyright-%231674b1?style=flat-square">
<img src="https://img.shields.io/github/stars/externref/peanuts?style=flat-square">
<img src="https://img.shields.io/github/last-commit/externref/peanuts?style=flat-square">
<img src="https://img.shields.io/pypi/pyversions/peanuts?style=flat-square"><br>
A minimalist JSON based database for people who love peanuts :3 ( or not )
</p>



> [!WARNING]  
> This database system is just a raw idea and prone to unexpected behaviour. 

## examples 

### connecting

* To connect to the database this command should be used 
```sh
$ peanuts <db_name>
```

### setting up schemas

* To create a schema the `!add_schema` command can be used 
```sh
!add_schema <schema_name> <schema_json_mapping>
```
The schema must be defined using a dictionary with name-type mapping, allowed types are:
* `string`: For strings
* `integer`: For integers
* `float`: For floats
* `bool`: For booleans
* `a_<other type>`: For array of above types ( `a_string`, `a_bool`, etc...)

### inserting data

* To write new data for a schema the `insert` command can be used
```sh
insert <schema_name> <item_id> <item_data>
# or, to provide data one by one without using JSON, enter only item_id
insert <schema_name> <item_id> 
# this will ask for the schema data values one by one while verifying types.
```

### live example

```sh
(peanuts-py3.10) sarthak@sarthak:~/peanuts$ peanuts students
[01:55:32]  Welcome to peanuts 🥜, Connected to database: students
# !add_schema studentinfo { "name": "string", "age": "integer" }
# insert studentinfo 1
name (type: SchemaTypeToPyT.STRING): sarthak
age (type: SchemaTypeToPyT.INTEGER): 19
# select studentinfo 1
╭──────┬─────────╮
│ name │ sarthak │
├──────┼─────────┤
│ age  │ 19      │
╰──────┴─────────╯
```

## database structure

JSON data is saved under folders with IDs as filenames linked to schemas.

```
└── data/databases
    └── database_name
        ├── data
        │   └── schema_name
        │       └── entry_id.pnuts.json
        └── schemas
            └── schema_name.pnuts.json
```
![](./assets/structure.png)


