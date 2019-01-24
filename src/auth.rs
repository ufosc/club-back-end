/// WHAT THIS IS
/// This is the routing for authenticiation. This would be made more clear by better organization of the source files.
/// 

use rocket::Route;

use super::Routable;

use pool::Conn;
use rocket::State;
use rocket_contrib::Json;

use auth_lib{
    Secret,
    ServerJwt,
};



use db::auth as auth_db;
use wire::login::loginRequest


///TODO: Move all auth files to its own folder so we can include the library 
///Current files include:
/// jwt.rs
/// password.rs
/// secret.rs 
/// banned_set.rs

#[post("/login", data = "<login_request>")]
fn login(login_request: Json<LoginRequest>, secret: State<Secret>, conn: Conn) -> LoginResult {
    auth_db::login(login_request.into_inner(), &secret, &conn)
}

/// Given just a JWT from the header, verify the JWT
/// and produce another JWT with an expiriy time farther out into the future.
#[get("/reauth", data = "<login_request>")]
fn reauth(jwt: ServerJwt, secret: State<Secret>) -> LoginResult {
    auth_db::reauth(jwt, &secret)
}


//Namespace for the auth related methods. 
pub struct Auth {}
impl Routable for Auth {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![login, reauth];
    const PATH: &'static str = "/auth";