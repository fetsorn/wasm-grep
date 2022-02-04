extern crate wasm_bindgen;
extern crate grep_regex;
extern crate grep_matcher;
extern crate grep_searcher;

use wasm_bindgen::prelude::*;
use grep_matcher::Matcher;
use grep_regex::RegexMatcher;
use grep_searcher::Searcher;
use grep_searcher::sinks::UTF8;

#[wasm_bindgen]
extern "C" {
    type HTMLDocument;
    static document: HTMLDocument;
    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;
    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    type Element;
    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner_html(this: &Element, html: &str);
    #[wasm_bindgen(method, js_name = appendChild)]
    fn append_child(this: &Element, other: Element);
}

const SHERLOCK: &'static [u8] = b"\
          For the Doctor Watsons of this world, as opposed to the Sherlock
          Holmeses, success in the province of detective work must always
          be, to a very large extent, the result of luck. Sherlock Holmes
          can extract a clew from a wisp of straw or a flake of cigar ash;
          but Doctor Watson has to have it taken out for him and dusted,
          and exhibited clearly, with a label attached.
          ";

#[wasm_bindgen]
pub fn grep(textfile: String, patternfile: String) -> Result<String, JsValue> {
    let mut matches: Vec<String> = vec![];
    let patterns: &str = patternfile.strip_suffix("\n").unwrap();
    for pattern in patterns.split("\n") {
        let matcher = RegexMatcher::new(&pattern).unwrap();
        Searcher::new().search_slice(&matcher, textfile.as_bytes(), UTF8(|lnum, line| {
            let mymatch = matcher.find(line.as_bytes())?.unwrap();
            matches.push(line.to_string());
            Ok(true)
        })).unwrap();
    }
    matches.sort();
    matches.dedup();
    Ok(matches.join("\n"))
    //let patterns1: Vec<&str> = patterns.split("\n").collect();
    //Ok(patterns1.join("-"))
}

#[wasm_bindgen]
pub fn run() {
    let val = document.createElement("p");
    val.set_inner_html("Hello from Rust!");
    document.body().append_child(val);
}
