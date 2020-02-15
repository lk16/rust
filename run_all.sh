#!/bin/bash

# the first "project" doesn't use cargo
echo "running code in chapter01/hello_world"
cd chapter_01/hello_world
rustc main.rs
./main
cd ../..

for dir in `ls -d ./*/* | grep -v hello_world`
do
    echo "running code in $dir"
    cd $dir
    cargo run
    cd ../..
done
