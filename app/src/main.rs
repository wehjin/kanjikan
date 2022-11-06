extern crate gloo_utils;

use web_sys::console;
use yew::prelude::*;

const SCENE_INNER: &str = include_str!("a_scene_inner.html");

enum Msg {}

struct Model {}

impl Component for Model {
	type Message = Msg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self {}
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		let div = gloo_utils::document().create_element("a-scene").unwrap();
		div.set_inner_html(SCENE_INNER);
		console::log_1(&div);
		Html::VRef(div.into())
	}
}

fn main() {
	yew::start_app::<Model>();
}
