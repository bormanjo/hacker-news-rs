use client::api;

macro_rules! parametrized_get_stories_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[tokio::test]
        async fn $name() {
            let hn = api::HNClient::new();
            let story_ids = hn.get_stories($value).await.unwrap();
            assert!(story_ids.len() > 0);
        }
    )*
    }
}

parametrized_get_stories_tests! {
    test_get_new_stories: api::Story::New,
    test_get_top_stories: api::Story::Top,
    test_get_best_stories: api::Story::Best,
}


#[tokio::test]
async fn test_get_item() {
    let hn = api::HNClient::new();
    let item: api::Item = hn.get_item(1).await;
    assert_eq!(*item.id(), 1u32);
}
