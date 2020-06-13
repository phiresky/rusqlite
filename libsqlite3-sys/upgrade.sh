#!/bin/bash

SCRIPT_DIR=$(cd "$(dirname "$_")" && pwd)
echo "$SCRIPT_DIR"
cd "$SCRIPT_DIR" || { echo "fatal error"; exit 1; }
export SQLITE3_LIB_DIR=$SCRIPT_DIR/sqlite3
export SQLITE3_INCLUDE_DIR=$SQLITE3_LIB_DIR

# Download and extract amalgamation
SQLITE=sqlite-amalgamation-3320200
curl -sSf -O https://sqlite.org/2020/$SQLITE.zip
unzip -p "$SQLITE.zip" "$SQLITE/sqlite3.c" > "$SQLITE3_LIB_DIR/sqlite3.c"
unzip -p "$SQLITE.zip" "$SQLITE/sqlite3.h" > "$SQLITE3_LIB_DIR/sqlite3.h"
unzip -p "$SQLITE.zip" "$SQLITE/sqlite3ext.h" > "$SQLITE3_LIB_DIR/sqlite3ext.h"
rm -f "$SQLITE.zip"

# Regenerate bindgen file
cargo update

function generate_bindgen_binding() {
  features=$1
  target_file=$2

  rm -f "$target_file"
  # Just to make sure there is only one bindgen.rs file in target dir
  find "$SCRIPT_DIR/../target" -type f -name bindgen.rs -exec rm {} \;
  env LIBSQLITE3_SYS_BUNDLING=1 cargo build --features "$features" --no-default-features
  find "$SCRIPT_DIR/../target" -type f -name bindgen.rs -exec cp {} "$target_file" \;
  # rerun rustfmt after (possibly) adding wrappers
  rustfmt $target_file
}

generate_bindgen_binding "buildtime_bindgen" "$SQLITE3_LIB_DIR/bindgen_bundled_version.rs"
generate_bindgen_binding "buildtime_bindgen,loadable_extension" "$SQLITE3_LIB_DIR/bindgen_bundled_version-ext.rs"

# Sanity check
cd "$SCRIPT_DIR/.." || { echo "fatal error"; exit 1; }
cargo update
cargo test --features "backup blob chrono functions limits load_extension serde_json trace vtab bundled"
# TODO check loadable_extension / loadable_extension_embedded
echo 'You should increment the version in libsqlite3-sys/Cargo.toml'
