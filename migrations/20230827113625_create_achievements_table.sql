-- Add migration script here
CREATE TABLE achievements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    date_achieved TIMESTAMP NOT NULL,
    amount_saved DOUBLE PRECISION NOT NULL,
    goal_id UUID NOT NULL REFERENCES saving_goals(id) ON DELETE CASCADE
);