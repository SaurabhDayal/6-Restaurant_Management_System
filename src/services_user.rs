use actix_web::{web};
use actix_web::{get, post, Responder};
use actix_web::{web::Data, HttpResponse};
use rand::distributions::{Alphanumeric, DistString};
use sqlx;

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
async fn logout(state: Data<AppState>, user: web::Json<Users>, usr: UserAuth) -> Result<impl Responder, MyError> {
    let user = user.into_inner();
    let b_id = usr.user_id;

    let table_user = sqlx::query_as!(Users, "select user_id, user_name, user_password, user_email from users 
        WHERE user_name =$1",
        user.user_name)
        .fetch_one(&state.db)
        .await?;

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
async fn get_restaurant_list(state: Data<AppState>, usr: UserAuth) -> Result<impl Responder, MyError> {

    let rows = sqlx::query_as!(Restaurants,
        "SELECT restaurant_id, restaurant_name, restaurant_address, user_id 
        FROM Restaurants"
        )
        .fetch_all(&state.db)
        .await?;

    Ok(HttpResponse::Ok().json(rows))

}


// User get all dish list from a Particular restaurant
#[get("/user/dish/{res_id}")]
async fn get_dish_list(state: Data<AppState>, res_id: web::Path<i32>, usr: UserAuth) -> Result<impl Responder, MyError> {
    let res_id=res_id.into_inner();
 
    let rows = sqlx::query_as!( Dishes,
            "SELECT dish_id, dish_name, dish_cost, restaurant_id, user_id 
            FROM dishes WHERE restaurant_id=$1", res_id
        )
        .fetch_all(&state.db)
        .await?;

    Ok(HttpResponse::Ok().json(rows))
    
}

// Add address to User OR Restaurant
#[post("/address")]
pub async fn add_address(state: Data<AppState>, adderess: web::Json<Addresses>, usr:UserAuth) -> Result<impl Responder, MyError> {

    let a =adderess.into_inner();
    
    let row = sqlx::query_as!(Addresses,
            "INSERT INTO addresses (address_name, address_lat, address_lng, user_id) VALUES ($1, $2, $3, $4) 
            RETURNING address_id, address_name, address_lat, address_lng, user_id",
            a.address_name, a.address_lat, a.address_lng, a.user_id
        )
        .fetch_one(&state.db)
        .await?;

    Ok(actix_web::web::Json(row))

}


// Get distance from all user addresses to a particular restaurant address
#[get("/address/{res_id}/{user_add_id}")]
pub async fn get_distance(state: Data<AppState>, path: web::Path<(i32,i32)>, usr:UserAuth) -> Result<impl Responder, MyError> {
    
    let (res_id,user_add_id)=path.into_inner();

    let user_add=sqlx::query_as!( Addresses, 
        "SELECT address_id, address_name, address_lat, address_lng, user_id FROM addresses WHERE address_id=$1", user_add_id         
    )
    .fetch_one(&state.db)
    .await?;

    let res_add= sqlx::query_as!( Addresses,
        "SELECT address_id, address_name, address_lat, address_lng, user_id FROM addresses WHERE address_id=$1", res_id         
    )
    .fetch_one(&state.db)
    .await?;


    let dlong = res_add.address_lng  - user_add.address_lng;
    let dlat = res_add.address_lat - user_add.address_lat;

    let ans = (dlong * dlat) /2.0; 
    let ans = ans * 6.371;

    let distance =format!("Distance of Restaurant-{} with User Address-{} is :   {} ", 
        res_add.address_name, usr.user_name, ans);

    let add_dist = AddressDistance {
        distance : distance
    };

    Ok(actix_web::web::Json(add_dist))

}

