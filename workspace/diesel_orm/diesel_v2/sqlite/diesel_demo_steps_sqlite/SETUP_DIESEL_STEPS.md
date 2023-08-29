# Setup


## Notice

- [x] Replace char " with ` for using linux
- [x] AUTOINCREMENT up.sql  will remove if we have gottten error

---

1. First install diesel_cli_ext:

```
cargo install diesel_cli_ext
```

2. You would have to generate your schema file the diesel way if you haven"t yet:

```
diesel print-schema > src/schema.rs  --database-url="../db/diesel2_sample_steps.sqlite"
```

3. generate the models file(optional in this step):

```
diesel_ext --model > src/models.rs
```

4. setup

```
diesel setup --database-url="../db/diesel2_sample_steps.sqlite"
```


5. create migrations folder

```
mkdir ./migrations
```

6. generate and fill out up, down.sql

```
diesel migration generate posts --migration-dir="migrations"
diesel migration generate users --migration-dir="migrations"
```

7. migration

```
diesel migration run --database-url="../db/diesel2_sample_steps.sqlite" 
```

8. return to step 2 for cheking it is nessecarry to check schema file, is it generated by step 7 or not?

9. check created tables in db

```
sqlite3 ../db/diesel2_sample_steps.sqlite
```


---
diesel migration revert --database-url="../db/diesel2_sample_steps.sqlite" 
diesel migration redo --database-url="../db/diesel2_sample_steps.sqlite" 
---