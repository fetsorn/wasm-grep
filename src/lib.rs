#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen;
extern crate grep_regex;
extern crate grep_matcher;
extern crate grep_searcher;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use grep_regex::RegexMatcher;
use grep_searcher::{SearcherBuilder, Searcher};
use grep_searcher::sinks::UTF8;

#[wasm_bindgen]
pub async fn grep(textfile: String, patternfile: String, is_inverted: bool) -> Result<JsValue, JsValue> {
    print!("{:#?}", is_inverted);

    let patterns = patternfile.trim_matches('\n')
                              .split("\n")
                              .filter(|line| {line != &""});


    if is_inverted {
        let mut sets: Vec<HashSet<String>> = vec![];

        let mut builder = SearcherBuilder::new();

        builder.invert_match(true);

        let mut searcher = builder.build();

        for pattern in patterns {
            let matcher = match RegexMatcher::new(&pattern) {
                Ok(u) => u,
                _ => continue,
            };

            let mut set = HashSet::new();

            match searcher.search_slice(&matcher,
                                        textfile.as_bytes(),
                                        UTF8(|_, line| {
                                            set.insert((line.trim_matches('\n').to_string() + "\n").to_string());
                                            Ok(true)
                                        })
            ) {
                Err(e) => panic!("{}", e),
                _ => (),
            };

            sets.push(set);
        }

        let (intersection, others) = sets.split_at_mut(1);

        let intersection = &mut intersection[0];

        for other in others {
            intersection.retain(|e| other.contains(e));
        }

        let set: HashSet<String> = sets[0].clone();

        let matches: Vec<String> = Vec::from_iter(set);

        let resultfile: String = matches.join("");

        let promise = js_sys::Promise::resolve(&resultfile.into());

        let result = wasm_bindgen_futures::JsFuture::from(promise).await?;

        Ok(result)
    } else {
        let mut matches: Vec<String> = vec![];

        let mut searcher = Searcher::new();

        for pattern in patterns {
            let matcher = match RegexMatcher::new(&pattern) {
                Ok(u) => u,
                _ => continue,
            };

            match searcher.search_slice(&matcher,
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
}
