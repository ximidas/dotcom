use yew::prelude::*;

use crate::components::typed::TypedEffect;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub struct Skills;
impl Component for Skills {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    // TODO refactor: Transform into a beautiful list all skills and upon clicking on one of them, display a description of the experience related to it.
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <div class="overlay">
            <TypedEffect />
        <div class="overlay__inner">
            <div class="overlay__description">
                <p class="about_p">
                    {"Programming languages: "}
                    <strong>{"Rust, Python, JavaScript, TypeScript, Ruby, PHP"}</strong>
                </p>
                <p class="about_p">
                    {"Databases: "}
                    <strong>{"PostgreSQL, MySQL, MariaDB, Neo4j, Mongodb, ClickHouse, Redis"}</strong>
                </p>
                <p class="about_p">
                    {"Broker: "}
                    <strong>{"RabbitMQ"}</strong>
                </p>
                <p class="about_p">
                    {"Linux: "}
                    <strong>{"Nginx"}</strong>
                </p>
                <p class="about_p">
                    {"CI/CD: "}
                    <strong>{"Gitlab Runner, Github CI"}</strong>
                </p>
                <p class="about_p">
                    {"OS: "}
                    <strong>{"Linux, Windows, MacOS"}</strong>
                </p>
                <p class="about_p">
                    {"Virtualization: "}
                    <strong>{"Docker, WSL"}</strong>
                </p>
                <p class="about_p">
                    <strong>{"Rust: "}</strong> {"Actix, Axum, Yew"}
                </p>
                <p class="about_p">
                    <strong>{"Python: "}</strong> {"Pytorch, Tensorflow, Django, Flask"}
                </p>
                <p class="about_p">
                    <strong>{"JavaScript & TypeScript: "}</strong> {"Node.js, Fastify, Express, React, Vue"}
                </p>
                <p class="about_p">
                    <strong>{"Ruby: "}</strong> {"Ruby on Rails"}
                </p>
                <p class="about_p">
                    <strong>{"PHP: "}</strong> {"Swoole, Laravel, Symfony, CodeIgniter"}
                </p>
            </div>
          </div>
        </div>
          
            }
    }
}
