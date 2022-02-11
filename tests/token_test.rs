use ftapi::token::{token_info, check_token_valide};

#[tokio::test]
async fn token_info_fail_test() {
    let res = token_info(Some("not working token".to_string())).await;
    // let res = token_info(None).await;
    if let Ok(token_info) = res {
        println!("{:?}", token_info); // cargo run test -- --nocapture
        assert_eq!(token_info.resource_owner_id.is_none(), true);
    }
}

/*
#[tokio::test]
async fn token_info_success_test() {
    let res =
        token_info("Some Working Token")
            .await;
    if let Ok(token_info) = res {
        println!("{:?}", token_info);
        assert_eq!(token_info.resource_owner_id.is_none(), false);
    }
}
*/

#[tokio::test]
async fn check_token_valide_fail_test() {
    let res = check_token_valide(Some("not working token".to_string())).await;
    // let res = check_token_valide(None).await;
    if let Ok(t) = res {
        assert_eq!(t, false);
    }
}

/*
#[tokio::test]
async fn check_token_valide_success_test() {
    let res = check_token_valide(
        "Some Working Token",
    )
    .await;
    if let Ok(t) = res {
        assert_eq!(t, true);
    }
}
*/

/*
#[tokio::test]
async fn authorize_test() {
    // Don't forget to test with --nocapture option
    let res = generate_token(Session {
        client_id: "YOUR CLIENT_ID".to_string(),
        client_secret: "YOUR CLIENT SECRET"
            .to_string(),
    })
    .await;
    if let Ok(t) = res {
        assert_ne!(t, "".to_string());
    }
}
*/