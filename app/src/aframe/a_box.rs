use yew::{Context, Html, html};

use crate::aframe::AEntity;

pub struct ABox {}

impl yew::Component for ABox {
	type Message = ();
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		ABox {}
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		let html = html! {
			<AEntity/>
		};
		html
	}
}
