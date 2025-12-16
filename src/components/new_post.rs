use yew::{prelude::*};
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_add_post: Callback<(String, String)>,
}

#[function_component(NewPost)]
pub fn new_post(props: &Props) -> Html {
    let title = use_state(|| String::new());
    let content = use_state(|| String::new());

    let on_name_input = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };

    let on_content_input = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            content.set(input.value());
        })
    };

    let on_click_upload = {
        let title = title.clone();
        let content = content.clone();
        let on_add_post = props.on_add_post.clone();

        if !title.is_empty() {
            Callback::from(move |_| {
                on_add_post.emit(((*title).clone(), (*content).clone()));
                title.set(String::new());
                content.set(String::new());
            })
        }else{
            Callback::from(move |_| {})
        }
    };

    let on_click_clear = {
        let title = title.clone();
        let content = content.clone();

        if !title.is_empty() {
            Callback::from(move |_| {
                title.set(String::new());
                content.set(String::new());
            })
        }else{
            Callback::from(move |_| {})
        }
    };

    html! {
        <div class="new_post">
            <label>
                <input oninput={on_name_input} placeholder="Post Name" class="post_name_field" value={(*title).clone()}/>
            </label>


            <div class="content_row">
                <label>
                    <textarea oninput={on_content_input} placeholder="Post Content" class="post_content_field" value={(*content).clone()}/>
                </label>

                <div class="buttons_new_post">
                    <button onclick={on_click_upload}>{ "Upload" }</button>
                    <button onclick={on_click_clear}>{ "Clear" }</button>
                </div>
            </div>

        </div>
        }
}

