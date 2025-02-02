CREATE TABLE flavors (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE flavor_search_terms (
    flavor_id INTEGER NOT NULL,
    search_term TEXT NOT NULL,
    FOREIGN KEY (flavor_id) REFERENCES flavors(id)
);

CREATE TABLE stores (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE store_flavors (
    store_id INTEGER NOT NULL,
    flavor_id INTEGER NOT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (flavor_id) REFERENCES flavors(id)
);