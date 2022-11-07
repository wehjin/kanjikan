extern crate gloo_utils;
extern crate web_sys;

use yew::prelude::*;

use crate::aframe::Xyz;

enum Msg {}

struct Model {}

impl Component for Model {
	type Message = Msg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self {}
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		//<a-box position="-1 0.5 -3" rotation="0 45 0" color="#4CC3D9"></a-box>
		let x = html! {
			<a-scene>
				<a-entity laser-controls="hand: left" raycaster="objects: .collidable; far: 10"></a-entity>
				<a-entity laser-controls="hand: right" raycaster="objects: .collidable;"></a-entity>
				<a-sky src="https://cdn.glitch.global/78e9163c-0114-44ea-b6d2-3fb627944853/sky360.jpeg?v=1667676752316"></a-sky>
				<a-plane class="collidable" position="0 0 -4" rotation="-90 0 0" width="8" height="4" color="#7BC8A4"></a-plane>
				<a-sphere position="0 0 -2" radius="0.05" color="#333333"></a-sphere>
				<a-entity class="collidable" position="-2 0.5 -3" rotation="0 45 0" geometry="primitive: box" material="color: #4CC3D9"/>
				<a-sphere class="collidable" position="0 0.95 -6" radius="1" color="#EF2D5E"></a-sphere>
				<a-cylinder class="collidable" position="2 0.75 -3" radius="0.5" height="1.5" color="#FFC65D"></a-cylinder>
				<KanjiBox text="見学" position={Xyz::new(0.0, 0.5, -3.5)}/>
				<KanjiBox text="すべて" position={Xyz::new(-2.0, 1.5, -3.0)} rotation={Xyz::new(0.0, 45.0, 0.0)}/>
			</a-scene>
		};
		let html = x;
		html
	}
}

pub mod aframe {
	use yew::html::IntoPropValue;
	use yew::virtual_dom::AttrValue;

	#[derive(Copy, Clone, PartialEq, Default)]
	pub struct Xyz {
		pub x: f64,
		pub y: f64,
		pub z: f64,
	}

	impl Xyz {
		pub fn new(x: f64, y: f64, z: f64) -> Self {
			Xyz { x, y, z }
		}
	}

	impl IntoPropValue<Option<AttrValue>> for Xyz {
		fn into_prop_value(self) -> Option<AttrValue> {
			let s = format!("{} {} {}", self.x, self.y, self.z);
			let attr_value = AttrValue::from(s);
			Some(attr_value)
		}
	}
}

#[derive(Properties, PartialEq)]
pub struct KanjiBoxProps {
	pub text: String,
	pub position: aframe::Xyz,
	#[prop_or_default]
	pub rotation: aframe::Xyz,
}

impl Default for KanjiBoxProps {
	fn default() -> Self {
		KanjiBoxProps {
			text: String::new(),
			position: Xyz::default(),
			rotation: Xyz::default(),
		}
	}
}

#[function_component(KanjiBox)]
pub fn kanji_block(props: &KanjiBoxProps) -> Html {
	let text_value = props.text.clone();
	let html = html! {
		<a-entity position={props.position} rotation={props.rotation}>
			<a-text value={ text_value } position="0 0 0" side="double" color="black" align="center" shader="msdf" font="https://cdn.glitch.global/78e9163c-0114-44ea-b6d2-3fb627944853/NotoSansJP-Black.json?v=1667671610240" font-image="https://cdn.glitch.global/78e9163c-0114-44ea-b6d2-3fb627944853/NotoSansJP-Black.png?v=1667673945948"></a-text>
			<a-entity class="collidable" position="0 0 0" geometry="primitive: box; width: 1.0; height: 1.0" material="color: #445566; opacity: 0.4; transparent: true"/>
		</a-entity>
	};
	html
}

fn main() {
	yew::start_app::<Model>();
}
