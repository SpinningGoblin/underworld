rm -rf public

cd docs
mdbook build

cd ../app
npm run build
cp -r dist ../public
