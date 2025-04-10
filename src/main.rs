use std::collections::HashMap;
use std::fmt::Formatter;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};

pub mod pzn {
    pub mod serde {
        pub mod chrono {
            pub mod to_ms {
                use std::fmt::Formatter;
                use chrono::{DateTime, NaiveDateTime};
                use serde::de::{Error, Visitor};
                use serde::{Deserialize, Deserializer, Serializer};

                pub fn serialize<S>(datetime: &NaiveDateTime, serializer: S)
                                    -> Result<S::Ok, S::Error> where S: Serializer {
                    let ms = datetime.and_utc().timestamp_millis();
                    serializer.serialize_i64(ms)
                }

                struct NaiveDateTimeVisitor;

                impl <'de> Visitor<'de> for NaiveDateTimeVisitor {
                    type Value = NaiveDateTime;

                    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                        formatter.write_str("Expexting u64")
                    }

                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                    where
                        E: Error
                    {
                        let datetime = DateTime::from_timestamp_millis(v as i64).unwrap().naive_utc();
                        Ok(datetime)
                    }
                }

                pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error> where D: Deserializer<'de>,
                {
                    deserializer.deserialize_u64(NaiveDateTimeVisitor)
                }
            }
        }
    }
}


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

struct NameVisitor;

impl<'de> Visitor<'de> for NameVisitor {
    type Value = Name;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("expecting name as string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error
    {
        let result: Vec<&str> = value.split(" ").collect();
        if result.len() != 2 {
            return Err(E::custom("expecting first name and last name"));
        }

        Ok(Name{
            first: result[0].to_string(),
            last: result[1].to_string()
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Admin {
    id: String,
    name: Name,
    #[serde(with ="crate::pzn::serde::chrono::to_ms")]
    created_at: NaiveDateTime,
    #[serde(with ="crate::pzn::serde::chrono::to_ms")]
    updated_at: NaiveDateTime,
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

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_string(NameVisitor)
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_toml() {
    let category: Category = Category{
        id: "123".to_string(),
        name: "Test Toml".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let toml = toml::to_string(&category).unwrap();
    println!("{}", toml);

    let result: Category = toml::from_str(&toml).unwrap();
    println!("{:?}", result);
}

#[test]
fn test_custom_serialize() {
    let admin = Admin{
        id: "admin".to_string(),
        name: Name{
            first: "ekotaro".to_string(),
            last: "kuroniwa".to_string()
        },
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    let json: String = serde_json::to_string(&admin).unwrap();
    println!("{}", json);

    let result: Admin = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);
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