use yew::prelude::*;
use yew_router::prelude::*;
use load_dotenv::load_dotenv;
use web_sys::{window, Location};

load_dotenv!();

mod components;
mod pages;
use pages::home::Home;
use pages::page_not_found::PageNotFound;
use pages::post::PostDetails;
use pages::post_list::PostList;
use pages::skills::Skills;
use yew::html::Scope;

use crate::components::particles::ParticlesEffect;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/skills")]
    Skills,
    #[at("/posts/:id")]
    Post { id: u64 },
    #[at("/posts")]
    Posts,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Msg {
    UrlUpdate(String),
}

pub struct App {
    current_url: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
      log::info!("it's easier to see it here - https://github.com/ximidas/dotcom");
      let window = window().expect("Failed to get window object");
      let location: Location = window.location();

      let pathname = location.pathname().expect("Failed to get pathname");
      let search = location.search().expect("Failed to get search");
      let hash = location.hash().expect("Failed to get hash");

      let current_url = format!("{}{}{}", pathname, search, hash);

      Self { current_url }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
      match msg {
          Msg::UrlUpdate(url) => {
              self.current_url = url;
              true
          }
      }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
              <BrowserRouter>
                  { self.view_nav(ctx.link()) }
                  <ParticlesEffect />
                  <div id="particles-js"></div>
                  <main>
                      <Switch<Route> render={switch} />
                  </main>
                  <div class="overlay__btns">
          <a href="https://github.com/ximidas" target="_blank" rel="noopener noreferrer">
            <button class="overlay__btn">
              <i class="github fab fa-github"></i>
            </button>
          </a>
          <a href="https://reddit.com/user/ximidas" target="_blank" rel="noopener noreferrer">
            <button class="overlay__btn">
              <i class="reddit fab fa-reddit"></i>
            </button>
          </a>
          <a href="https://twitter.com/ximidas" target="_blank" rel="noopener noreferrer">
            <button class="overlay__btn">
              <i class="twitter fab fa-twitter"></i>
            </button>
          </a>
          <a href="https://www.youtube.com/@XIMIDAS" target="_blank" rel="noopener noreferrer">
            <button class="overlay__btn">
              <i class="youtube fa-brands fa-youtube"></i>
            </button>
          </a>
          <a href="https://t.me/ximidas_blog" target="_blank" rel="noopener noreferrer">
            <button class="overlay__btn">
              <i class="telegram fa fa-telegram" aria-hidden="true"></i>
            </button>
          </a>
          <a href="https://www.instagram.com/ximidascom" target="_blank" rel="noopener noreferrer">
            <button class="overlay__btn">
              <i class="instagram fab fa-instagram"></i>
            </button>
          </a>
          <a href="https://www.linkedin.com/in/ximidas" target="_blank" rel="noopener noreferrer">
            <button class="overlay__btn">
              <i class="linkedin fab fa-linkedin"></i>
            </button>
          </a>
          <a href="https://huggingface.co/ximidas" target="_blank" rel="noopener noreferrer">
            <button class="overlay__btn">
              <img src="https://huggingface.co/front/assets/huggingface_logo-noborder.svg" width="15px" alt="Hugging Face logo" />
            </button>
          </a>
        </div>
        <div>
          <a href="mailto:ximidas@ximidas.com" class="footer-mail">{ "ximidas@ximidas.com" }</a>
          <p class="footer-text">
          { "The client-side of this website was built using "} <a href="https://yew.rs" target="_blank">{"Yew"}</a>{", while the server-side was built using "}<a href="https://actix.rs" target="_blank">{"Actix Web"}</a>{". The source code of this project can be viewed "}<a href="https://github.com/ximidas/dotcom" target="_blank">{"here"}</a>{"."}</p>
        </div>
              </BrowserRouter>
          }
    }
}

impl App {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        html! {
            <nav class="inner__nav">
                <ul>
                  <li class={if self.current_url == "/" { "active" } else { "" }} onclick={link.callback(|_| Msg::UrlUpdate("/".to_string()))}>
                          <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                      </li>
                    <li class={if self.current_url.contains("/posts") { "active" } else { "" }} onclick={link.callback(|_| Msg::UrlUpdate("/posts".to_string()))}>
                        <Link<Route> to={Route::Posts}>{ "Posts" }</Link<Route>>
                    </li>
                    <li class={if self.current_url == "/skills" { "active" } else { "" }} onclick={link.callback(|_| Msg::UrlUpdate("/skills".to_string()))}>
                        <Link<Route> to={Route::Skills}>{ "Skills" }</Link<Route>>
                    </li>
                    //<li>
                    //  {"Portfolio"}
                    //</li>
                    //<li>
                    //  <i class="fa fa-language" aria-hidden="true"> </i>{"English"}
                    //</li>
                </ul>
            </nav>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Skills => {
            html! { <Skills /> }
        }
        Route::Post { id } => {
            html! { <PostDetails id={id as usize} /> }
        }
        Route::Posts => {
            html! { <PostList /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
  yew::Renderer::<App>::new().render();
}
