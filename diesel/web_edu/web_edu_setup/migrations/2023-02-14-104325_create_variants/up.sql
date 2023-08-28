CREATE TABLE variants (
   id INTEGER PRIMARY KEY NOT NULL, 
   name VARCHAR NOT NULL
);

CREATE TABLE products_variants (
   id INTEGER PRIMARY KEY NOT NULL, 
   variant_id INTEGER NOT NULL,
   product_id INTEGER NOT NULL,
   value VARCHAR,
   FOREIGN KEY(variant_id) REFERENCES variants(id) ON DELETE CASCADE,
   FOREIGN KEY(product_id) REFERENCES products(id) ON DELETE CASCADE
);