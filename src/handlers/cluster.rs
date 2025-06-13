use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, ActiveModelTrait, Set};
use serde::Deserialize;

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

#[derive(Deserialize)]
pub struct CreateCluster {
    pub name: String,
    pub chip_serial: i32,
}

#[post("/cluster")]
pub async fn create_cluster(
    auth: Auth,
    payload: web::Json<CreateCluster>,
) -> Result<impl Responder, CustomError> {
    let db = establish_connection().await?;

    let new_cluster = crate::models::cluster::ActiveModel {
        name: Set(payload.name.clone()),
        chip_serial: Set(payload.chip_serial),
        user_id: Set(auth.user_id),
        ..Default::default()
    };

    let cluster = new_cluster.insert(&db).await?;

    Ok(HttpResponse::Ok().json(ClusterData {
        id: cluster.id,
        name: cluster.name,
        chip_serial: cluster.chip_serial,
    }))
}

#[derive(Deserialize)]
pub struct UpdateCluster {
    pub name: Option<String>,
    pub chip_serial: Option<i32>,
}

#[put("/cluster/{id}")]
pub async fn edit_cluster(
    auth: Auth,
    path: web::Path<i32>,
    payload: web::Json<UpdateCluster>,
) -> Result<impl Responder, CustomError> {
    let db = establish_connection().await?;
    let cluster_id = path.into_inner();

    // Find the cluster and ensure it belongs to the user
    let cluster = Cluster::find()
        .filter(Column::Id.eq(cluster_id))
        .filter(Column::UserId.eq(auth.user_id))
        .one(&db)
        .await?;

    let Some(cluster) = cluster else {
        return Err(CustomError::Other("Not found".to_string()));
    };

    let mut active = crate::models::cluster::ActiveModel {
        id: Set(cluster.id),
        name: Set(cluster.name),
        chip_serial: Set(cluster.chip_serial),
        user_id: Set(cluster.user_id),
        created_at: Set(cluster.created_at),
        updated_at: Set(cluster.updated_at),
    };
    if let Some(name) = &payload.name {
        active.name = Set(name.clone());
    }
    if let Some(chip_serial) = payload.chip_serial {
        active.chip_serial = Set(chip_serial);
    }
    let updated = active.update(&db).await?;

    Ok(HttpResponse::Ok().json(ClusterData {
        id: updated.id,
        name: updated.name,
        chip_serial: updated.chip_serial,
    }))
}

#[delete("/cluster/{id}")]
pub async fn delete_cluster(
    auth: Auth,
    path: web::Path<i32>,
) -> Result<impl Responder, CustomError> {
    let db = establish_connection().await?;
    let cluster_id = path.into_inner();

    // Find the cluster and ensure it belongs to the user
    let cluster = Cluster::find()
        .filter(Column::Id.eq(cluster_id))
        .filter(Column::UserId.eq(auth.user_id))
        .one(&db)
        .await?;

    let Some(cluster) = cluster else {
        return Err(CustomError::Other("Not found".to_string()));
    };

    let active = crate::models::cluster::ActiveModel {
        id: Set(cluster.id),
        ..Default::default()
    };
    active.delete(&db).await?;

    Ok(HttpResponse::Ok().json("Cluster deleted"))
}
