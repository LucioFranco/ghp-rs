#!/bin/bash

source assertions.sh

echo "Creating tmp directory..."

mkdir tmp
cd tmp

echo "\nAdding files..."

echo "build" > .gitignore
echo "<html></html>" > index.html
mkdir css
echo "// main css file" > css/main.css

echo "Creating git repo with files..."

git init > /dev/null
git add --all
git commit -m "inital commit" > /dev/null

echo "Creating build folder and files..."

mkdir build
echo "// a bunch of js bs" > build/bundle.js
echo "# bash file" > build/run.sh
chmod +x build/run.sh

echo "Running command \"ghp build\"..."
../../target/debug/ghp build

rm -rf build
git checkout gh-pages
check_branch

cd ..
rm -rf tmp

exit 0
