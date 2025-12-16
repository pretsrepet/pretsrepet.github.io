use chrono::Local;
use yew::{prelude::*};

use crate::types::Post;

use crate::components::{NewPost, PostAction};

use crate::types::PostScore;

#[function_component(App)]
pub fn app() -> Html {
    let posts = use_state(|| Vec::<Post>::new());

    let on_add_post = {
        let posts = posts.clone();
        let score =  use_state(|| PostScore { like_count: 0, comment_count: 0, share_count: 0 });

        Callback::from(move |(title, content): (String, String)| {
            let mut new_posts = (*posts).clone();
            let score = score.clone();
            new_posts.push(Post {
                title,
                content,
                date: Local::now().format("%B %d, %Y").to_string(),
                score: score
            });
            posts.set(new_posts);
        })
    };


    html! {
        <div>
            <header>
                <div class="container">
                    <h1 class="bigass">{"Janky3c Blog Post"}</h1>
                </div>
            </header>


            <NewPost {on_add_post} />

            <div class="container">
                <div class="main_content">
                    {
                    for posts.iter().map(|post| html! {
                    <div class="blogpost">
                        <div class="post_header">
                            <p class="title_text">{ post.title.clone() }</p>

                            <PostAction score={post.score.clone()}/>

                        </div>
                        <p class="date_text">{format!("Posted on {}", post.date)}</p>
                        <p class="content_text">{ post.content.clone() }</p>
                    </div>
                    })
                    }
                </div>
            </div>
        </div>
        }
}
