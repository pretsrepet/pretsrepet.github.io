use yew::{prelude::*};

use crate::{components::post_action, types::{_PostScore::like_count, PostScore}};

#[derive(Properties, PartialEq)]
pub struct PostActionProps {
    pub score: UseStateHandle<PostScore>
}

#[function_component(PostAction)]
pub fn post_action(props: &PostActionProps) -> Html {

    let on_click_like = {
        let score = props.score.clone();
        Callback::from(move |_| {
            score.set(PostScore {
                like_count: score.like_count + 1,
                comment_count: score.comment_count,
                share_count: score.share_count,
            });
        }) // <- no return value
    };

    html! {
        <div>
            <div class="post_actions">
                <button class="comment_button">
                    <img class="comment_button_img" src="assets/comment.png" alt="comment"/>
                </button>

                <button class="like_button" onclick={on_click_like}>
                    <img class="like_button_img" src="kdsfassets/like.png" alt="like"/>
                </button>


                <button class="share_button">
                    <img class="share_button_img" src="assets/share.png" alt="share"/>
                </button>

                <button class="edit_button">
                    <img class="edit_button_img" src="assets/edit.png" alt="edit"/>
                </button>
            </div>
            <div class="post_score">
                <p class="comment_count">{props.score.comment_count}</p>
                <p class="like_count">{props.score.like_count}</p>
                <p class="share_count">{props.score.share_count}</p>
            </div>
        </div>
        }
}
