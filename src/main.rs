use pulldown_cmark::{html as md_html, Options, Parser};
use yew::virtual_dom::VNode;
use yew::web_sys::Node;
use yew::{classes, html, web_sys, Component, ComponentLink, Html, InputData, ShouldRender};

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
            input_value: "## Please Write Your Markdown!!".into(),
            link,
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
        let markdown_input_string = &self.input_value.clone();
        let markdown_input = markdown_input_string.as_str();
        println!("Parsing the following markdown string:\n{}", markdown_input);

        // Set up options and parser. Strikethroughs are not part of the CommonMark standard
        // and we therefore must enable it explicitly.
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(markdown_input, options);

        // Write to String buffer.
        let mut html_output: String = String::with_capacity(markdown_input.len() * 3 / 2);
        md_html::push_html(&mut html_output, parser);

        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        div.set_inner_html(&html_output);
        let node = Node::from(div);
        let vnode = VNode::VRef(node);

        html! {
            <div class=classes!("container")>
                <div class=classes!("mdinput")>
                <textarea value={self.input_value.clone()}
                oninput={self.link.callback(|e: InputData| Msg::OnInput(e.value))}
                 />
                </div>
                <div class=classes!("htmloutput")>
                {vnode}
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
