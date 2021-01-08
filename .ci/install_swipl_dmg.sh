#!/bin/bash
HASH='235a08a3fcc4eecfa023a8e4f89a78ce54f7723120883f9944efaaf89930f444'
SWIPL_DMG_NAME='swipl-8.2.3-1.x86_64'
SWIPL_DMG="$SWIPL_DMG_NAME.dmg"
echo "$HASH  $SWIPL_DMG" > checksum

curl "https://www.swi-prolog.org/download/stable/bin/$SWIPL_DMG" > "$SWIPL_DMG"
shasum -a 256 -c checksum
sudo hdiutil attach "$SWIPL_DMG"
cp -rf /Volumes/"$SWIPL_DMG_NAME"/*.app /Applications
