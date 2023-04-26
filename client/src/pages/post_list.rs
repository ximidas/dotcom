use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pagination::{PageQuery, Pagination};
use crate::components::post_card::PostCard;
use crate::Route;

const ITEMS_PER_PAGE: u64 = 3;

pub enum Msg {
    PageUpdated,
    PostsFetched(Vec<Post>),
    TotalPagesFetched(u64),
}

#[derive(Debug, Deserialize)]
struct PostsResponse {
    posts: Vec<Post>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Properties)]
pub struct Post {
    pub id: usize,
    pub title: String,
    pub content: String,
    pub username: String,
    pub images: Option<Vec<String>>,
    pub created_at: String,
}

pub struct PostList {
    posts: Vec<Post>,
    page: u64,
    total_pages: u64,
    _listener: LocationHandle,
}

fn current_page(ctx: &Context<PostList>) -> u64 {
    let location = ctx.link().location().unwrap();

    location.query::<PageQuery>().map(|it| it.page).unwrap_or(1)
}

impl Component for PostList {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let listener = ctx
            .link()
            .add_location_listener(link.callback(move |_| Msg::PageUpdated))
            .unwrap();

        let page = current_page(ctx);

        let mut post_list = Self {
            posts: vec![],
            page,
            total_pages: 1,
            _listener: listener,
        };

        post_list.fetch_posts(&ctx, page);
        post_list.fetch_total_pages(&ctx);

        post_list
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PageUpdated => {
                self.page = current_page(ctx);
                self.fetch_posts(&ctx, self.page);
            }
            Msg::PostsFetched(fetched_posts) => {
                self.posts = fetched_posts;
            }
            Msg::TotalPagesFetched(total_pages) => {
                self.total_pages = total_pages;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let page = self.page;

        html! {
        <div>
            { self.view_posts(ctx) }
                        <Pagination
                            {page}
                            total_pages={self.total_pages}
                            route_to_page={Route::Posts}
                        />
        </div>

                }
    }
}

impl PostList {
    fn fetch_posts(&mut self, ctx: &Context<Self>, page: u64) {
        let url = format!(
            "{}/api/posts/get_all?page={}&page_size={}",
            env!("API_URL"), page, ITEMS_PER_PAGE
        );
        let link = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_response: PostsResponse = Request::get(&url)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            link.send_message(Msg::PostsFetched(fetched_response.posts));
        });
    }

    fn view_posts(&self, _ctx: &Context<Self>) -> Html {
        let cards = self.posts.iter().map(|post| {
            html! {
                <PostCard post={post.clone()} key={post.id} />
            }
        });

        html! {
            <div class="posts">
                { for cards }
            </div>
        }
    }

    fn fetch_total_pages(&mut self, ctx: &Context<Self>) {
        let url = format!(
            "{}/api/posts/total_pages/{}",
            env!("API_URL"),
            ITEMS_PER_PAGE
        );
        let link = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            let total_pages: u64 = Request::get(&url)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            link.send_message(Msg::TotalPagesFetched(total_pages));
        });
    }
}
