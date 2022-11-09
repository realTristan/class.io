#!bin/bash
echo "Clearing Database.."
sqlx database drop
y
echo "Database Dropped.."
sqlx database create
echo "Created New Database.."
sqlx migrate run
echo "Successfully Cleared Database!"