use crate::dao::paciente::Paciente;
use crate::Dao;
use crate::Error;
use actix_web::{get, web, HttpResponse, Responder};
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
    if (query.offset > i32::MAX as u32) || (query.quantity > i32::MAX as u32) {
        return Ok(HttpResponse::BadRequest().body("Integer overflow"));
    }
    let offset: i32 = query.offset as i32;
    let quantity: i32 = query.quantity as i32;
    let dao: Arc<Arc<Dao>> = dao.into_inner();
    let pacientes: Vec<Paciente> = Paciente::read_many(quantity, offset, dao.pool()).await?;
    let result: HttpResponse = HttpResponse::Ok().json(pacientes);
    Ok(result)
}
