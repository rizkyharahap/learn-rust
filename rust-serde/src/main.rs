use std::{collections::HashMap, fmt::Formatter};

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error, Visitor},
};

pub mod rizky {
    pub mod serde {

        pub mod chrono {

            pub mod to_ms {
                use chrono::{DateTime, NaiveDateTime};
                use serde::de::{Error, Visitor};
                use serde::{Deserializer, Serializer};

                pub fn serialize<S>(
                    datetime: &NaiveDateTime,
                    serializer: S,
                ) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    let ms = datetime.and_utc().timestamp_millis();
                    serializer.serialize_i64(ms)
                }

                struct NaiveDateTimeVisitor;

                impl<'de> Visitor<'de> for NaiveDateTimeVisitor {
                    type Value = NaiveDateTime;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("Expecting u64")
                    }

                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                    where
                        E: Error,
                    {
                        let datetime = DateTime::from_timestamp_millis(v as i64)
                            .unwrap()
                            .naive_utc();

                        Ok(datetime)
                    }
                }

                pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    deserializer.deserialize_u64(NaiveDateTimeVisitor)
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
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

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequest {
    username: String,
    password: String,
    email: String,

    #[serde(rename = "alamat")]
    address: AddressRequest,
}

#[test]
fn test_create_json_for_create_user_request() {
    let request = CreateUserRequest {
        username: "testuser".to_string(),
        password: "testpassword".to_string(),
        email: "test@localhost.com".to_string(),
        address: AddressRequest {
            street: "Jalan".to_string(),
            state: "Provinsi".to_string(),
            city: "Kota".to_string(),
            zip: "134241".to_string(),
        },
    };

    let json = serde_json::to_string(&request).unwrap();
    println!("{}", json);

    let result: CreateUserRequest = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);
}

#[test]
fn test_create_json_from_array() {
    let numbers = [10, 11, 12, 13, 14];
    let json = serde_json::to_string(&numbers).unwrap();
    println!("{}", json);
}

#[derive(Debug, Serialize, Deserialize)]
enum Gender {
    Male,
    Female,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde[tag = "type"]]
enum Payment {
    CreditCard {
        card_number: String,
        expiration: String,
    },
    BankAccount {
        account_number: String,
        bank_name: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde[rename_all(serialize = "SCREAMING_SNAKE_CASE", deserialize = "SCREAMING_SNAKE_CASE")]]
struct User {
    username: String,
    password: String,
    first_name: String,
    last_name: String,
    hobbies: Vec<String>,
    phone: Option<String>,
    gender: Gender,
    payment: Payment,
}

#[test]
fn test_vector() {
    let user = User {
        username: "testuser".to_string(),
        password: "testpassword".to_string(),
        first_name: "Test User".to_string(),
        last_name: "Test User".to_string(),
        hobbies: vec!["reading".to_string(), "swimming".to_string()],
        phone: Some("08213213123".to_string()),
        gender: Gender::Male,
        payment: Payment::BankAccount {
            account_number: "1231231".to_string(),
            bank_name: "BCA".to_string(),
        },
    };

    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json);

    let result: User = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);
}

#[test]
fn test_map() {
    let mut values = HashMap::new();
    values.insert("one".to_string(), 1);
    values.insert("two".to_string(), 2);
    values.insert("three".to_string(), 3);

    let json = serde_json::to_string(&values).unwrap();
    println!("{}", json);

    let result: HashMap<String, i32> = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);
}

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    id: String,
    name: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    updated_at: DateTime<Utc>,
}

#[test]
fn test_chrono() {
    let category = Category {
        id: "gadget".to_string(),
        name: "Gadget".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let json = serde_json::to_string(&category).unwrap();
    println!("{}", json);

    let result: Category = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);
}

#[derive(Debug)]
struct Name {
    first: String,
    last: String,
}

struct NameVisitor;

impl<'de> Visitor<'de> for NameVisitor {
    type Value = Name;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Expecting name string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let result: Vec<&str> = v.split(" ").collect();
        if result.len() != 2 {
            return Err(Error::custom("Expecting first and last name"));
        }

        return Ok(Name {
            first: result[0].to_string(),
            last: result[1].to_string(),
        });
    }
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return serializer.serialize_str(format!("{} {}", self.first, self.last).as_str());
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        return deserializer.deserialize_string(NameVisitor);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Admin {
    id: String,
    name: Name,
    #[serde(with = "crate::rizky::serde::chrono::to_ms")]
    created_at: NaiveDateTime,
    #[serde(with = "crate::rizky::serde::chrono::to_ms")]
    updated_at: NaiveDateTime,
}

#[test]
fn test_custom_serialize() {
    let admin = Admin {
        id: "admin".to_string(),
        name: Name {
            first: "Rizki".to_string(),
            last: "Harahap".to_string(),
        },
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let json = serde_json::to_string(&admin).unwrap();
    println!("{}", json);

    let result: Admin = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);
}

#[test]
fn test_toml() {
    let category = Category {
        id: "gadget".to_string(),
        name: "Gadget".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let json = toml::to_string(&category).unwrap();
    println!("{}", json);

    let result: Category = toml::from_str(&json).unwrap();
    println!("{:?}", result);
}
