use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct UserLoginRequest {
    username: String,
    password: String,
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_create_json_for_user_login_request() {
    let login_request = UserLoginRequest {
        username: "testuser".to_string(),
        password: "testpassword".to_string(),
    };

    let json = serde_json::to_string(&login_request).unwrap();
    println!("{}", json);

    let login_result: UserLoginRequest = serde_json::from_str(&json).unwrap();
    println!("{:?}", login_result);
}