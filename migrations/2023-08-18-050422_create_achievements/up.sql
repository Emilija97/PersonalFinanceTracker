-- Your SQL goes here
CREATE TABLE achievements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    date_achieved TIMESTAMP NOT NULL,
    amount_saved DOUBLE PRECISION NOT NULL,
    goal_id UUID NOT NULL REFERENCES saving_goals(id) ON DELETE CASCADE
);
INSERT INTO public.users(
	id, username, email, created_at, updated_at)
	VALUES ('292a485f-a56a-4938-8f1a-bbbbbbbbbbb1'::UUID, 'emili', 'emili@test', current_timestamp, null);

-- SELECT * FROM public.users;