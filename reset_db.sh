#!/bin/bash

# Database credentials
DB_NAME="taskmanager"
DB_USER="tristanus"
DB_PASSWORD=""
DB_HOST="localhost"

echo "🔄 Resetting database '$DB_NAME'..."

# Drop the database
PGPASSWORD=$DB_PASSWORD psql -U $DB_USER -h $DB_HOST -c "DROP DATABASE IF EXISTS $DB_NAME;"

# Recreate the database
PGPASSWORD=$DB_PASSWORD psql -U $DB_USER -h $DB_HOST -c "CREATE DATABASE $DB_NAME;"

# Run migrations (recreate tables)
PGPASSWORD=$DB_PASSWORD psql -U $DB_USER -h $DB_HOST -d $DB_NAME -a -f migrations.sql

echo "✅ Database '$DB_NAME' has been reset successfully!"