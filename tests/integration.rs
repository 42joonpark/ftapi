use ftapi::{self, token};

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[tokio::test]
async fn token_info_fail_test() {
    let res = token::get_token_info("not working token").await;
    if let Ok(token_info) = res {
        println!("{:?}", token_info); // cargo run test -- --nocapture
        assert_eq!(token_info.application.is_none(), true);
    }
}

/*
#[tokio::test]
async fn token_info_success_test() {
    let res =
        token::get_token_info("Some Working Token")
            .await;
    if let Ok(token_info) = res {
        println!("{:?}", token_info);
        assert_eq!(token_info.application.is_none(), false);
    }
}
*/

#[tokio::test]
async fn check_token_valide_fail_test() {
    let res = token::check_token_valide("not working token").await;
    if let Ok(t) = res {
        assert_eq!(t, false);
    }
}

/*
#[tokio::test]
async fn check_token_valide_success_test() {
    let res = token::check_token_valide(
        "Some Working Token",
    )
    .await;
    if let Ok(t) = res {
        assert_eq!(t, true);
    }
}
*/
