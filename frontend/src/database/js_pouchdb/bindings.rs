use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "pouchdb")]
extern "C" {

    #[wasm_bindgen(js_name = default)]
    pub type PouchDB;

    #[wasm_bindgen(constructor, js_class = default)]
    pub fn new(name: String) -> PouchDB;

    #[wasm_bindgen(method, js_class = default)]
    pub fn destroy(this: &PouchDB) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn info(this: &PouchDB) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn put(this: &PouchDB, doc: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn get(this: &PouchDB, docId: JsValue) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn get_all(this: &PouchDB) -> Promise;

    // #[wasm_bindgen(method, js_class = default)]
    // pub fn get_newest_million(this: &PouchDB) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn close(this: &PouchDB) -> Promise;

}
