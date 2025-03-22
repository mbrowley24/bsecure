use bcrypt::{hash, DEFAULT_COST};
use std::collections::{HashMap};
use crate::models::defaults::{
    default_false
};
use regex::Regex;
use serde::{
    Serialize,
    Deserialize,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::user::user::User;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Register {


    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id            : Option<Uuid>,
    pub username             : String,
    pub email                : String,
    pub email_confirmed      : String,

    #[serde(default = "default_false")]
    pub email_verified       : bool,
    pub firstname            : String,
    pub lastname             : String,
    pub password             : String,
    pub password_confirm     : String,
    pub phone_number         : String,
    pub phone_number_confirm : String,

}

impl Register {

    //User field validations
    fn clean_phone_numbers(&mut self) {

        self.phone_number.retain(|c| c.is_digit(10));
    }

    //email is not verified
    fn email_unverified(&mut self) {

        self.email_verified = false;
    }

    //email_verification set email verification
    fn email_verified(&mut self){

        self.email_verified = true;
    }

    //validate email of users
    fn is_valid_email(&self) -> bool {

        let re = Regex::new(r"^[\w.-]+@[\w.-]+\.\w+$").unwrap();

        re.is_match(&self.email)
    }

    fn is_valid_username(&self) -> bool {
        let re = Regex::new(r"^[a-zA-Z0-9_\-]+$").unwrap();

        re.is_match(&self.username)
    }

    //validate first and last names of users
    fn is_valid_first_name(&self) ->bool {

        let re = Regex::new(r"^[A-Za-z]+(?:[-' ][A-Za-z]+)*$").unwrap();

        re.is_match(&self.firstname)
    }

    fn is_valid_last_name(&self) ->bool {

        let re = Regex::new(r"^[A-Za-z]+(?:[-' ][A-Za-z]+)*$").unwrap();

        re.is_match(&self.lastname)
    }

    fn is_valid_phone_number(&self) -> bool {

        let re = Regex::new(r"^\d{10}$").unwrap();

        re.is_match(&self.phone_number)
    }

    //generate hashed password
    pub fn generate_hashed_password(&self) -> Result<String, bcrypt::BcryptError> {

        hash(&self.password, DEFAULT_COST)

    }

    fn validate(self) -> HashMap<String, String>  {

        let mut errors = HashMap::new();


        if self.is_valid_username() {

            errors.insert("username".to_string(), "invalid username".to_string());

        }else if self.username.len() < 5 {

            errors.insert("username".to_string(), "username too short".to_string());

        }else if self.username.len() > 25 {

            errors.insert("username".to_string(), "username too long".to_string());

        }else if self.username.trim().is_empty() {

            errors.insert("username".to_string(), "required".to_string());
        }

        if !self.is_valid_first_name(){

            errors.insert("first_name_chars".to_string(), "first_name is not valid".to_string());

        }else if self.firstname.len() > 50 {

            errors.insert("first_name_length".to_string(), "first_name is too long".to_string());

        } else if !self.firstname.len() < 2 {

            errors.insert("first_name_length".to_string(), "first_name to short".to_string());

        }else if self.firstname.trim().is_empty() {
            errors.insert("first_name".to_string(), "required".to_string());
        }

        if !self.lastname.trim().is_empty() {

            errors.insert("last_name".to_string(), "required".to_string());

        }else if self.lastname.len() < 10 {

            errors.insert("last_name_length".to_string(), "last name is too short".to_string());

        }else if self.lastname.len() > 50 {

            errors.insert("last_name_length".to_string(), "last name is too long".to_string());
        }

        if !self.is_valid_phone_number() {

            errors.insert("phone_number".to_string(), "phone_number is not valid".to_string());

        }else if self.phone_number != self.phone_number_confirm {

            errors.insert("phone_number".to_string(), "does not match".to_string());

        }

        if !self.is_valid_email(){

            errors.insert("email".to_string(), "email is not valid".to_string());

        }else if self.email != self.email_confirmed {

            errors.insert("email".to_string(), "does not match".to_string());


        }

        if self.password != self.password_confirm {

            errors.insert("password".to_string(), "does not match".to_string());

        }else if self.password.len() < 10 {

            errors.insert("password_length".to_string(), "password is too short".to_string());

        }else if self.password.len() > 50{

            errors.insert("password_length".to_string(), "password is too long".to_string());

        }else if self.password.trim().is_empty() {

            errors.insert("password".to_string(), "required".to_string());
        }

        return errors;

    }
}