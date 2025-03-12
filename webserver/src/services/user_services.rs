use actix_web::web::Data;
use models::User;
use mongodb::{Collection, Database};


pub fn get_user_collection(db: Data<Arc<Database>> ) -> Collection<User::User> {
    db.collection("users")
}