use js_sys::Promise;
use wasm_bindgen::prelude::*;

// TODO find a solution for browser and node
#[wasm_bindgen(module = "pouchdb")]
extern "C" {

    //#[wasm_bindgen] // works neither in browser nor in node?
    #[wasm_bindgen(js_name = default)] // works in browser with es6
    pub type PouchDB;

    //#[wasm_bindgen(constructor)]
    #[wasm_bindgen(constructor, js_class = default)] // works in browser with es6
    pub fn new(name: String) -> PouchDB;

    //#[wasm_bindgen(method)]
    #[wasm_bindgen(method, js_class = default)] // works in browser with es6
    pub fn info(this: &PouchDB) -> Promise;

    //#[wasm_bindgen(method)]
    #[wasm_bindgen(method, js_class = default)] // works in browser with es6
    pub fn put(this: &PouchDB, doc: JsValue) -> Promise;

    //#[wasm_bindgen(method)]
    #[wasm_bindgen(method, js_class = default)] // works in browser with es6
    pub fn get(this: &PouchDB, docId: JsValue) -> Promise;

    //#[wasm_bindgen(method)]
    #[wasm_bindgen(method, js_class = default)] // works in browser with es6
    pub fn close(this: &PouchDB) -> Promise;

    // #[wasm_bindgen(static_method_of = PouchDB, js_class = default)]
    // pub fn replicate(source: &JsValue, target: &JsValue) -> JsValue;
    //
    // #[wasm_bindgen(static_method_of = PouchDB, js_class = default, js_name = replicate)]
    // pub fn replicate_with_options(source: &JsValue, target: &JsValue, options: JsValue) -> JsValue;
}
