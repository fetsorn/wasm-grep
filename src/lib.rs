extern crate wasm_bindgen;
extern crate grep_regex;
extern crate grep_matcher;
extern crate grep_searcher;

use wasm_bindgen::prelude::*;
use grep_regex::RegexMatcher;
use grep_searcher::Searcher;
use grep_searcher::sinks::UTF8;

#[wasm_bindgen]
pub async fn grep(textfile: String, patternfile: String) -> Result<JsValue, JsValue> {
    let mut matches: Vec<String> = vec![];
    let patterns: &str = patternfile.strip_suffix("\n").unwrap();
    for pattern in patterns.split("\n") {
        let matcher = RegexMatcher::new(&pattern).unwrap();
        Searcher::new().search_slice(&matcher, textfile.as_bytes(), UTF8(|_, line| {
            matches.push(line.to_string());
            Ok(true)
        })).unwrap();
    }
    matches.sort();
    matches.dedup();
    let promise = js_sys::Promise::resolve(&(matches.join("\n")).into());
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(result)
}
