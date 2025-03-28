use std::collections::HashMap;
use chrono::{Local, NaiveDateTime};
use crate::services::common_services::generate_uuid;
use crate::db_statements::{
    sql_statements::{
        exists_name,
    },

};
use sqlx::PgPool;
use uuid::Uuid;


fn us_states_lowercase() -> HashMap<&'static str, &'static str> {
    let mut states = HashMap::new();

    states.insert("al", "alabama");
    states.insert("ak", "alaska");
    states.insert("az", "arizona");
    states.insert("ar", "arkansas");
    states.insert("ca", "california");
    states.insert("co", "colorado");
    states.insert("ct", "connecticut");
    states.insert("de", "delaware");
    states.insert("fl", "florida");
    states.insert("ga", "georgia");
    states.insert("hi", "hawaii");
    states.insert("id", "idaho");
    states.insert("il", "illinois");
    states.insert("in", "indiana");
    states.insert("ia", "iowa");
    states.insert("ks", "kansas");
    states.insert("ky", "kentucky");
    states.insert("la", "louisiana");
    states.insert("me", "maine");
    states.insert("md", "maryland");
    states.insert("ma", "massachusetts");
    states.insert("mi", "michigan");
    states.insert("mn", "minnesota");
    states.insert("ms", "mississippi");
    states.insert("mo", "missouri");
    states.insert("mt", "montana");
    states.insert("ne", "nebraska");
    states.insert("nv", "nevada");
    states.insert("nh", "new hampshire");
    states.insert("nj", "new jersey");
    states.insert("nm", "new mexico");
    states.insert("ny", "new york");
    states.insert("nc", "north carolina");
    states.insert("nd", "north dakota");
    states.insert("oh", "ohio");
    states.insert("ok", "oklahoma");
    states.insert("or", "oregon");
    states.insert("pa", "pennsylvania");
    states.insert("ri", "rhode island");
    states.insert("sc", "south carolina");
    states.insert("sd", "south dakota");
    states.insert("tn", "tennessee");
    states.insert("tx", "texas");
    states.insert("ut", "utah");
    states.insert("vt", "vermont");
    states.insert("va", "virginia");
    states.insert("wa", "washington");
    states.insert("wv", "west virginia");
    states.insert("wi", "wisconsin");
    states.insert("wy", "wyoming");

    states
}
pub async fn create_us_state(db_pool : &PgPool) -> Result<(), sqlx::Error> {

    let states = us_states_lowercase();

    for (code, name) in states {


        let name_exists = exists_name(db_pool, name, "besecure_proj.us_states").await;

        if name_exists {

            //println!("use state {} already exists", name);
            continue;
        }

        let _ = insert_us_state(db_pool, code, name).await.expect("insert failed");
    }


    Ok(())
}


pub async fn insert_us_state(db_pool : &PgPool,
                             code    : &str,
                             name    : &str
) -> Result<Uuid, sqlx::Error> {

    let current_time : NaiveDateTime  = Local::now().naive_local();

    let public_id : Uuid = generate_uuid(db_pool, "besecure_proj.us_states")
        .await
        .expect("Failed to generate UUID");

    let current_time : NaiveDateTime  = Local::now().naive_local();

    sqlx::query("INSERT INTO besecure_proj.us_states (public_id, code, name, created_at,
                 updated_at) VALUES ($1, $2, $3, $4, $5)"
    )
        .bind(public_id)
        .bind(code)
        .bind(name)
        .bind(current_time)
        .bind(current_time)
        .execute(db_pool)
        .await?;

    Ok(public_id)

}