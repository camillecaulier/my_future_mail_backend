#!/bin/bash
set -e  # Exit immediately if a command exits with a non-zero status

# Load environment variables from .env if the file exists
if [ -f .env ]; then
  echo "Loading environment variables from .env..."
  export $(grep -v '^#' .env | xargs)
fi

# Define your database name
DB_NAME="my_future_mail"

echo "Creating database '$DB_NAME'..."
# Use the postgres superuser to create the database.
# If the database already exists, this command may fail.
sudo -u postgres createdb "$DB_NAME" || echo "Database '$DB_NAME' may already exist."

echo "Running migrations..."
# This assumes that sqlx is installed and your migrations directory is set up.
sqlx migrate run

echo "Database setup complete!"