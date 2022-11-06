#!/bin/sh
DEST=../docs
trunk build --release --filehash false --public-url kanjikan/
rm -rf $DEST
mkdir $DEST
cp dist/* $DEST
