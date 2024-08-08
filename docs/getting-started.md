The database is only available as a python package as of now, that can be downloaded from Github.

### Requirements

* [Python 3.8+](https://www.python.org/)

    Python comes pre-installed in most linux distros. You can install python for mac and windows from the official website. ([`Windows`](https://www.python.org/downloads/windows/), [`macOS`](https://www.python.org/downloads/macos/)). Make sure to install pip and add python to path.

* [Poetry](https://python-poetry.org/)

    Poetry is a python packaging and dependency management tool. `peanuts` heavily rely on it for the CLI and dependencies. You can install it from PYPI.
    ```sh
    python -m pip install poetry
    ```

* [Git (optional)](https://git-scm.com/)

    Git can be used to clone the repository to your device directly. Alternatively you can manually download code from github.

### Setting up the project

Clone the github repository using the git clone command and navigate into the project directory.

```sh
git clone https://github.com/externref/peanuts
cd peanuts
```

!!! info 
    Alternatively you can download the project from Github from [here](https://codeload.github.com/externref/peanuts/zip/refs/heads/main) and unzip it to get the same output as `git clone` if you don't plan to contribute.

To start with the database setup, install all the dependencies using `poetry install` command and enable the poetry venv using `poetry shell`

```sh
[python -m] poetry install
[python -m] poetry shell
# you might have to use [python -m] based on your python installation
```

### Schema

Schemas defines the strucutre of the data type to save. It acts as an equivalent of a SQL table in this case scenario. This is a sample schema configuration: 

```json
{
    "name": "string",
    "age": "integer"
}
```

The schema must be defined using a dictionary with name-type mapping, allowed types are:

* `string`: For strings

* `integer`: For integers

* `float`: For floats

* `bool`: For booleans

* `a_<other type>`: For array of above types ( `a_string`, `a_bool`, etc...)

### Example

!!! note "Here's a functional example of how easy it is to use the database."

    ```
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
