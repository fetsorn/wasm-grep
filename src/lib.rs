#![cfg(target_arch = "wasm32")]

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
    let patterns = patternfile.trim_matches('\n')
                              .split("\n")
                              .filter(|line| {line != &""});
    for pattern in patterns {
        let matcher = match RegexMatcher::new(&pattern) {
            Ok(u) => u,
            _ => continue, 
        };
        match Searcher::new().search_slice(&matcher,
                                           textfile.as_bytes(),
                                           UTF8(|_, line| {
                                               matches.push((line.trim_matches('\n').to_string() + "\n").to_string());
                                               Ok(true)
                                           })
        ) {
            Err(e) => panic!("{}", e),
            _ => (),
        };
    }
    matches.sort();
    matches.dedup();
    let resultfile: String = matches.join("");
    let promise = js_sys::Promise::resolve(&resultfile.into());
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(result)
}

