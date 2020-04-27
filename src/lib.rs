extern crate serde_wasm_bindgen;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;

use wasm_bindgen::prelude::*;
use std::panic;

#[wasm_bindgen]
extern "C" {
	pub fn alert(s: &str);

	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);

	#[wasm_bindgen(js_namespace = console)]
	fn error(s: &str);
}

macro_rules! console_log {
	 ($($t:tt)*) => (crate::log(&format!($($t)*)))
}

macro_rules! console_error {
	 ($($t:tt)*) => (crate::error(&format!($($t)*)))
}

mod scheduler;
pub mod pickletime;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
	panic::set_hook(Box::new(console_error_panic_hook::hook));

	let window = web_sys::window().expect("no global `window` exists");
	let document = window.document().expect("no DOM exists");

	pickletime::hook(&document);

	let local_storage = window.local_storage().expect("no local storage exists");

	if let Some(ref local_storage) = local_storage {
		local_storage.set_item("test", "{\"json\": \"something\"}").expect("failed to write to webstorage");
		let test_json = local_storage.get_item("test").expect("failed to read from webstorage");
		console_log!("{:?}", test_json);
	}

	Ok(())
}
