use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/utils/javascript/js-scripts.js")]
extern "C" {

    #[wasm_bindgen(js_name = "createPouchDatabase")]
    pub fn create_pouch_database(name: String);

    #[wasm_bindgen(js_name = "replicateDatabase")]
    pub fn replicate(database_name: String);

}

// #[wasm_bindgen(module = "../static/main/public/assets/js/pouchdb-7.2.1.min.js")]
// extern "C" {
//
//     #[wasm_bindgen(js_name = default)]
//     pub type PouchDB;
//
//     #[wasm_bindgen(constructor, js_class=default)]
//     pub fn new(name: String) -> PouchDB;
//
//     #[wasm_bindgen(method, js_class = default)]
//     pub fn info(this: &PouchDB) -> Promise;
//
// }
