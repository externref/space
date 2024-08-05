# peanuts

<p align="center">
<img src="https://raw.githubusercontent.com/externref/peanuts/main/assets/peanut.png" height=150 width=150><br><br>
<br><br>
db for people gone nuts >.<
</p>

> [!WARNING]  
> This database system is just a raw idea and prone to unexpected behaviour. 

## database structure

![](./assets/structure.png)

JSON data is saved under folders with IDs as filenames linked to schemas.

```
data/databases
    - test_database
        - schemas
            - test_schema.pnuts.json
        - data
            - test_schema
                - input_1.pnuts.json
```