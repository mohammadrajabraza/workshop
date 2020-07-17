use crate::schema::students;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// This struct will be used for taking input from user's request
#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "students"]
pub struct Student {
    first_name: String,
    last_name: String,
    department: String,
    is_graduated: bool,
    age: i32,
}

// This struct will be used for retrieval from data sources
#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "students"]
pub struct Students {
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
    pub fn create(student: Student) {
    }

    // Function that will list all the data in the data store
    pub fn find_all() {
    }

    // Function that will show the record with specified id
    pub fn find(id: i32) {
    }

    // Function that will update an existing record
    pub fn update(id: i32, student: Student) {
    }

    // Function that will delete an specified object
    pub fn delete(id: i32) {
    }
}