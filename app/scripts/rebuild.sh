#!/bin/sh
DEST=../docs
trunk build --release --filehash false
rm -rf $DEST 
mkdir $DEST 
cp dist/* $DEST
