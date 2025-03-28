use chrono::{Local, NaiveDateTime};
use crate::constants::{
    table_names::USERS_TABLE
};
use crate::db_statements::sql_statements::{
  insert,
};
use crate::models::user::dto::DTO;
use serde::{
    Serialize,
    Deserialize,
};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Default, Clone, FromRow, Serialize, Deserialize)]
pub struct User{


    pub id              : Option<i64>,
    pub public_id       : Option<Uuid>,
    pub username        : Option<String>,
    pub email           : Option<String>,
    pub firstname       : Option<String>,
    pub lastname        : Option<String>,
    pub password        : Option<String>,
    pub email_verified  : bool,
    pub email_key       : Option<String>,
    pub phone           : Option<String>,
    pub created_at      : Option<NaiveDateTime>,
    pub updated_at      : Option<NaiveDateTime>,

}



impl User {

    pub fn register_to_new(dto: DTO) -> Option<Self> {
        let current_time : NaiveDateTime = dto.current_time
            .unwrap_or_else(|| Local::now().naive_local());

        let public_id : Uuid = match &dto.public_id {

            Some(public_id) => public_id.to_owned(),
            None => return None,

        };

        Some(
            Self{
                id             : None,
                public_id      : dto.public_id,
                username       : dto.username,
                email          : dto.email,
                firstname      : dto.firstname,
                lastname       : dto.lastname,
                password       : dto.password,
                phone          : dto.phone_number,
                email_verified : dto.email_verified,
                email_key      : Some(String::new()),
                created_at     : Some(current_time),
                updated_at     : Some(current_time),
            }
        )
    }

    pub fn insert_new_user_query(&self) -> Option<String> {


        let insert_field : String = String::from(
            "public_id, username, firstname, lastname, email, password, email_verified, \
                email_key, password, created_at, updated_at"
        );

        let insert_value: String = String::from(
            "$1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11"
        );

        let return_values :String = String::from("id, email");


       Some(insert(USERS_TABLE.to_string(), insert_field, insert_value, return_values))
    }
}