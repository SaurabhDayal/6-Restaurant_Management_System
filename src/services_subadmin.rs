use actix_web::{web};
use actix_web::{get, post, Responder};
use actix_web::{web::Data, HttpResponse};
use sqlx::{self};

use crate::error::MyError;
use crate::model::*;


// Create Restaurant
#[post("/restaurant")]
async fn create_restaurant(state: Data<AppState>, restaurant: web::Json<Restaurants>, usr:Users) -> Result<impl Responder, MyError> {
    let r = restaurant.into_inner();
    let b_id = usr.user_id;

    let role_row = sqlx::query_as!( Roles,"SELECT role_id, role_type, user_id FROM roles WHERE user_id=$1", b_id
    )
    .fetch_one(&state.db)
    .await?;

    if role_row.role_type =="Admin".to_string()||role_row.role_type =="SubAdmin".to_string() {
        let row = sqlx::query_as!( Restaurants,
            "INSERT INTO restaurants (restaurant_name, restaurant_address, user_id) VALUES ($1, $2, $3) 
            RETURNING restaurant_id, restaurant_name, restaurant_address, user_id",
            r.restaurant_name, r.restaurant_address, b_id
        )
        .fetch_one(&state.db)
        .await?;
    Ok(HttpResponse::Ok().json(row))
    } else{
        Err(MyError::UnAuthorized)
    }
    
}

// Create Dishes
#[post("/dish")]
async fn create_dish(state: Data<AppState>, dish: web::Json<Dishes>, usr:Users) -> Result<impl Responder, MyError> {
    let d = dish.into_inner();
    let b_id = usr.user_id;

    let role_row = sqlx::query_as!( Roles,"SELECT role_id, role_type, user_id FROM roles WHERE user_id=$1", b_id
    )
    .fetch_one(&state.db)
    .await?;

    if role_row.role_type =="Admin".to_string()||role_row.role_type =="SubAdmin".to_string() {
        let row = sqlx::query_as!( Dishes,
            "INSERT INTO dishes (dish_name, dish_cost, restaurant_id, user_id) VALUES ($1, $2, $3, $4) 
            RETURNING dish_id, dish_name, dish_cost, restaurant_id, user_id",
            d.dish_name, d.dish_cost, d.restaurant_id, b_id
        )
        .fetch_one(&state.db)
        .await?;
    Ok(HttpResponse::Ok().json(row))
    } else{
        Err(MyError::UnAuthorized)
    }
    
}


// Admin list all restaurant 
// Subadmin list his/her restaurants
#[get("/restaurant")]
async fn get_restaurant_by_user_id(state: Data<AppState>, usr: Users) -> Result<impl Responder, MyError> {
    let b_id = usr.user_id;
    let role_row = sqlx::query_as!( Roles,"SELECT role_id, role_type, user_id FROM roles WHERE user_id=$1", b_id
    )
    .fetch_one(&state.db)
    .await?;

    if role_row.role_type=="Admin".to_string(){
        let rows = sqlx::query_as!(Restaurants,
            "SELECT restaurant_id, restaurant_name, restaurant_address, user_id 
            FROM Restaurants"
        )
        .fetch_all(&state.db)
        .await?;
        Ok(HttpResponse::Ok().json(rows))
    } else if role_row.role_type=="SubAdmin".to_string() {
        let rows = sqlx::query_as!(Restaurants,
            "SELECT restaurant_id, restaurant_name, restaurant_address, user_id 
            FROM Restaurants WHERE user_id=$1", b_id
        )
        .fetch_all(&state.db)
        .await?;
        Ok(HttpResponse::Ok().json(rows))
    } else {
        Err(MyError::UnAuthorized)
    }

}


// Admin list all dishes
// Subadmin list his/her dishes
#[get("/dish")]
async fn get_dish_by_user_id(state: Data<AppState>, usr: Users) -> Result<impl Responder, MyError> {
    let b_id = usr.user_id;
    let role_row = sqlx::query_as!( Roles,"SELECT role_id, role_type, user_id FROM roles WHERE user_id=$1", b_id
    )
    .fetch_one(&state.db)
    .await?;

    if role_row.role_type=="Admin".to_string(){
        let rows = sqlx::query_as!(Dishes,
            "SELECT dish_id, dish_name, dish_cost, restaurant_id, user_id 
            FROM dishes"
        )
        .fetch_all(&state.db)
        .await?;
        Ok(HttpResponse::Ok().json(rows))
    } else if role_row.role_type=="SubAdmin".to_string() {
        let rows = sqlx::query_as!(Dishes,
            "SELECT dish_id, dish_name, dish_cost, restaurant_id, user_id 
            FROM dishes WHERE user_id=$1", b_id
        )
        .fetch_all(&state.db)
        .await?;
        Ok(HttpResponse::Ok().json(rows))
    } else {
        Err(MyError::UnAuthorized)
    }

}




// Get all users 
#[get("/users")]
async fn get_users_list(state: Data<AppState>, usr:Users) -> Result<impl Responder, MyError> {

    let b_id = usr.user_id;

    let role_row = sqlx::query_as!( Roles,"SELECT role_id, role_type, user_id FROM roles WHERE user_id=$1", b_id
    )
    .fetch_one(&state.db)
    .await?;

    if role_row.role_type =="Admin".to_string()||role_row.role_type =="Admin".to_string() {
        let row = sqlx::query_as!( Users,
            "SELECT user_id, user_name, user_password, user_email FROM users"
        )
        .fetch_all(&state.db)
        .await?;
        Ok(HttpResponse::Ok().json(row))
    } else{
        Err(MyError::UnAuthorized)
    }
    
}



