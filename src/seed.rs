use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Set};

use crate::models::cluster::{ActiveModel as ClusterActiveModel, Entity as Cluster};
use crate::models::user::{ActiveModel as UserActiveModel, Entity as User};
use crate::utils::hash_password;

pub async fn seed_database(db: &DatabaseConnection) -> Result<(), DbErr> {
    println!("Seeding Database...");

    // user
    let hashed_password1 = hash_password("1234").map_err(|e| DbErr::Custom(e.to_string()))?;
    let first_user = UserActiveModel {
        username: Set("November Rain".to_owned()),
        password: Set(hashed_password1),
        ..Default::default()
    };
    let user_result = User::insert(first_user).exec(db).await?;

    // cluster
    let user_id = user_result.last_insert_id;
    let first_user_first_cluster = ClusterActiveModel {
        name: Set("I Cant Stop The Loneliness".to_owned()),
        user_id: Set(user_id),
        ..Default::default()
    };
    Cluster::insert(first_user_first_cluster).exec(db).await?;
    let first_user_second_cluster = ClusterActiveModel {
        name: Set("A Hope From Sad Street".to_owned()),
        user_id: Set(user_id),
        ..Default::default()
    };
    Cluster::insert(first_user_second_cluster).exec(db).await?;

    println!("Seeded");

    Ok(())
}
