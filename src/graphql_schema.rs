use std::collections::HashMap;

use firebase_rs::Firebase;
// graphql_schema.rs
use juniper::{RootNode};
use serde::{Serialize, Deserialize};
use dotenv::dotenv;



#[derive(Serialize,Deserialize,Debug)]
struct User {
  first_name: String,
  last_name: String,
  email:String,
  password:String
}

#[derive(Serialize,Deserialize,Debug)]
struct Response{
    name:String
}



#[juniper::object(description = "User")]
    impl User {
      pub fn first_name(&self) -> &str {
        self.first_name.as_str()
      }
      pub fn last_name(&self) -> &str {
        self.last_name.as_str()
      }
      pub fn email(&self) -> &str {
        self.email.as_str()
      }
      pub fn password(&self) -> &str {
        self.password.as_str()
      }
    }



pub struct QueryRoot;

#[juniper::object]
#[firebase_rs::Firebase]
impl QueryRoot {
  fn getAllUsers() -> Vec<User> {
    dotenv().ok();
    let link = std::env::var("DATABASE_LINK").expect("MAILCOACH_API_TOKEN must be set.");
    let firebase: Firebase = Firebase::new(&link).unwrap();
    let mut vec:Vec<User> = Vec::new();
    let users: HashMap<String, User> = get_users(&firebase);
    for value in users.values(){
      let new_user: User = User {
        first_name: value.first_name.to_owned(),
        last_name:value.last_name.to_owned(),
        email: value.email.to_owned(),
        password:value.password.to_owned(),
      };
      vec.push(new_user);
    }
    return vec;
    }
    fn findUser(user_id:String) -> User {
      dotenv().ok();
      let link = std::env::var("DATABASE_LINK").expect("MAILCOACH_API_TOKEN must be set.");
      let firebase: Firebase = Firebase::new(&link).unwrap();
      let user:User = get_user(&firebase,&user_id.to_owned());
      return user;
      }
    
}

pub struct MutationRoot;

#[juniper::object]
#[firebase_rs::Firebase]
impl MutationRoot {
  fn createUser(input_first_name:String,input_last_name:String,input_email:String,input_password:String) -> User {
    dotenv().ok();
    let link = std::env::var("DATABASE_LINK").expect("MAILCOACH_API_TOKEN must be set.");
    let firebase: Firebase = Firebase::new(&link).unwrap();
    let new_user = User {
      first_name: input_first_name.to_string(),
      last_name:input_last_name.to_string(),
      email:input_email.to_string(),
      password:input_password.to_string()
    };
    let response = set_user(&firebase, &new_user);

    return new_user;
    }
    
}







#[tokio::main]
async fn get_users(firebase_client: &Firebase) -> HashMap<String, User>{
  let firebase: Firebase = firebase_client.at("users");
  let users: Result<HashMap<String, User>, firebase_rs::errors::RequestError> = firebase.get::<HashMap<String, User>>().await;
  return users.unwrap();
}

#[tokio::main]
async fn get_user(firebase_client: &Firebase, id: &String) -> User{
  let firebase = firebase_client.at("users").at(&id);
  let user = firebase.get::<User>().await;
  return user.unwrap();
}

#[tokio::main]
async fn set_user(firebase_client: &Firebase, user: &User) -> Response{
  let firebase = firebase_client.at("users");
  let _users = firebase.set::<User>(&user).await;
  return string_to_response(&_users.unwrap().data);
}


fn string_to_response(s: &str) -> Response{
  serde_json::from_str(s).unwrap()
}







pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;
    
pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {})
}