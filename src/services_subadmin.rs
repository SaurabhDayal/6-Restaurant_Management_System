use actix_web::{web};
use actix_web::{get, post, Responder};
use actix_web::{web::Data, HttpResponse};
use sqlx;

use crate::error::MyError;
use crate::model::*;


// Create Restaurant
#[post("/restaurant")]
async fn create_restaurant(state: Data<AppState>, restaurant: web::Json<Restaurants>, usr:UserAuth) -> Result<impl Responder, MyError> {
    let res = restaurant.into_inner();

    let role_row = usr.roles;

    if role_row.iter().any(|x| {
        x.role_type == "Admin"|| x.role_type == "SubAdmin"
    })  {
        let row = sqlx::query_as!( Restaurants,
            "INSERT INTO restaurants (restaurant_name, restaurant_address, user_id) VALUES ($1, $2, $3) 
            RETURNING restaurant_id, restaurant_name, restaurant_address, user_id",
            res.restaurant_name, res.restaurant_address, usr.user_id
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
async fn create_dish(state: Data<AppState>, dish: web::Json<Dishes>, usr:UserAuth) -> Result<impl Responder, MyError> {
    let d = dish.into_inner();
    let role_row = usr.roles;

    if role_row.iter().any(|x| {
        x.role_type == "Admin"|| x.role_type == "SubAdmin"
    }) {
        let row = sqlx::query_as!( Dishes,
            "INSERT INTO dishes (dish_name, dish_cost, restaurant_id, user_id, time) VALUES ($1, $2, $3, $4, $5) 
            RETURNING dish_id, dish_name, dish_cost, restaurant_id, user_id, time",
            d.dish_name, d.dish_cost, d.restaurant_id, usr.user_id, d.time
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
async fn get_restaurant_by_user_id(state: Data<AppState>, usr: UserAuth) -> Result<impl Responder, MyError> {

    let role_row = usr.roles;

    if role_row.iter().any(|x| {
        x.role_type == "Admin"
    }){
        let rows = sqlx::query_as!(Restaurants,
            "SELECT restaurant_id, restaurant_name, restaurant_address, user_id 
            FROM Restaurants"
        )
        .fetch_all(&state.db)
        .await?;
        Ok(HttpResponse::Ok().json(rows))
    } else if role_row.iter().any(|x| {
        x.role_type == "SubAdmin"
    }){
        let rows = sqlx::query_as!(Restaurants,
            "SELECT restaurant_id, restaurant_name, restaurant_address, user_id 
            FROM Restaurants WHERE user_id=$1", usr.user_id
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
async fn get_dish_by_user_id(state: Data<AppState>, usr: UserAuth) -> Result<impl Responder, MyError> {
    
    let role_row = usr.roles;

    if role_row.iter().any(|x| {
        x.role_type == "Admin"
    }){
        let rows = sqlx::query_as!(Dishes,
            "SELECT dish_id, dish_name, dish_cost, restaurant_id, user_id, time
            FROM dishes"
        )
        .fetch_all(&state.db)
        .await?;
        Ok(HttpResponse::Ok().json(rows))
    } else if role_row.iter().any(|x| {
        x.role_type == "SubAdmin"
    }) {
        let rows = sqlx::query_as!(Dishes,
            "SELECT dish_id, dish_name, dish_cost, restaurant_id, user_id, time 
            FROM dishes WHERE user_id=$1", usr.user_id
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
async fn get_users_list(state: Data<AppState>, usr:UserAuth) -> Result<impl Responder, MyError> {

    let role_row = usr.roles;

    if role_row.iter().any(|x| {
        x.role_type == "Admin"|| x.role_type == "SubAdmin"
    }) {
        let row = sqlx::query_as!( Users,
            "SELECT user_id, user_name, user_password, user_email, credit FROM users"
        )
        .fetch_all(&state.db)
        .await?;
        Ok(HttpResponse::Ok().json(row))
    } else{
        Err(MyError::UnAuthorized)
    }
    
}



