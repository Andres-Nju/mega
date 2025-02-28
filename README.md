# Mega - Monorepo Engine for Enterprise and Individual

Mega is an engine for managing a monorepo. It functions similarly to Google's Piper and helps to streamline Git and trunk-based development for large-scale projects.

## Git Compatible

Git is a version control system that distributes file versions across local machines, allowing for quick access and collaboration. While mid-sized companies can store repositories as large as 20TB, managing such extensive codebases can pose challenges. Mega offers the ability to utilize Git with a monorepo. This allows for easy cloning or pulling of any monorepo folder into local filesystem as a Git repository, and seamless pushing of changes back. Despite Git's widespread use, it does not inherently support monorepo structures, but Mega fills this void.

## Trunk-based Development

When it comes to managing large codebases in a centralized manner, trunk-based development is the way to go. This workflow is particularly well-suited for monorepos. The idea behind trunk-based development is to work on a single codebase, making frequent commits and testing regularly. This approach helps identify issues early on, which ultimately leads to greater code stability. Additionally, trunk-based development enables consistency and integration, making it easier to manage monorepos and collaborate effectively on larger projects.

## Quick Started for developing and testing Mega on MacOS

1. Install Rust on your MacOS machine.

   ```bash
   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone mega repository and build it.

   ```bash
   $ git clone https://github.com/web3infra-foundation/mega.git
   $ cd mega
   $ cargo build
   ```

3. Install PostgreSQL and init database.

   1.  Install PostgreSQL 16 with `brew` command.

   ```bash
   $ brew install postgresql@16
   $ echo 'export PATH="/opt/homebrew/opt/postgresql@16/bin:$PATH"' >> ~/.zshrc
   $ brew services start postgresql@16
   $ initdb /Volumes/Data/postgres -E utf8 # /Volumes/Data is path store data
   ```

   2.  Create database, then find the dump file in the SQL directory of the Mega repository and import it into the database.

   ```bash
   $ psql postgres
   ```

   ```sql
   postgres=# \l
   postgres=# DROP DATABASE IF EXISTS mega;
   postgres=# CREATE DATABASE mega;
   postgres=# \q
   ```

   ```bash
   $ cd mega/sql/postgres
   $ psql mega < pg_2023092__init.sql
   ```
   
   3. Craeate user and grant privileges.

   ```sql
   postgres=# DROP USER IF EXISTS mega;
   postgres=# CREATE USER mega WITH ENCRYPTED PASSWORD 'rustgit';
   postgres=# GRANT ALL PRIVILEGES ON DATABASE mega TO mega;
   ```

   ```bash
   $ psql mega -c "GRANT ALL ON ALL TABLES IN SCHEMA public to mega;"
   $ psql mega -c "GRANT ALL ON ALL SEQUENCES IN SCHEMA public to mega;"
   $ psql mega -c "GRANT ALL ON ALL FUNCTIONS IN SCHEMA public to mega;"
   ```

4. Install redis.

   ```bash
   $ brew install redis
   $ brew services start redis
   ```

5. Config environment variables for local test. For local testing, Mega uses the .env file to configure the required parameters. However, before starting the project, you also need to configure the environment variables such as `DB_USERNAME`, `DB_SECRET`, and `DB_HOST`.

   ```ini
   MEGA_DB_POSTGRESQL_URL = "postgres://mega:rustgit@127.0.0.1/mega"
   MEGA_DB_MAX_CONNECTIONS = 32
   MEGA_DB_MIN_CONNECTIONS = 16

   MEGA_DB_SQLX_LOGGING = false # Whether to disabling SQLx Log
   # If the object file size exceeds a threshold value, it will be stored in the specified location instead of the database.
   MEGA_BIG_OBJ_THRESHOLD_SIZE = 1024 # Unit KB
   MEGA_BIG_OBJ_STORAGR_PATH = "/Volumes/Data/mega"

   MGEA_LFS_FILE_LOCAL_PATH = "/tmp/.mega/objects" # This configuration is used to set the local location of the lfs store

   GIT_INTERNAL_DECODE_CACHE_SIZE = 100 # Maximum number of git objects in LRU cache
   GIT_INTERNAL_DECODE_STORAGE_BATCH_SIZE = 1000 # The maximum number of git object in a "INSERT" SQL database operation
   GIT_INTERNAL_DECODE_STORAGE_TQUEUE_SIZE = 1 # The maximum number of parallel insertion threads in the database operation queue
   GIT_INTERNAL_DECODE_CACHE_TYEP = "redis" # {lru,redis}
   REDIS_CONFIG = "redis://127.0.0.1:6379"

   ## Bazel build config, you can use service like buildfarm to enable RBE(remote build execution)
   # you can refer to https://bazelbuild.github.io/bazel-buildfarm/docs/quick_start/ for more details about remote executor
   BAZEL_BUILD_ENABLE = true # leave true if you want to trigger bazel build in each push process
   BAZEL_BUILDP_PATH = "/tmp/.mega/bazel_build_projects" # Specify a temporary directory to build the project with bazel
   BAZEL_REMOTE_EXECUTOR = "grpc://localhost:8980" # If enable the remote executor, please fillin the remote executor address, or else leave empty if you want to build by localhost. 
   BAZEL_GIT_CLONE_URL = "http://localhost:8000" # Tell bazel to clone the project from the specified git url
   ```

6. Init the Mega

   ```bash
   $ cd mega
   $ cargo run init
   ```

7. Start the Mega server for testing.

   ```bash
   $ cargo run https
   ```

8. Test the push 

   ```bash
   $ cd mega
   $ git remote add local http://localhost:8000/projects/mega.git
   $ git push local main
   $ cd /tmp
   $ git clone http://localhost:8000/projects/mega.git
   ```

9. View from the browser

   ```bash
   $ cd mega/ui
   $ npm install --force
   $ npm run dev # Lanuch a chrome to open http://127.0.0.1:3000
   ```

## Contributing

The mega project relies on community contributions and aims to simplify getting started. To develop Mega, clone the repository, then install all dependencies and initialize the database schema, run the test suite and try it out locally. Pick an issue, make changes, and submit a pull request for community review.

More information on contributing to Mega is available in the [Contributing Guide](docs/contributing.md).

## License

Mega is licensed under this Licensed:

- MIT LICENSE ( [LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)