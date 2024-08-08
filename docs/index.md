# peanuts

<p align="center">
<img src="./assets/anya.png" width="80%" style="border-radius: 2.5%;">
<br><br>
<img src="https://img.shields.io/github/license/externref/peanuts?style=flat-square">
<img src="https://img.shields.io/badge/code%20style-ruff-000000.svg?style=flat-square">
<img src="https://img.shields.io/badge/%20type_checker-pyright-%231674b1?style=flat-square">
<img src="https://img.shields.io/github/stars/externref/peanuts?style=flat-square">
<img src="https://img.shields.io/github/last-commit/externref/peanuts?style=flat-square">
<img src="https://img.shields.io/pypi/pyversions/peanuts?style=flat-square"><br>
A minimalist JSON based database for people who love peanuts :3 ( or not )
</p>

!!! warning
    This database system is just a raw idea and prone to unexpected behaviour. 

[Getting Started](/getting-started){ .md-button .md-button--primary }

## About

`peanuts` is a JSON based database system using file-name approach to store user data with strict schema checking and CLI flexibility.

It uses simple SQL like syntax for querying and inserting data along-side a expressive CLI with helpful error messages in case you make a mistake while using the commands.


!!! note "How is the data stored?"
    
    The project comes with a `data` folder that stores all the data. The `databases` sub-directory stores all the database related content.

    For each database, a new directory is created under `data/databases` with this structure 

    ```sh
    └── data/databases
        └── database_name # name of the database
            ├── data # this is where data is stored
            │   └── schema_name # schema related to the item
            │       └── entry_id.pnuts.json  # saved data with the unique ID
            └── schemas # all schema structure are saved under this
                └── schema_name.pnuts.json # json of a particular schema
    ```

