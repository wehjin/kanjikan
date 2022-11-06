#!/bin/sh
trunk build --release --filehash false
rm -rf docs/
mkdir docs/
cp dist/* docs/
