#! /bin/bash

cd frontend
npm i
npm run build

cd ..
cd docs
cargo install mdbook
mdbook build
cd ..
cp -r ./docs/book ./frontend/dist/docs
