use crate::dao::paciente::Paciente;
use crate::Dao;
use crate::Error;
use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Arc;

#[get("/paciente/{id}")]
pub async fn read_paciente(
    dao: web::Data<Arc<Dao>>,
    id: web::Path<u32>,
) -> Result<impl Responder, Error> {
    let id: i32 = id.into_inner() as i32;
    let dao: Arc<Arc<Dao>> = dao.into_inner();
    let paciente: Paciente = Paciente::read(id, dao.pool()).await?;
    let result: HttpResponse = HttpResponse::Ok().json(paciente);
    Ok(result)
}
