### Elements to be installed before starting the project

## [DISEL]
# 1. Use Cargo to Install the Dell CLI across the System
cargo install diesel_cli --no-default-features --features "mysql"

# 2. To create a migration, use the following command.
# where <migration_name> is the name of the migration you want to create. 
# This command creates the migration/<migration_name> directory and creates the up.sql and down.sql files within it. 
# The up.sql file contains SQL commands that are executed when you apply the migration, and the down.sql file contains SQL commands that are executed when you rollback the migration.
diesel migration generate <migration_name>
diesel migration generate gellery_ver1
diesel migration generate migrations


# 3. Use the following commands to apply created migrations to the database
diesel migration run



## [EXECUTE Async Program]
task::spawn_blocking(move|| {
    task....
}).await


## RWLock Process


## ok_or
# Option 형이 None 일때 에러를 반환한다.
# Err(anyhow::Error::msg("The insert operation failed."))




## REDIS
ZADD postLikes:postID123 1713862543 user1


ZADD likePhoto:100 1713862543 1
