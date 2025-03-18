-- Add migration script here
CREATE TABLE scheduled_emails (
    id SERIAL PRIMARY KEY,
    to_address TEXT NOT NULL,
    subject TEXT NOT NULL,
    body TEXT NOT NULL,
    scheduled_time TIMESTAMP NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending'
);

-- Down Migration
DROP TABLE scheduled_emails;-- Add migration script here
