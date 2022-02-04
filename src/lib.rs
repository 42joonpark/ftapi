pub mod error;
pub mod token;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn token_test() {
        let res = crate::token::get_token_info("working token").await;
        if let Ok(token_info) = res {
            println!("{:?}", token_info);
            assert_eq!(token_info.application.is_none(), false);
        }
    }
}