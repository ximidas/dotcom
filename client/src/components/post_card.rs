use crate::pages::post_list::Post;
use crate::Route;
use yew::prelude::*;
use yew_router::components::Link;

pub struct PostCard {
    pub props: Props,
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub post: Post,
}

impl Component for PostCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        Self { props }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let post = &self.props.post;
        let no_image = "/no_image_available.jpg".to_string();
        let image = match &post.images {
            Some(images) => images.get(0),
            None => Some(&no_image),
        };

        html! {
            <div class="container">
                <Link<Route> to={Route::Post { id: post.id as u64 }} classes={classes!("post-image-list")}>
                    <div class="blog_card" style={format!("background-image: url({})", image.unwrap())}>
                        <div class="dark-overlay"></div>
                        <p class="post-line">{"Date: "} { &post.created_at }</p>
                        <p class="post-link">{ &post.title }</p>
                        <p class="post-line">{"reading time: ~"} {&post.content.len() / 900} {" min"}</p>
                    </div>
                </Link<Route>>
            </div>
        }
    }
}
