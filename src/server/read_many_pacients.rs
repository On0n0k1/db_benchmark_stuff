use crate::dao::paciente::Paciente;
use crate::Dao;
use crate::Error;
use actix_web::{get, web, HttpResponse, Responder};
use log::info;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct Query {
    pub offset: u32,
    pub quantity: u32,
}

#[get("/paciente")]
pub async fn read_many_pacients(
    dao: web::Data<Arc<Dao>>,
    query: web::Query<Query>,
) -> Result<impl Responder, Error> {
    let query: Query = query.into_inner();
    let offset: i32 = query.offset.try_into().or_else(Error::conversion)?;
    let quantity: i32 = query.quantity.try_into().or_else(Error::conversion)?;
    info!("Offset is: {offset}. Quantity is: {quantity}");

    let dao: Arc<Arc<Dao>> = dao.into_inner();
    let pacientes: Vec<Paciente> = Paciente::read_many(quantity, offset, dao.pool()).await?;
    // let result: HttpResponse = HttpResponse::Ok().json(pacientes);
    // Ok(result)
    Ok(HttpResponse::Ok().finish())
}
