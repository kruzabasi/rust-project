use crate::helpers::TestApp;

#[tokio::test]
async fn verify_token_works() {
    let app = TestApp::new().await;
    let jwt_token = "your_jwt_token_here";
    let response = app.verify_token(jwt_token).await;
    assert_eq!(response.status().as_u16(), 200);
}