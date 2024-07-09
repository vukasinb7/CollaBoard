use wasm_bindgen::prelude::wasm_bindgen;


#[wasm_bindgen(module = "/src/excalidraw.js")]
extern "C" {
    #[wasm_bindgen(js_name = "toast")]
    fn toast(title:String,message:String);
}
pub fn toast_notification(title:String,message:String){
    toast(title,message);
}