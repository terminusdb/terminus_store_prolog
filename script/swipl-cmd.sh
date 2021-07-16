# 1. Set $SCRIPTDIR.
# 2. `source` this file.
# 3. Use $CMD to run `swipl` with its arguments

# Pack shared object directory used by `swipl`.
PACKSODIR="lib/$($SCRIPTDIR/swiarch.pl)"

# Top-level directory
TOPDIR="$SCRIPTDIR/.."

# Run `swipl`, add the shared library to the search path, and `consult/1` the
# Prolog.
CMD=(swipl
  -g "asserta(file_search_path(foreign,'$TOPDIR/$PACKSODIR'))"
  -g "['$TOPDIR/prolog/terminus_store.pl']"
  -g version
  "$@")

# Use the above array by calling "${CMD[@]}".
