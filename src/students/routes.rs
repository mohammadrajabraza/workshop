use crate::students::{Student, Students};
use actix_web::{delete, get, post, put, web, HttpResponse};

// This route handler will list all the data available
#[get("/students")]
async fn find_all() -> HttpResponse {
    let students = Students::find_all();
    HttpResponse::Ok().body(format!("List of students"))
}

// This route handler will list data with specific id
#[get("/students/{id}")]
async fn find(id: web::Path<i32>) -> HttpResponse {
    let student = Students::find(id.into_inner());
    HttpResponse::Ok().body(format!("Fetched Record"))
}

// This route handler will create a new record
#[post("/students")]
async fn create(student : web::Json<Student>) -> HttpResponse {
    let student = Students::create(student.into_inner());
    HttpResponse::Ok().body(format!("Created record"))

}

// This route handler will update an existing record
#[put("/students/{id}")]
async fn update(id : web::Path<i32>, student : web::Json<Student>) -> HttpResponse {
    let student = Students::update(id.into_inner(), student.into_inner());
    HttpResponse::Ok().body(format!("Updated record"))

}

// This route handler will delete an specified record
#[delete("/students/{id}")]
async fn delete(id: web::Path<i32>) -> HttpResponse {
    let student = Students::delete(id.into_inner());
    HttpResponse::Ok().body(format!("Deleted record"))
}

// this function will initialize all the route handlers
pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}