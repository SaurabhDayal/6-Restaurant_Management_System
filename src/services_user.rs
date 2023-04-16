use actix_web::{web};
use actix_web::{get, post, Responder};
use actix_web::{web::Data, HttpResponse};
use rand::distributions::{Alphanumeric, DistString};
use sqlx::{self};

use crate::error::MyError;
use crate::model::*;

// User register
#[post("/user/register")]
pub async fn register(state: Data<AppState>, user: web::Json<Users>) -> Result<impl Responder, MyError> {

    let user =user.into_inner();

    let row = sqlx::query_as!(Users,
        "INSERT INTO users (user_name, user_password, user_email) VALUES ($1, $2, $3) 
        RETURNING user_id, user_name, user_password, user_email",
        user.user_name, user.user_password, user.user_email
    )
    .fetch_one(&state.db)
    .await?;

    sqlx::query_as!( Roles,"INSERT INTO roles (role_type, user_id) VALUES($1, $2) 
    RETURNING role_id, role_type, user_id", "User".to_string(), row.user_id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(actix_web::web::Json(row))
}

// User login
#[post("/user/login")]
async fn login(state: Data<AppState>, user: web::Json<Users>) -> Result<impl Responder, MyError> {
    let user = user.into_inner();
    let table_user = sqlx::query_as!(Users, "select user_id, user_name, user_password, user_email from users where user_name =$1",
         user.user_name)
    .fetch_one(&state.db).await?;

    if user.user_password==table_user.user_password {
        let user_token = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let row = sqlx::query_as!(Auths,"Insert into auths (user_id, user_token) VALUES ($1, $2) 
        RETURNING user_id, user_token", table_user.user_id, user_token)
        .fetch_one(&state.db)
        .await?;
        Ok(actix_web::web::Json(row))                
    } else {
        Err(MyError::UnAuthorized)
    }

}

// User logout
#[post("/user/logout")]
async fn logout(state: Data<AppState>, user: web::Json<Users>, usr: Users) -> Result<impl Responder, MyError> {
    let user = user.into_inner();
    let b_id = usr.user_id;

    let table_user = sqlx::query_as!(Users, "select user_id, user_name, user_password, user_email from users where user_name =$1",
         user.user_name)
    .fetch_one(&state.db).await?;

    if table_user.user_id==b_id {
        let row = sqlx::query_as!(Auths,"DELETE FROM auths WHERE user_id=$1 RETURNING user_id, user_token", b_id)
        .fetch_one(&state.db)
        .await?;
        Ok(actix_web::web::Json(row))                
    } else {
        Err(MyError::UnAuthorized)
    }

}

// User get all restaurant list 
#[get("/user/restaurant")]
async fn get_restaurant_list(state: Data<AppState>, usr: Users) -> Result<impl Responder, MyError> {
    let b_id = usr.user_id;
    let role_row = sqlx::query_as!( Roles,"SELECT role_id, role_type, user_id FROM roles WHERE user_id=$1", b_id
    )
    .fetch_one(&state.db)
    .await?;

    if role_row.role_type=="Admin".to_string()|| role_row.role_type=="User".to_string()|| role_row.role_type=="SubAdmin".to_string(){
        let rows = sqlx::query_as!(Restaurants,
            "SELECT restaurant_id, restaurant_name, restaurant_address, user_id 
            FROM Restaurants"
        )
        .fetch_all(&state.db)
        .await?;
    Ok(HttpResponse::Ok().json(rows))
    } else {
        Err(MyError::UnAuthorized)
    }
    
    
}


// User get all dish list from a Particular restaurant
#[get("/user/dish/{res_id}")]
async fn get_dish_list(state: Data<AppState>, res_id: web::Path<i32>, usr: Users) -> Result<impl Responder, MyError> {
    let res_id=res_id.into_inner();
    let b_id = usr.user_id;
    let role_row = sqlx::query_as!( Roles,"SELECT role_id, role_type, user_id FROM roles WHERE user_id=$1", b_id
    )
    .fetch_one(&state.db)
    .await?;

    if role_row.role_type=="Admin".to_string()|| role_row.role_type=="User".to_string()|| role_row.role_type=="SubAdmin".to_string(){
        let rows = sqlx::query_as!( Dishes,
            "SELECT dish_id, dish_name, dish_cost, restaurant_id, user_id 
            FROM dishes WHERE restaurant_id=$1", res_id
        )
        .fetch_all(&state.db)
        .await?;
    Ok(HttpResponse::Ok().json(rows))
    } else {
        Err(MyError::UnAuthorized)
    }
    
}

// Add address to User OR Restaurant
#[post("/address")]
pub async fn add_address(state: Data<AppState>, adderess: web::Json<Addresses>, usr:Users) -> Result<impl Responder, MyError> {

    let a =adderess.into_inner();
    let b_id = usr.user_id;
    let role_row = sqlx::query_as!( Roles,"SELECT role_id, role_type, user_id FROM roles WHERE user_id=$1", b_id
    )
    .fetch_one(&state.db)
    .await?;

    if role_row.role_type=="Admin".to_string()|| role_row.role_type=="User".to_string()|| role_row.role_type=="SubAdmin".to_string(){
        let row = sqlx::query_as!(Addresses,
            "INSERT INTO addresses (address_name, address_lat, address_lng, user_id) VALUES ($1, $2, $3, $4) 
            RETURNING address_id, address_name, address_lat, address_lng, user_id",
            a.address_name, a.address_lat, a.address_lng, a.user_id
        )
        .fetch_one(&state.db)
        .await?;

        Ok(actix_web::web::Json(row))
    } else {
        Err(MyError::UnAuthorized)
    }
}


// Get distance from all user addresses to a particular restaurant address
#[get("/address/{res_id}")]
pub async fn get_distance(state: Data<AppState>, res_id: web::Path<i32>, usr:Users) -> Result<impl Responder, MyError> {
    
    let b_id = usr.user_id;

    let res_id=res_id.into_inner();
    let res_add= sqlx::query_as!( Addresses,
        "SELECT address_id, address_name, address_lat, address_lng, user_id FROM addresses WHERE address_id=$1", res_id         
    )
    .fetch_one(&state.db)
    .await?;

    let user_add_list = sqlx::query_as!( Addresses,
        "SELECT address_id, address_name, address_lat, address_lng, user_id FROM addresses WHERE user_id=$1", b_id         
    )
    .fetch_all(&state.db)
    .await?;

    let mut response = Vec::new();
    response.push(format!("Distance of restaurant with id {:?} with user with user_id {}, if any are: ", res_id, b_id ));

    for address in user_add_list {
        
        let dlong = res_add.address_lng  - address.address_lng;
        let dlat = res_add.address_lat - address.address_lat;

        let ans = (dlong * dlat) /2.0; 
        let R = 6371.00;
        let ans = ans * R;

        response.push(format!("Distance of restaurant with id {:?} with user with user_id {} is :   {} ", res_id, b_id, ans ));
    }

    Ok(actix_web::web::Json(response))
}

