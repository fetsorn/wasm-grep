# Example

Note that the binaries are available in `./node_modules/.bin`.

## Start Webpack

In watch and development mode:

```sh
npm start
```

## Start the HTTP server

```sh
npm run server
```

## publish
```sh
rm -r pkg/ target/
wasm-pack build --scope fetsorn
(cd pkg && wasm-pack publish --access=public)
```
