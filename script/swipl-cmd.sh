# 1. Set $SCRIPTDIR.
# 2. `source` this file.
# 3. Use $CMD to run `swipl` with its arguments

# Top-level directory
TOPDIR="$SCRIPTDIR/.."

# Set up the environment for `swipl` to use the built library
source "$TOPDIR/buildenv.sh"

# Run `swipl`, add the shared library to the search path, and `consult/1` the
# Prolog.
CMD=(swipl
  -g "asserta(file_search_path(foreign,'$TOPDIR/$PACKSODIR'))"
  -g "['$TOPDIR/prolog/terminus_store.pl']"
  -g version
  "$@")

# Use the above array by calling "${CMD[@]}".
