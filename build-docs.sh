# To execute, run `use build-docs.sh`.
cargo doc --no-deps;
rm -Rf docs;
mv target/doc docs;
"<!DOCTYPE html>
<html>
  <head>
    <meta http-equiv=\"refresh\" content=\"0; url='./acid/index.html'\" />
  </head>
</html>" >> docs/index.html;

cd examples/acid-web;
let OUT_DIR = "../../docs/acid/web-impl";
rm -Rf OUT_DIR;
mkdir -p OUT_DIR;
cp index.html OUT_DIR;
wasm-pack build --target web --out-dir OUT_DIR;