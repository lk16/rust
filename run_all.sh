#!/bin/bash

# the first "project" doesn't use cargo
echo "running code in chapter01/hello_world"
cd chapter_01/hello_world
rustc main.rs
./main
cd ../..

for dir in `ls -d ./*/* | grep -v hello_world | grep -v restaurant`
do
    echo "running code in $dir"
    cd $dir
    cargo run
    cd ../..
done

echo "checking code in chapter07/restaurant"
cd chapter_07/restaurant
cargo check
cd ../..
