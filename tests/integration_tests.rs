macro_rules! parametrized_get_stories_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[tokio::test]
        async fn $name() {
            let hn = client::HNClient::new();
            let story_ids = hn.get_stories($value).await.unwrap();
            assert!(story_ids.len() > 0);
        }
    )*
    }
}

parametrized_get_stories_tests! {
    test_get_new_stories: client::Story::New,
    test_get_top_stories: client::Story::Top,
    test_get_best_stories: client::Story::Best,
}


#[tokio::test]
async fn test_get_item() {
    let hn = client::HNClient::new();
    let item: client::Item = hn.get_item(1).await.unwrap();
    assert_eq!(*item.id(), 1u32);
}