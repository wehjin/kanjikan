#!/bin/sh
DEST=..
trunk build --release --filehash false
cp dist/* $DEST
