use bcrypt::{hash, DEFAULT_COST};
use std::collections::{HashMap};
use chrono::NaiveDateTime;
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
pub struct DTO {


    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id            : Option<Uuid>,
    pub username             : Option<String>,
    pub email                : Option<String>,
    #[serde(default = "default_false")]
    pub email_verified       : bool,
    pub firstname            : Option<String>,
    pub lastname             : Option<String>,
    pub password             : Option<String>,
    pub hash_password        : Option<String>,
    pub password_confirm     : Option<String>,
    pub phone_number         : Option<String>,
    pub current_time         : Option<NaiveDateTime>

}

impl DTO {

    //User field validations
    fn clean_phone_number(&mut self) {

        if let Some(phone_number) = &mut self.phone_number {
            phone_number.retain(|c| c.is_ascii_digit());
        }
    }

    //email is not verified
    fn email_unverified(&mut self) {

        self.email_verified = false;
    }

    //email_verification set email verification
    fn email_verified(&mut self){

        self.email_verified = true;
    }


    //generate hashed password
    pub fn generate_hashed_password(&mut self) -> Result<(), bcrypt::BcryptError> {

        if let Some(password) = &self.password{

            match hash(password, DEFAULT_COST) {
                Ok(hash) => {
                    self.password = Some(hash);
                }
                Err(e) => return Err(e)
            }

        }else{

            return Err(bcrypt::BcryptError::InvalidHash("".to_string()))
        }

        Ok(())

    }

    pub fn get_current_time(&self) -> Option<&NaiveDateTime> {
        self.current_time.as_ref()
    }
    pub fn get_hashed_password(&self) -> Option<&str> {

        self.hash_password.as_deref()
    }
    pub fn get_public_id(&self) -> Option<&Uuid> {

        self.public_id.as_ref()

    }

    pub fn set_current_time(&mut self, current_time: NaiveDateTime) {

        self.current_time = Some(current_time);
    }

    pub fn set_hashed_password(&mut self, hashed_password: String) {

        self.hash_password = Some(hashed_password);
    }
    pub fn set_public_id(&mut self, public_id: Uuid) {
        self.public_id = Some(public_id)
    }

    pub fn validate(&mut self, errors: &mut HashMap<String, String>) {


        self.validate_username(errors);
        self.validate_firstname(errors);
        self.validate_lastname(errors);
        self.validate_phone_number(errors);
        self.validate_email(errors);
        self.validate_password(errors);

    }

    pub fn validate_email(&self,
                          errors: &mut HashMap<String, String>){


        let re = match Regex::new(r"^[\w.-]+@[\w.-]+\.\w+$"){

            Ok(re) => re,
            Err(_) => {

                errors.insert("email".to_string(), "required".to_string());
                return
            }
        };

        if let Some(email) = &self.email{

            if email.trim().is_empty() {
                errors.insert("email".to_string(), "required".to_string());
            }
            else if !re.is_match(email) {
                errors.insert("email".to_string(), "invalid".to_string());
            }

        }
        else{

            errors.insert("email".to_string(), "required".to_string());

        }

    }
    pub fn validate_firstname(&self,
                              errors: &mut HashMap<String, String>
    ){

        let name_reg = match Regex::new(r"^[A-Za-z]+(?:[-' ][A-Za-z]+)*$"){

            Ok(re) => re,
            Err(_) => {

                errors.insert("firstname".to_string(), "required".to_string());
                return
            }
        };


        if let Some(firstname) =  &self.firstname{

            if firstname.trim().is_empty() {

                errors.insert("firstname".to_string(), "required".to_string());
            }
            else if !name_reg.is_match(firstname){

                errors.insert("firstname".to_string(), "first name invalid".to_string());
            }
            else if firstname.len() > 50 {

                errors.insert("firstname_length".to_string(), "first_name is too long".to_string());
            }

            else if firstname.trim().len() < 2 {

                errors.insert("firstname_length".to_string(), "first_name to short".to_string());

            }

        }
        else {
            errors.insert("firstname".to_string(), "required".to_string());
        }

    }

    pub fn validate_lastname(&self,
                             errors: &mut HashMap<String, String>
    ){

        let name_reg = match Regex::new(r"^[A-Za-z]+(?:[-' ][A-Za-z]+)*$"){

            Ok(re) => re,
            Err(_) => {

                errors.insert("lastname".to_string(), "required".to_string());

                return
            }
        };


        if let Some(lastname) =  &self.lastname{

            if lastname.trim().is_empty() {

                errors.insert("last_name".to_string(), "required".to_string());
            }
            else if !name_reg.is_match(lastname){
                errors.insert("last_name".to_string(), "last name invalid".to_string());
            }
            else if lastname.trim().len() < 2 {

                errors.insert("last_name_length".to_string(), "last name is too short".to_string());

            }else if lastname.trim().len() > 50 {

                errors.insert("last_name_length".to_string(), "last name is too long".to_string());
            }
        }
        else{
            errors.insert("lastname".to_string(), "required".to_string());
        }
    }

    pub fn validate_password(&self,
                             errors: &mut HashMap<String, String>
    ) {

        let re = match Regex::new(r#"['"<>;\\\x00\n\r]"#){
            Ok(re) => re,
            Err(_) =>{
                errors.insert("password".to_string(), "required".to_string());
                return
            }
        };

        if let Some((pw, cpw)) = self.password.as_ref().zip(
            self.password_confirm.as_ref()
        ){
            if pw != cpw {
                errors.insert("password_match".to_string(), "passwords do not match".to_string());
            }

            if re.is_match(pw) {

                errors.insert("password_invalid_char".to_string(), "password invalid".to_string());

            }else if pw.trim().len() > 50 {

                    errors.insert("password_length".to_string(), "password to long".to_string());

            }else if pw.trim().len() < 10 {

                errors.insert("password_length".to_string(), "password to short".to_string());
            }

        }else{
            errors.insert("password".to_string(), "required".to_string());
        }
    }

    pub fn validate_phone_number(&mut self,
                                 errors: &mut HashMap<String, String>
    ){


        self.clean_phone_number();


        let re = match Regex::new(r"^\d{10}$"){
            Ok(re) => re,
            Err(_) =>{
                errors.insert("phone_number".to_string(), "required".to_string());
                return
            }
        };



        if let Some(phone_number) = &self.phone_number{

            if phone_number.trim().is_empty() {
                errors.insert("phone_number".to_string(), "required".to_string());
            }
            else if !re.is_match(phone_number) {

                errors.insert("phone_number".to_string(), "phone_number is not valid".to_string());

            }
        }


    }

    pub fn validate_username(&self,
                             errors: &mut HashMap<String, String>
    ){

        let username_reg = match Regex::new(r"^[a-zA-Z0-9]+$"){
            Ok(re) => re,
            Err(_) =>{
                errors.insert("username".to_string(), "required".to_string());
                return
            }
        };

        if let Some(username) = &self.username {

            if username.trim().is_empty() {

                errors.insert("username".to_string(), "required".to_string());
            }
            else if !username_reg.is_match(username) {

                errors.insert(String::from("username"), String::from("username invalid"));
            }
            else if username.len() < 4 {

                errors.insert("username".to_string(), "username too short".to_string());

            }
            else if username.len() > 25 {

                errors.insert("username".to_string(), "username too long".to_string());

            }

        }else{

            errors.insert("username".to_string(), "required".to_string());
        }

    }

}