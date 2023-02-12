extern crate wasm_grep;

use wasm_bindgen_test::*;
use wasm_bindgen::JsValue;
use wasm_grep::grep;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn searches() {
    let textfile = "first
second
third";
    let patternfile = "s\n";
    let result = "first\nsecond\n";
    assert_eq!(grep(textfile.to_string(), patternfile.to_string(), false).await,
               Ok(JsValue::from_str(result)));
}

#[wasm_bindgen_test]
async fn searches_eol() {
    let textfile = "first
second
third";
    let patternfile = "d$\n";
    let result = "second\nthird\n";
    assert_eq!(grep(textfile.to_string(), patternfile.to_string(), false).await,
               Ok(JsValue::from_str(result)));
}

#[wasm_bindgen_test]
async fn searches_patternfile_trailing_newline() {
    let textfile = "first
second
third";
    let patternfile = "first
";
    let result = "first\n";
    assert_eq!(grep(textfile.to_string(), patternfile.to_string(), false).await,
               Ok(JsValue::from_str(result)));
}

#[wasm_bindgen_test]
async fn searches_patternfile_no_trailing_newline() {
    let textfile = "first
second
third";
    let patternfile = "first";
    let result = "first\n";
    assert_eq!(grep(textfile.to_string(), patternfile.to_string(), false).await,
               Ok(JsValue::from_str(result)));
}

#[wasm_bindgen_test]
async fn searches_no_patternfile() {
    let textfile = "first
second
third";
    let patternfile = "";
    let result = "";
    assert_eq!(grep(textfile.to_string(), patternfile.to_string(), false).await,
               Ok(JsValue::from_str(result)));
}

#[wasm_bindgen_test]
async fn searches_wildcard() {
    let textfile = "first
second
third";
    let patternfile = "ir.*";
    let result = "first\nthird\n";
    assert_eq!(grep(textfile.to_string(), patternfile.to_string(), false).await,
               Ok(JsValue::from_str(result)));
}
