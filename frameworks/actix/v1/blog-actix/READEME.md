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

Working with Diesel requires some macros to be in scope which is why we have the macro_use attribute on the extern crate diesel item.

Modules and code organization
--------------------------------------------------------------------------------

"...cost of spreading code across multiple files was higher than the benefit."
cr. What is the tipping point? e.g. for Angular lazy loading

Rust has a module system to organize code within a crate for readability and ease 
of reuse as well as to control privacy of items. 
You can declare modules within a file with the `mod` keyword followed by a name, 
followed by curly braces to contain the code for the module:
```rust
mod a_module {
  pub fn some_function() {} 
  fn hidden_function() {}
  
  mod inner_module {
    pub fn inner_function() {}
    fn inner_hidden_function() {}
  } 
}

fn do_things() { 
  a_module::some_function(); 
  a_module::inner_module::inner_function();
}
```

We can also spread modules over files and directories which helps with file size 
and makes working with large projects much easier. 
Instead of declaring a module inline, you can simply refer to it via:
`mod a_module;`

The language then expects to find a file `a_module.rs` or a file `mod.rs` inside 
a directory with the name of the module, i.e. `a_module/mod.rs`.
Recommended style:
`
src/
    ├── a_module
    │   └── inner_module.rs
    ├── a_module.rs
    └── lib.rs
`

Errors
--------------------------------------------------------------------------------
The first module we are going to deal with is the errors module. 
We define our own error type which unifies our notion of error states across different 
parts of the application. This encapsulates the different types of errors that 
can happen so we can explicitly handle those scenarios and can avoid generic 500 errors as much as possible. 
We also use this type to translate errors from other libraries to our own domain specific error type.

Straightforward route of creating an enum for our error type. 
In this context encountering an error means we enter one of a small number of 
states where we want to return an error code and a message instead of continuing to process the request. 
This is a natural use case for an enumerated type. We define the type as:
`src/errors.rs`
```rust
#[derive(Debug)]
pub enum AppError {
  RecordAlreadyExists,
  RecordNotFound, 
  DatabaseError(diesel::result::Error), 
  // OperationCanceled is related to a actix_web error having to do with an async operation which we will explain later.
  OperationCanceled,
}
```

These are the only error states that we going to explicitly handle. 
As you build up your application and rely on more libraries that could fail you 
can add variants to this type to capture those errors if you want to deal with them this way.


pub(self) is equivalent to nothing at all, i.e. pub(self) mod foo is the same as mod foo which is actually private. Why? The answer lies in macros that can generate code with visibility specifiers. If a macro outputs code like pub($arg) where $arg
Even More Web 121
is an input argument, you might want to specify that the item should be private, so passing self as the argument achieves that goal.

CRUD Examples
--------------------------------------------------------------------------------
Create user
`curl -s -H 'Content-Type: application/json' -X POST http://localhost:8998/users -d '{"username":"Frank"}'`

{
  "id": 1,
  "username": "Frank"
}

`curl -s -H 'Content-Type: application/json' -X POST http://localhost:8998/users -d '{"username":"Bob"}'`

{
  "id": 2,
  "username": "Bob"
}

`curl -s -H 'Content-Type: application/json' -X POST http://localhost:8998/users -d '{"username":"Bob"}'`

{
  "err": "This record violates a unique constraint"
}

Lookup User by name
`curl -s -H 'Content-Type: application/json' http://localhost:8998/users/find/Frank`
{
  "id": 1,
  "username": "Frank"
}

Lookup User by primary key
`curl -s -H 'Content-Type: application/json' http://localhost:8998/users/1`
{
  "id": 1,
  "username": "Frank"
}

`curl -s -H 'Content-Type: application/json' http://localhost:8998/users/find/Steve`
{
  "err": "This record does not exist"
}

Extending data model
--------------------------------------------------------------------------------

`diesel migration generate create_posts`

migrations/up.sql
`
CREATE TABLE posts (
  id INTEGER PRIMARY KEY NOT NULL,
  user_id INTEGER NOT NULL REFERENCES users (id),
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 0
)
`

migrations/down.sql
`DROP TABLE posts`

Run migration. Diesel will update src/schema.rs
`diesel migration run`

Extending further
--------------------------------------------------------------------------------

`diesel migration generate create_comments`

`
CREATE TABLE comments (
  id INTEGER PRIMARY KEY NOT NULL,
  user_id INTEGER NOT NULL REFERENCES users (id),
  post_id INTEGER NOT NULL REFERENCES posts (id),
  body TEXT NOT NULL
)
`

Run migration. Diesel will update src/schema.rs
`diesel migration run`

Create a post
`curl -s -H 'Content-Type: application/json' -X POST http://localhost:8998/users/1/posts -d '{"title":"Frank says hello","body":"Hello friends"\ }'`
{
  "id": 1,
  "user_id": 1,
  "title": "Frank says hello",
  "body": "Hello friends",
  "published": false
}
Create a post
`curl -s -H 'Content-Type: application/json' -X POST http://localhost:8998/users/2/posts -d '{"title":"Bob is here too","body":"Hello friends, \ also"}'`
{
  "id": 2,
  "user_id": 2,
  "title": "Bob is here too",
  "body": "Hello friends, also",
  "published": false
}

Publish a post
`curl -s -H 'Content-Type: application/json' -X POST http://localhost:8998/posts/1/publish`
{
  "id": 1,
  "user_id": 1,
  "title": "Frank says hello",
  "body": "Hello friends",
  "published": true
}
Comment on a post
`curl -s -H 'Content-Type: application/json' -X POST http://localhost:8998/posts/1/comments -d '{"user_id":2,"body":"Hi Frank, this is your fri\ end Bob"}'`
{
  "id": 1,
  "user_id": 2,
  "post_id": 1,
  "body": "Hi Frank, this is your friend Bob"
}
List all posts
`curl -s -H 'Content-Type: application/json' http://localhost:8998/post`

See posts
`curl -s -H 'Content-Type: application/json' http://localhost:8998/users/1/posts`

Publish other post
`curl -s -H 'Content-Type: application/json' -X POST http://localhost:8998/posts/2/publish`

List all posts again
`curl -s -H 'Content-Type: application/json' http://localhost:8998/posts`

See users comments
`curl -s -H 'Content-Type: application/json' http://localhost:8998/users/2/comments`

See post comments
`curl -s -H 'Content-Type: application/json' http://localhost:8998/posts/1/comments`
