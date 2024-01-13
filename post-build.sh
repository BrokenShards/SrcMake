#! bin/bash

mkdir target/debug > /dev/null 2>&1
mkdir target/release > /dev/null 2>&1

cp -a templates/. target/debug/templates/
cp -a templates/. target/release/templates/

cp -a languages/. target/debug/languages/
cp -a languages/. target/release/languages/
