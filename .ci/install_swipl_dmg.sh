#!/bin/bash
HASH='1fd495fea2e523b098c7221c092fb6403cbeed7f9c99df3737cd336bb39d6b84'
SWIPL_DMG_NAME='swipl-8.2.1-1.x86_64'
SWIPL_DMG="$SWIPL_DMG_NAME.dmg"
echo "$HASH  $SWIPL_DMG" > checksum

curl "https://www.swi-prolog.org/download/stable/bin/$SWIPL_DMG" > "$SWIPL_DMG"
shasum -a 256 -c checksum
sudo hdiutil attach "$SWIPL_DMG"
cp -rf /Volumes/"$SWIPL_DMG_NAME"/*.app /Applications
