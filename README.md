This is a Rust GraphQL server that connects with a firebase realtime database
It can get all users from the database, a chosen user from the database, and add a user to the database. 

Setup:

-      git clone https://github.com/ronantakizawa/rustgraphqlserver.git  
-      cargo install

Add .env file in root of directory for your firebase realtime database URL. 

Make sure your realtime database is in Test mode. 

Running app:

-      cargo run