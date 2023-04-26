use yew::prelude::*;
use gloo_net::http::Request;
use crate::pages::post_list::Post;

pub enum Msg {
    PostFetched(Post),
}

pub struct PostDetails {
    post: Option<Post>,
    id: usize,
}

#[derive(Clone, Properties, PartialEq, Eq)]
pub struct Props {
    pub id: usize,
}

impl Component for PostDetails {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        let mut post_details = Self {
            post: None,
            id: props.id,
        };

        post_details.fetch_post(&ctx);

        post_details
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PostFetched(fetched_post) => {
                self.post = Some(fetched_post);
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        let new_props = ctx.props().clone();
        if self.id != new_props.id {
            self.id = new_props.id;
            self.fetch_post(&ctx);
            true
        } else {
            false
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        
        html! {
            <>
                { if let Some(post) = &self.post {
                    let image_address = &post.clone().images.unwrap_or(vec!["/no_image_available.jpg".to_string()]);
                    let image = format!("<img class=\"post-image\" width=\"250px\" src=\"{}\" />", &image_address[0]);
                    let parsed_image_html = Html::from_html_unchecked(AttrValue::from(image));
                    let parsed_content = Html::from_html_unchecked(AttrValue::from(post.content.clone()));
                    html! {
                        <div class="overlay">
                            <div class="overlay__inner-post">
                            {parsed_image_html}
                                <p>{"Reading time: ~"} {&post.content.len() / 600} {" min | Date: "} { &post.created_at } {" | Author: "} { &post.username }</p>
                            <h1 class="overlay__post-title">{ &post.title }</h1>
                            {parsed_content}
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div>{ "Loading post details..." }</div>
                    }
                } }
            </>
        }
    }
}

impl PostDetails {

    fn fetch_post(&mut self, ctx: &Context<Self>) {
        let url = format!("{}/api/posts/{}", env!("API_URL"), self.id);
        let link = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_post: Post = Request::get(&url)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            link.send_message(Msg::PostFetched(fetched_post));
        });
    }
}
