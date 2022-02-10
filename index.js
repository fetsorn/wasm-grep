import('./pkg').then((module) => {
    console.log(module.grep("line1\nline2\nline3\nline22\n", "2\n1\n"))
})
