use sqlx::PgPool;
use crate::services::{
    plan_services,
    role_services,
    address_services,
};


pub async fn init_services(pg_pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {

    role_services::create_roles(pg_pool).await.expect("Error planning services");


    println!("Created role services");

    plan_services::create_plans(pg_pool).await.expect("Error planning services");

    println!("Created plan services");

    address_services::create_us_state(pg_pool).await.expect("Error planning services");

    println!("Created us states");

    Ok(())

}