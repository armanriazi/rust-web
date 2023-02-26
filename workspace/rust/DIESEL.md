# Diesel
Diesel is one of the most popular and stable ORM tools in Rust.

In this lesson, we’ll learn to install and configure it for our projects.

We want to install the required database drivers. Instructions for installing these drivers for Linux/Mac can be found here.

Install database drivers on Windows
Instructions for installing the three most popular relational database management systems (RDBMS) on Windows are below.

## SQLite
Download precompile Windows binaries.
Extract both archives to folder C:\sqlite3. We can name this folder anything.
Open developer command prompt for Visual Studio (VS) by typing “developer command” in the Windows search bar.
Go to the folder with the extracted source code and binary files (via opened cmd or PowerShell).
Run lib /DEF:sqlite3.def /OUT:sqlite3.lib /MACHINE:x64.
Add C:\sqlite3 to the environment variable PATH.
Create an environment variable called SQLITE3_LIB_DIR also pointing to C:\sqlite3.
## MySQL
Download the MySQL connector.

Locate mysqlclient.lib. It might be installed in C:\Program Files\MySQL\MySQL Connector C 6.1\lib\vs14 if you use the MSI Installer.

Create an environment variable called MYSQLCLIENT_LIB_DIR that points to the previous folder.

## PostgreSQL
Download the installer and install PostgreSQL.
Add the bin directory to the PATH environment variable.
Create a variable named PQ_LIB_DIR pointing to the lib directory.
Prepare Diesel
Run the next command with the corresponding RDBMS. In the example, we are using SQLite.
```
cargo install diesel_cli --no-default-features --features sqlite
```
### Setup ORM
Finally, we can create our database with the below command.
```
diesel setup
```
We need to specify the database_url to create the testing database.
```
diesel setup --database-url='web_edu_test.sqlite
```

## Migration


### Migration Create Products

```
diesel migration generate create_products
```

The previous command will create two files named up.sql and down.sql inside the migrations folder.

We’ll enter the below code in the up.sql file.

```sql
CREATE TABLE products (
  id INTEGER PRIMARY KEY,
  name VARCHAR NOT NULL,
  cost DOUBLE NOT NULL,
  active BOOLEAN NOT NULL DEFAULT 0 --Sqlite does not have a Boolean value
)
```
Let’s work with the down.sql file now.
```
drop table products
```
Next:
```
diesel migration run
```
---

### Migration Product Variants

Next:
We need to create another table to save shoe variants.
```
diesel migration generate create_variants
```
Below is how the up.sql file will look.
```sql
CREATE TABLE variants (
   id INTEGER PRIMARY KEY NOT NULL, 
   name VARCHAR NOT NULL
);

CREATE TABLE products_variants (
   id INTEGER PRIMARY KEY NOT NULL, 
   variant_id INTEGER NOT NULL,
   product_id INTEGER NOT NULL,
   value VARCHAR,
   FOREIGN KEY(variant_id) REFERENCES variants(id),
   FOREIGN KEY(product_id) REFERENCES products(id)   
);
```
Below is how the down.sql file.
```
drop table variants;
drop table products_variants;
```

### Migration For Test DB
We did not include indexes in the above code widgets, but you should do so according to your frequent queries. Next, we run the migrations.
```
diesel migration run
diesel migration run --database-url='db/web_edu_test.sqlite'
```

### Migration For Edit Form

The delete method is easier to implement. However, we should make some modifications in our SQL first. We open the up.sql file and update it with the below commands.
The clause ON DELETE CASCADE will allow us to delete the relations with variants every time we delete a product.
```sql
CREATE TABLE products_variants (
   id INTEGER PRIMARY KEY NOT NULL, 
   variant_id INTEGER NOT NULL,
   product_id INTEGER NOT NULL,
   value VARCHAR,
   FOREIGN KEY(variant_id) REFERENCES variants(id) ON DELETE CASCADE,
   FOREIGN KEY(product_id) REFERENCES products(id) ON DELETE CASCADE
);
```
Notice(In code): 
We need to be aware that SQLite does not enforce foreign keys, so we need to tell the database with the following command: 
```
connection.execute("PRAGMA foreign_keys = ON").unwrap();
```
We can see a couple of interesting lines in this test. First, we have the annotation #[should_panic(expected = "NotFound")]We use this annotation to tell Rust that this test should effectively panic. This will happen when we try to show an inexistent product and unwrap an Err result.

```
#[should_panic(expected = "NotFound")]
```

### How to make a `SELECT .... WHERE EXISTS` query with diesel?

```sql
SELECT * FROM user AS u
WHERE EXISTS(
    SELECT 0 FROM address AS a
    WHERE a.user_id = u.id
        AND a.city = 'Berlin'
)
```
>
```rust,no_run
diesel::table! {
    addresses (id) {
        id -> Int4,
        user_id -> Int4,
        city -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(addresses -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    addresses,
    users,
);
```
> Response

```rust
let sub_query = addresses::table
    .select(0.into_sql::<diesel::sql_types::Integer>())
    .filter(addresses::user_id.eq(users::id))
    .filter(addresses::city.eq("Berlin"));
let result = users::table.filter(diesel::dsl::exists(sub_query)).load::<CompatibleType>(&mut conn)?;
```

### Sorting and Filtering

[sorting-and-filtering-with-diesel](https://cloudmaker.dev/sorting-and-filtering-with-diesel/)

# References

[no_arg_sql_function](https://diesel.rs/guides/migration_guide.html#2-0-0-no_arg_sql_function)

