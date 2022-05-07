use std::collections::HashMap;

use underworld_core::{
    components::rooms::room_view::RoomViewArgs,
    generators::{generator::Generator, rooms::random_room_generator},
    systems::view::room,
};
use yew::prelude::*;

enum RoomMsg {
    GenerateRoom,
}

struct RoomDescriptions {
    room_description: String,
    inhabitants_description: String,
}

impl Component for RoomDescriptions {
    type Message = RoomMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            room_description: "".to_string(),
            inhabitants_description: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RoomMsg::GenerateRoom => {
                let room_prototype = random_room_generator(None);
                let room = room_prototype.generate();
                let quick_view = room::look_at(&room, RoomViewArgs::default(), false);
                let deeper_look = room::view(&room, HashMap::new(), HashMap::new(), false);
                self.room_description = format!("{}", &quick_view);
                self.inhabitants_description = deeper_look.describe_inhabitants();
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| RoomMsg::GenerateRoom)}>{ "Generate" }</button>
                <p>{ &self.room_description }</p>
                <p>{ &self.inhabitants_description }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<RoomDescriptions>();
}
