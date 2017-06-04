cargo clean
mkdir web
cargo build-wasm --release
git stash
git checkout gh-pages
cp -p web/* .
rm -rf web
git add .
git commit -m "Deployed"
git push
git checkout master
git stash pop
