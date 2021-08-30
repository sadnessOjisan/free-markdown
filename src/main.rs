use yew::{html, Html, InputData,ShouldRender,ComponentLink,Component};

enum Msg {
    OnInput(String),
}

struct Model {
    link: ComponentLink<Self>,
    input_value: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            input_value: "".into(),
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnInput(value) => {
                let old_value = self.input_value.clone();
                self.input_value = value;
                true
    }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input value={self.input_value.clone()} 
                oninput={self.link.callback(|e: InputData| Msg::OnInput(e.value))} />
                <div>
                {self.input_value.clone()}
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}