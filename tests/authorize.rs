use ftapi::{self};

/// test my_authorize()
/// cargo run test -- --nocapture
#[tokio::test]
async fn authorize_test() {
    // Don't forget to test with --nocapture option
    let res = ftapi::authorize::my_authorize(ftapi::authorize::Session {
        client_id: "37b03bd9f3fa8ba93bc2736ef348fec878949ae649543d8cf0ea15c6743da0e3".to_string(),
        client_secret: "f9d31d50bd7fcdb68f83dfd961ba4184681f7baab8d0d8d8427c156a1fc1a733"
            .to_string(),
    })
    .await;
    if let Ok(t) = res {
        assert_ne!(t, "".to_string());
    }
}
