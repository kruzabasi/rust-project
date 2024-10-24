use crate::helpers::{TestApp, get_random_email};

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "email": get_random_email(),
        }),
        serde_json::json!({ 
            "password": "password123",
            "requires2FA": "invalid_boolean"
        }),
    ];

    for test_case in test_cases {
        let response = app.login(&test_case).await;
        assert_eq!(response.status().as_u16(), 422, "Failed for input: {:?}", test_case);
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "email": "",
            "password": "password"
        }),
        serde_json::json!({
            "email": "test@example.com",
            "password": ""
        }),
    ];

    for test_case in test_cases {
        let response = app.login(&test_case).await;
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", test_case);
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;
    let response = app.login(&serde_json::json!({
        "email": "invalid_email@example.com",
        "password": "invalid_password"
    })).await;
    assert_eq!(response.status().as_u16(), 401);
}