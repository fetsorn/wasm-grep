const wasm = await import('../pkg/bundler/index.js')

const contents = "line1\nline2\nline3\nline22\n"
const query = "2\n1\n"

const result = await wasm.grep(contents, query, true)

console.log(`text ${contents}`)
console.log(`search ${query}`)
console.log(`found ${result}`)
