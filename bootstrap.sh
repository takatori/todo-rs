#! /usr/bin/env bash

# pwd -P  Display the physical current working directory (all symbolic links resolved)
SCRIPT_PATH=$(cd "$(dirname "$0")" ; pwd -P)
DATABASE_URL=${SCRIPT_PATH}/db/db.sql

pushd $SCRIPT_PATH > /dev/null
# install the diesel CLI tools if they're not installed
if ! command -v diesel > /dev/null 2>&1; then
    cargo install diesel_cli --features "sqlite" --no-default-features > /dev/null
fi

# create db/db.sql
diesel migration --database-url=$DATABASE_URL run > /dev/null
popd > /dev/null

echo "export DATABASE_URL"=$DATABASE_URL
