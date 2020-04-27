use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
	fn setTimeout(closure: &Closure<dyn FnMut()>) -> f64;
	fn cancelInterval(token: f64);
}

#[wasm_bindgen]
pub struct Interval {
	closure: Closure<dyn FnMut()>,
	token: f64,
}

impl Interval {
	pub fn new<F: 'static>(millis: u32, f: F) -> Interval
		where
			F: FnMut()
	{
		// Construct a new closure.
		let closure = Closure::new(f);

		// Pass the closuer to JS, to run every n milliseconds.
		let token = setInterval(&closure, millis);

		Interval { closure, token }
	}

	pub fn run_now(self) -> Self {
		setTimeout(&self.closure);
		self
	}

	pub fn make_permanent(self) {
		Box::leak(Box::new(self));
	}
}

// When the Interval is destroyed, cancel its `setInterval` timer.
impl Drop for Interval {
	fn drop(&mut self) {
		cancelInterval(self.token);
	}
}
