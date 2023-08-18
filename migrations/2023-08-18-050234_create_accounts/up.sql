-- Your SQL goes here
CREATE TYPE account_type AS ENUM (
    'bank',
    'cash',
    'card'
);

CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    account_type account_type NOT NULL,
    balance DOUBLE PRECISION NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE
);
