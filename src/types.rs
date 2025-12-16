use yew::{prelude::*};

#[derive(Clone)]
#[derive(Properties, PartialEq)]
pub struct PostScore {
    pub like_count: i64,
    pub comment_count: i64,
    pub share_count: i64,
}

#[derive(Clone)]
pub struct Post{
    pub title: String,
    pub date: String,
    pub content: String,

    pub score: UseStateHandle<PostScore>,
}
