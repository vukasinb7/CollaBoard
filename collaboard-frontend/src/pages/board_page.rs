use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;


#[function_component(BoardPage)]
pub fn board_page() -> Html {
    #[no_mangle]
    #[wasm_bindgen(module = "/bundle.js")]
    extern "C" {
        fn render();
    }
    // use_effect(||{render()});



    html! {
        <div>
            <div id="root">

        </div>
        </div>
    }
}



