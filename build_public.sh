rm -rf public
cd app
rm -rf dist
trunk build --release
cp -r dist ../public
