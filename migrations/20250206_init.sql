CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT,
    password TEXT,
    salt TEXT,
    confirmed INTEGER
);

CREATE TABLE IF NOT EXISTS wallets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    user_id INTEGER,

    name TEXT,
    initial_value INTEGER,

    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS materialized_wallets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    wallet_id INTEGER,

    FOREIGN KEY (wallet_id) REFERENCES wallets(id)
);

CREATE TABLE IF NOT EXISTS invoices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    wallet_id INTEGER,

    value INTEGER,
    name TEXT,
    description TEXT,

    FOREIGN KEY (wallet_id) REFERENCES wallets(id)
);

CREATE TABLE IF NOT EXISTS invoice_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    invoice_id INTEGER

    value INTEGER,
    name TEXT,
    description TEXT,

    FOREIGN KEY (invoice_id) REFERENCES invoices(id)
);


CREATE TABLE IF NOT EXISTS labels (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    user_id INTEGER

    icon TEXT,
    name TEXT,
    description TEXT,

    FOREIGN KEY (user_id) REFERENCES users(id)
);
