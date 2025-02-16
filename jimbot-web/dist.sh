cd "$(dirname "$0")"
rm -rf dist
mkdir dist

wasm-pack build --release --target web --out-dir dist/jimbot-web-wasm

cp ./index.html ./dist/
