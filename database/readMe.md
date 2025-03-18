## How to create the migration file 
sqlx migrate add create_scheduled_emails_table

## How to run the migrations file 

``sqlx migrate run``


if this does not work then it means that you need to check postgres is working 

``sudo service postgresql status``


if you have 
`12/main (port 5432): down`


start up postgres 

``sudo service postgresql start``

you will also need to create a database 
sudo -u postgres psql
then 
CREATE DATABASE my_future_mail;


or sudo -u postgres psql -c "CREATE DATABASE my_future_mail"

delete the postgres server without having to go into the postgres terminal

sudo -u postgres psql -c "DROP DATABASE ${DB_NAME};"
