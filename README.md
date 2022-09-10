## new publish

``` sh
yarn build
yarn publish
```

## old publish
```sh
rm -r pkg/ target/
wasm-pack build --scope fetsorn
(cd pkg && wasm-pack publish --access=public)
```
