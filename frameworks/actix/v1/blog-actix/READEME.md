Run migrations via Diesel CLI:
`diesel migration run'`

Migrations
--------------------------------------------------------------------------------
The Diesel CLI binary is named diesel, so we can setup our project for working
with Diesel by running the setup command:
` diesel setup`

Creating migrations directory at: ~/blog-actix/migrations

The --database-url argument must be passed, or the DATABASE_URL environment variable must be set.

By default this assumes the existence of an environment variable named `DATABASE_URL` or a `.env` file with this variable defined. 

It is possible to pass this manual to each CLI command, but using one of the aforementioned methods is much more convenient. 

Create `.env` file
`DATABASE_URL=file:blog.db`

This will create a `migrations directory` as well as a `diesel.toml` file.
If you are using Postgres this command will also create a migration that 
creates a SQL function for working with timestamps. This does not happen for other backends.

Diesel manages migrations using a directory called migrations with a subdirectory for each migration. 
The name of the subdirectories are a timestamp prefix followed by a name. 
Within each migration directory are two self-explanatory files: up.sql and down.sql. 
Diesel uses SQL for migrations rather than a custom DSL. 
Therefore changing databases requires rewriting most of your migrations.

### Running migrations
The primary use for the CLI is managing migrations which uses the migration command with further subcommands.

`diesel migration list`

`diesel migration run`

`diesel migration --help`

### Schema
To connect your database Diesel uses a schema file that is a code representation of your database. 
Running the migrations also modifies this code representation at `src/schema.rs`. 
The name of this file and whether this happens can be controlled via settings in `diesel.toml`, 
but the default is usually what you want.

### Create users migration
The first step is to add a migration that will create the database table users to hold our users:
`diesel migration generate create_users`

This syntax is specific to the Sqlite backend so this it should be clear why 
all migrations could need to be rewritten if you decide to use a different backend. 
For example, some databases allow you restrict the size of VARCHAR columns which 
might be a reasonable thing to do for a username, but Sqlite does not actually 
enforce any limit you happen to write.

The corresponding `down.sql` file should perform whatever transformations are necessary to undue what happens in `up.sql`.

You can do whatever you want in up and down, but for your own sanity, 
the schema should be the same before running the migration and after running up followed by down. 
That is down should revert the schema to the prior state.

### Make username unique
`diesel migration generate index_username`

Schema (96), (98)
--------------------------------------------------------------------------------

cr. Notes are from Schema (98)
Run migrations via Diesel CLI:
`diesel migration run`

Once this runs successfully two things will be true. 
- First, the database file at `blog.db` will be in the state after running all of our up migrations. 
You can verify this by opening the Sqlite shell:
`sqlite3 blog.db`

and dumping the schema:
`sqlite> .schema`

Note that the `__diesel_schema_migrations` table is automatically created by Diesel 
and it is how the CLI knows which migrations have or have not run.

- Second, file `src/schema.rs` is updated with Rust code which Diesel uses to 
understand the state of your database.


