rm -r pkg
mkdir -p pkg/assets
tailwindcss -i ./input.css -o ./assets/tailwind.css

## --release or --dev - exclude/include debug info
## --no-typescript - disable .d.ts files output
## --out-dir - where to write the compiled files
## --out-name - force output file names
## --target - always use "web"!
## See https://rustwasm.github.io/wasm-pack/book/commands/build.html
echo Building wasm module...
wasm-pack build --dev --no-typescript --out-dir "./pkg" --target web

## wasm-pack creates bunch of useless files:
echo Removing trash files...
rm -f pkg/.gitignore
rm -f pkg/package.json

cp ./manifest_v3.json ./pkg/manifest.json
cp assets/tailwind.css pkg/assets/tailwind.css

# These are needed otherwise you'll get the following error:
#    Uncaught EvalError: Refused to evaluate a string as JavaScript because 
#    'unsafe-eval' is not an allowed source of script in the following Content 
#    Security Policy directive: "script-src 'self' 'wasm-unsafe-eval'  
cp ./index.js ./pkg/index.js
cp ./index.html ./pkg/index.html

