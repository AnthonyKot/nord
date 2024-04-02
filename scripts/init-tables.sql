-- Table for storing tokens
CREATE TABLE tokens (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    symbol VARCHAR(10) NOT NULL,
    decimals INT NOT NULL,
    weight DECIMAL(5, 4) NOT NULL,
    max_rate DECIMAL(10, 8) NOT NULL,
    optimal_rate DECIMAL(10, 8) NOT NULL,
    optimal_utilization DECIMAL(5, 4) NOT NULL,
    origination_fee DECIMAL(5, 4) NOT NULL,
    liq_fee DECIMAL(5, 4) NOT NULL
);

-- Table for storing markets
CREATE TABLE markets (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(20) NOT NULL,
    base_token_id INT NOT NULL REFERENCES tokens(id),
    quote_token_id INT NOT NULL REFERENCES tokens(id),
    base_decimals INT NOT NULL,
    tick_decimals INT NOT NULL,
    maker_fee DECIMAL(5, 4) NOT NULL,
    taker_fee DECIMAL(5, 4) NOT NULL,
    liq_fee DECIMAL(5, 4) NOT NULL,
    imf DECIMAL(5, 4) NOT NULL
);

-- Table for storing user accounts
CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL
);

-- Table for storing user balances
CREATE TABLE balances (
    id SERIAL PRIMARY KEY,
    account_id INT NOT NULL REFERENCES accounts(id),
    token_id INT NOT NULL REFERENCES tokens(id),
    balance DECIMAL(30, 0) NOT NULL,
    borrow_multiplier DECIMAL(30, 20) NOT NULL DEFAULT 1.0,
    supply_multiplier DECIMAL(30, 20) NOT NULL DEFAULT 1.0
);

-- Table for storing orders
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    account_id INT NOT NULL REFERENCES accounts(id),
    market_id INT NOT NULL REFERENCES markets(id),
    side VARCHAR(4) NOT NULL, -- 'buy' or 'sell'
    type VARCHAR(10) NOT NULL, -- 'limit', 'market', etc.
    size DECIMAL(30, 0) NOT NULL,
    price DECIMAL(30, 10) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table for storing positions
CREATE TABLE positions (
    id SERIAL PRIMARY KEY,
    account_id INT NOT NULL REFERENCES accounts(id),
    market_id INT NOT NULL REFERENCES markets(id),
    size DECIMAL(30, 0) NOT NULL,
    entry_price DECIMAL(30, 10) NOT NULL,
    open_index DECIMAL(30, 20) NOT NULL,
    funding_index DECIMAL(30, 20) NOT NULL,
    unsettled_funding DECIMAL(30, 0) NOT NULL DEFAULT 0
);