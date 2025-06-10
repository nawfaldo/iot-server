use actix_web::{get, HttpResponse, Responder};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::db::establish_connection;
use crate::models::cluster::{ClusterData, Column, Entity as Cluster};
use crate::{errors::CustomError, utils::Auth};

#[get("/cluster")]
pub async fn get_clusters(auth: Auth) -> Result<impl Responder, CustomError> {
    let db = establish_connection().await?;

    let clusters: Vec<ClusterData> = Cluster::find()
        .filter(Column::UserId.eq(auth.user_id))
        .all(&db)
        .await?
        .into_iter()
        .map(|cluster| ClusterData {
            id: cluster.id,
            name: cluster.name,
            chip_serial: cluster.chip_serial,
        })
        .collect();

    Ok(HttpResponse::Ok().json(clusters))
}
