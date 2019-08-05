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

fn replace_project(github_url: &str, modifier: &Project) {
	let connection = database::establish_connection();

	let update_result = diesel::update(projects::table)
		.set(modifier)
		.get_result::<Project>(&connection);
}


fn modify_github_url(projectURL:&str, string_replace: &str)
{
let connection = database::establish_connection();

	let temp_project = projects::table
		.filter(projects::projectURL.eq(&str))
		.load::<Project>(&connection);



	let update_result = diesel::update(temp_project)
		.set(projects::github_url.eq(string_replace))
		.get_result::<Project>(&connection);

}


fn modify_description(projectURL: &str, string_replace: &str)
{

	let connection = database::establish_connection();


	let temp_project = projects::table
		.filter(projects::projectURL.eq(&str))
		.load::<Project>(&connection);

	let modifier = Project {
		description: string_replace.to_string(),
		..temp_event.unwrap()[0].clone()
	};

	replace_project(&projectURL, &modifier);
}

//fn modify_technology()
//{
//	
//}

fn modify_name(projectURL: &str, string_replace: &str)
{

	let connection = database::establish_connection();


	let temp_project = projects::table
		.filter(projects::projectURL.eq(&str))
		.load::<Project>(&connection);

	let modifier = Project {
		name: string_replace.to_string(),
		..temp_event.unwrap()[0].clone()
	};

	replace_project(&projectURL, &modifier);
}


fn modify_discord_channel(projectURL: &str, string_replace: &str)
{

	let connection = database::establish_connection();

	let temp_project = projects::table
		.filter(projects::projectURL.eq(&str))
		.load::<Project>(&connection);

	let modifier = Project {
		discord_channel: string_replace.to_string(),
		..temp_event.unwrap()[0].clone()
	};

	replace_project(&projectURL, &modifier);
}

fn modify_is_active(projectURL: &str, string_replace: &str)
{
	let connection = database::establish_connection();


	let temp_project = projects::table
		.filter(projects::projectURL.eq(&str))
		.load::<Project>(&connection);

	let modifier = Project {
		is_active: bool_replace.clone(),
		..temp_event.unwrap()[0].clone()
	};

	replace_project(&projectURL, &modifier);
}

fn modify_next_milestone_date(projectURL: &str, milestone_replace: Timestamptz)
{
	let connection = database::establish_connection();


	let temp_project = projects::table
		.filter(projects::projectURL.eq(&str))
		.load::<Project>(&connection);

	let modifier = Project {
		next_milestone_date: milestone_replace.clone(),
		..temp_event.unwrap()[0].clone()
	};

	replace_project(&projectURL, &modifier);
}

fn modify_image(projectURL: &str, image_replace: Bytea)
{
	let connection = database::establish_connection();


	let temp_project = projects::table
		.filter(projects::projectURL.eq(&str))
		.load::<Project>(&connection);

	let modifier = Project {
		image: image_replace.clone(),
		..temp_event.unwrap()[0].clone()
	};

	replace_project(&projectURL, &modifier);
}



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
pub fn remove_project(github_url: &str) {
    let connection = database::establish_connection();

    let num_deleted =
        diesel::delete(project::table.filter(project::columns::github_url.eq(github_url)))
            .execute(&connection)
            .expect("Error deleting project");

    println!("Deleted {} projects" num_deleted);
}

#[cfg(test)]
mod tests{

use super::*;

#[test]
fn test_modify_name() {	
	clear_table();
	let connection = database::establish_connection();

        add_project("testproject");
        

        modify_name("testproject", "changed");

	let result = projects::table
		.filter(projects::github_url.eq("testproject"))
		.load::<Project>(&connection);
       assert_eq!("changed", result.unwrap()[0].name);
}

#[test]
fn test_modify_description() {	
	clear_table();
	let connection = database::establish_connection();

        add_project("testproject");
        

        modify_description("testproject", "changed");

	let result = projects::table
		.filter(projects::github_url.eq("testproject"))
		.load::<Project>(&connection);
       assert_eq!("changed", result.unwrap()[0].description);
}

fn test_modify_discord_channel() {	
	clear_table();
	let connection = database::establish_connection();

        add_project("testproject");
        

        modify_discord_channel("testproject", "changed");
	let result = projects::table
		.filter(projects::github_url.eq("testproject"))
		.load::<Project>(&connection);
       assert_eq!("changed", result.unwrap()[0].discord_channel);

	
}

#[test]
fn test_modify_is_active() {	
	clear_table();
		let connection = database::establish_connection();

        add_project("testproject");
	modify_is_active("testproject", &true);

        let result = projects::table
		.filter(projects::github_url.eq("testproject"))
		.load::<Project>(&connection);
     
   assert_eq!(true, result.unwrap()[0].is_active);


}

#[test]
fn test_modify_github_url() {	
	clear_table();
		let connection = database::establish_connection();

        add_project("testproject");
	modify_github_url("testproject", "changed");

        let result = projects::table
		.filter(projects::github_url.eq("changed"))
		.load::<Project>(&connection);
     
   assert_eq!("changed", result.unwrap()[0].github_url);


}
}



