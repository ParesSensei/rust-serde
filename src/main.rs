use std::collections::HashMap;
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
    #[serde(rename= "alamat")]
    address: AddressRequest,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize="SCREAMING_SNAKE_CASE", deserialize="SCREAMING_SNAKE_CASE"))]
struct User {
    username: String,
    email: String,
    first_name: String,
    hobbies: Vec<String>,
    phone: Option<String>,
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
        first_name: "ekotaro kanedo".to_string(),
        hobbies: vec!["reading".to_string(), "swimming".to_string(), "eating".to_string()],
        phone: None,
    };

    let json = serde_json::to_string(&request).unwrap();
    println!("{}", json);

    let login_result: User = serde_json::from_str(&json).unwrap();
    println!("{:?}", login_result);
}

#[test]
fn test_vector_with_option() {
    let request = User {
        username: "testuser".to_string(),
        email: "ekotaro@gmail.com".to_string(),
        first_name: "ekotaro kanedo".to_string(),
        hobbies: vec!["reading".to_string(), "swimming".to_string(), "eating".to_string()],
        phone: Some("123456789".to_string()),
    };

    let json = serde_json::to_string(&request).unwrap();
    println!("{}", json);

    let login_result: User = serde_json::from_str(&json).unwrap();
    println!("{:?}", login_result);
}

#[test]
fn test_map() {
    let mut values: HashMap<String, i32> = HashMap::new();
    values.insert("one".to_string(), 1);
    values.insert("two".to_string(), 2);
    values.insert("three".to_string(), 3);

    let json = serde_json::to_string(&values).unwrap();
    println!("{}", json);

    let result: HashMap<String, i32> = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);
}