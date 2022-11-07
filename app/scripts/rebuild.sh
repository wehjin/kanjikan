#!/bin/sh
DEST=../docs
trunk build --release --filehash false --public-url kanjikan/
echo "Removing destination" && rm -rf $DEST
echo "Making destination" && mkdir $DEST
echo "Copying new files to destination" && cp dist/* $DEST
