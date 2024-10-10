use crate::helpers::TestApp;

#[tokio::test]
async fn verify_2fa_works() {
    let app = TestApp::new().await;
    let code = "your_2fa_code_here";
    let response = app.verify_2fa(code).await;
    assert_eq!(response.status().as_u16(), 200);
}