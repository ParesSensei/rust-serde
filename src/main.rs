use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer};

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
    gender: Gender,
    payment: Payment,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum Payment {
    CreditCard {
        card_number: String,
        expiration: String,
    },
    BankAccount {
        account_number: String,
        bank_name: String
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Gender {
    Male,
    Female,
}

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    id: String,
    name: String,
    #[serde(with ="chrono::serde::ts_milliseconds")]
    created_at: DateTime<Utc>,
    #[serde(with ="chrono::serde::ts_milliseconds")]
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
struct Admin {
    id: String,
    name: Name,
}

#[derive(Debug)]
struct Name {
    first: String,
    last: String,
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(format!("{} {}", self.first, self.last).as_str())
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_custom_serialize() {
    let admin = Admin{
        id: "admin".to_string(),
        name: Name{
            first: "ekotaro".to_string(),
            last: "kuroniwa".to_string()
        }
    };

    let json: String = serde_json::to_string(&admin).unwrap();
    println!("{}", json);
}

#[test]
fn test_chrono() {
    let category: Category = Category {
        id: "gatget".to_string(),
        name: "Gadget".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let json = serde_json::to_string(&category).unwrap();
    println!("{}", json);

    let result: Category = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);
}

#[test]
fn test_create_json_for_user_login_request() {
    let login_request = UserLoginRequest {
        username: "testuser".to_string(),
        password: "testpassword".to_string(),
    };

    let json = serde_json::to_string(&login_request).unwrap();
    println!("{}", json);

    let result: UserLoginRequest = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);
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

    let result: CreateUserRequest = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);
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
        gender: Gender::Male,
        payment: Payment::BankAccount {
            bank_name: "Bank Bca".to_string(),
            account_number: "123123123".to_string(),
        }
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
        first_name: "ekataro kanedo".to_string(),
        hobbies: vec!["reading".to_string(), "swimming".to_string(), "eating".to_string()],
        phone: Some("123456789".to_string()),
        gender: Gender::Female,
        payment: Payment::CreditCard {
            card_number: "123123123".to_string(),
            expiration: "2020-03-23".to_string(),
        }
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