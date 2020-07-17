// Necessary imports here 
use actix_web::{App, HttpServer};

// imports for de/serializing objects
use serde::{Deserialize};

// importing student module(s)
mod students;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Initializing server
    HttpServer::new(|| {
        App::new()
        // Associating service(s)/route_handler(s)
         .configure(students::routes::init_routes)
    })
    // Binding socket address server will receive requests on
    .bind("127.0.0.1:5000")?
    .run()
    .await
}

// -------------------------- Models -----------------------------

// This struct will be used for taking input from user's request
#[derive(Deserialize)]
struct Student {
    first_name: String,
    last_name: String,
    department: String,
    is_graduated: bool,
    age: i32,
}

// This struct will be used for retrieval from data sources
#[derive(Debug, Clone)]
struct Students {
    id: i32,
    first_name: String,
    last_name: String,
    department: String,
    is_graduated: bool,
    age: i32,
}

impl Students {
    // Constructor function to get a new Student object
    fn new(id : i32, first_name : String, last_name : String, department : String,
         is_graduated : bool, age : i32) -> Students {
             Students {
                id,
                first_name,
                last_name,
                department,
                is_graduated,
                age 
            }
    }

    // Function which transform Student object to Students object
    fn from(student : Student) {
    }

    // Function that will create an Student object and insert it into data store
    fn create(student: Student) {
    }

    // Function that will list all the data in the data store
    fn find_all() {
    }

    // Function that will show the record with specified id
    fn find(id: i32) {
    }

    // Function that will update an existing record
    fn update(id: i32, student: Student) {
    }

    // Function that will delete an specified object
    fn delete(id: i32) {
    }
}
// ------------------------ End Models ---------------------------
