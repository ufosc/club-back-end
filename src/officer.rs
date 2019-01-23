#![allow(dead_code)]

use diesel::prelude::*;

use super::database;
use super::schema::officer;

use chrono::NaiveDateTime;

/* Setting the roles enum first for tidiness */ 

enum Roles{
    president, 
    vicePresident,
    treasurer,
    secretary,
    externalRelationsChair,
    socialChair,
    projectLead,
}


/* Struct Setup */ 

//Struct for interacting with officer table
#[derive(Insertable, Queryable)]
#[table_name = "officer"]
pub struct Officer { 
    pub ufl_username: String, 
    pub role: Roles,
    pub password: 
}