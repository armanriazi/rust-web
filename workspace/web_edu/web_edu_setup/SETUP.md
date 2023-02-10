# Setup

---

1. First install diesel_cli_ext:

```
cargo install diesel_cli_ext`
```

2. You would have to generate your schema file the diesel way if you haven't yet:

```
diesel print-schema > workspace/web_edu/web_edu_lib/src/schema.rs
```

1. generate the models file(optional in this step):

```
diesel_ext --model > src/models.rs
```

4. setup

```
diesel setup --database-url='workspace/web_edu/web_edu_setup/web_edu.sqlite'
diesel setup --database-url='workspace/web_edu/web_edu_setup/web_edu_test.sqlite'
```

5. create migrations folder

```
mkdir ./workspace/web_edu/web_edu_setup/migrations
```

6. generate and fill out up, down.sql

```
diesel migration generate create_proucts   --migration-dir='workspace/web_edu/web_edu_setup/migrations'
```

7. migration

```
diesel migration run --database-url='workspace/web_edu/web_edu_setup/web_edu.sqlite'   
diesel migration run --database-url='workspace/web_edu/web_edu_setup/web_edu_test.sqlite'
```

8. check created tables in db

```
sqlite3 web_edu_test.sqlite
.tables
```

---
diesel migration revert
diesel migration redo
---
[print_schema]
import_types = [ "crate::schema::*" ]
import_types = ["diesel::sql_types::*", "diesel_full_text_search::types::*"]
filter = { only_tables = ["products"] }
----
