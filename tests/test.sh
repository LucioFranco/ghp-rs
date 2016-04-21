function check_bundle() {
    if [ -f bundle.js ]
    then
	echo "Passed test #1"
	return
    fi

    echo "Failed test #1: could not find bundle.js in gh-pages branch"
    exit 1
}




mkdir tmp
cd tmp

echo "build" > .gitignore
git commit -m "gitignore" -- .gitignore

echo "<html></html>" > index.html
mkdir css
echo "// main css file" > css/main.css


git init
git add --all
git commit -m "inital commit"

mkdir build
echo "// a bunch of js bs" > build/bundle.js

cargo run -- build

sleep 2
rm -rf build
git checkout gh-pages
check_bundle

cd ..
rm -rf tmp

exit 0
