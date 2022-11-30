use yew::{Component, Context, Html, html};

use crate::aframe;

pub enum AppMsg {}


pub struct App {}

impl Component for App {
	type Message = AppMsg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self {}
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		//<a-box position="-1 0.5 -3" rotation="0 45 0" color="#4CC3D9"></a-box>
		// <KanjiBox text="見学" position={Xyz::new(0.0, 0.5, -3.5)}/>
		// 	<KanjiBox text="すべて" position={Xyz::new(-2.0, 1.5, -3.0)} rotation={Xyz::new(0.0, 45.0, 0.0)}/>
		// <a-entity environment="preset: forest; dressingAmount: 500"></a-entity>
		let html = html! {
			<aframe::AScene/>
		};
		html
	}
}
