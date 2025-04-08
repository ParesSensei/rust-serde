use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct UserLoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AddressRequest {
    street: String,
    city: String,
    state: String,
    zip: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequest {
    username: String,
    password: String,
    email: String,
    address: AddressRequest,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
    hobbies: Vec<String>,
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

#[test]
fn test_create_json_for_create_user_request() {
    let request = CreateUserRequest {
        username: "testuser".to_string(),
        password: "testpassword".to_string(),
        email: "ekotaro@gamil.com".to_string(),
        address: AddressRequest{
            street: "Jl sunan batu".to_string(),
            city: "kota".to_string(),
            state: "provinsi".to_string(),
            zip: "76243572634".to_string(),
        }
    };

    let json = serde_json::to_string(&request).unwrap();
    println!("{}", json);

    let request_result: CreateUserRequest = serde_json::from_str(&json).unwrap();
    println!("{:?}", request_result);
}

#[test]
fn test_create_json_from_array() {
    let numbers = [10, 11, 12, 13, 14, 15];
    let json = serde_json::to_string(&numbers).unwrap();
    println!("{}", json);
}

#[test]
fn test_vector() {
    let request = User {
        username: "testuser".to_string(),
        email: "ekotaro@gmail.com".to_string(),
        hobbies: vec!["reading".to_string(), "swimming".to_string(), "eating".to_string()],
    };

    let json = serde_json::to_string(&request).unwrap();
    println!("{}", json);

    let login_result: User = serde_json::from_str(&json).unwrap();
    println!("{:?}", login_result);
}