use yew::prelude::*;

use crate::components::typed::TypedEffect;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="overlay">
                <TypedEffect />
                <div class="overlay__inner">
                    <img src="ximidas.png" alt="ximidas" width="150" height="150" class="zoom-tilt" />
                    <h1 class="overlay__title">
                        <span id="element"></span>
                    </h1>
                    <div class="overlay__description">
                        <p class="about_p">
                            {"As a software engineer, I bring experience working with various programming languages. My passion for "}<strong>{"Rust"}</strong>{", in particular, led me to use it for creating this website."}
                        </p>
                        <p class="about_p">
                            {"The main objective of the website is to host my personal blog, where I will share notes and articles on a diverse range of topics related to computer science."}
                        </p>
                        <p class="about_p">
                            {"Currently, I'm developing an innovative "}
                            <strong>{"tourism project"}</strong>
                            {", leveraging my IT expertise to ensure its success and attractiveness to users."}
                        </p>
                        <p class="about_p">
                            {"I am available for contact, and you can reach me via the email address provided below."} 
                        </p>
                        <p class="about_p">
                            {"Alternatively, feel free to connect with me on one of the social media platforms whose icons are located on the website."}
                        </p>
                    </div>
                </div>
            </div>
        }
    }
}
