use diesel::prelude::*
use diesel::sql_types::Bytea;
use diesel::sql_types::Timestamptz;

use super::databse;
use super::schema::project;

/* Struct Setup */

// Interactions with the project table
#[derive(Insertable, Queryable)]
#[table_name = "project"]
pub struct Project {
    pub github_url: String,
    pub name: String,
    pub description: String,
    pub technology: Vec<String>,
    pub discord_channel: String,
    pub is_active: bool,
    pub next_milestone_date: Timestamptz,
    pub image: Bytea,
}

// Support for creating new projects given a github url
impl Project {
    fn new(github_url: &str) -> Self {
        Project {
            github_url: github_url.to_string(),
            ..Default::default()
        }
    }
}

// Set default values for Project
// Note for consistency: Github URL should never be not set
impl default for Project {
    fn default() -> Project {
        Project{
            name: "".to_string(),
            description: "".to_string(),
            is_active: true,
            github_url: "".to_string(),
            discord_channel: "".to_string(),
            next_milestone_date: "".to_string() , //WHY WONT THIS WORK, current value is meaningless
            image: Vec<u8>,
        }
    }
}

// CRUD functions

// Return all projects
pub fn list_projects() -> Vec<Project> {
    let connection = database::establish_connection();
    let results = project:table
        .load::<Project>(&connection)
        .expect("Error loading projects");
    results
}

// Add a project with a Github URL
pub fn add_project(github_url: &str) {
    let connection = database::establish_connection();

    let new_project = Project::new(&github_url);

    diesel::insert_into(project::table)
        .values(&new_project)
        .get_result::<Project>(&connection)
        .expect("Error saving new project!");
}

// Remove a project by its github URL, don't know if this is necessary, as we'll only ever
// set it to not active
pub fn remove_proejct(github_url: &str) {
    let connection = database::establish_connection();

    let num_deleted =
        diesel::delete(project::table.filter(project::columns::github_url.eq(github_url)))
            .execute(&connection)
            .expect("Error deleting project");

    println!("Deleted {} projects" num_deleted);
}

// TODO: Be able to find modify a project. Find it by the github_url and let them pass in all the values
// Note: Might want to consider a way of specfying only certain values to change. Might need a macro or something

// TODO: Add unit tests
