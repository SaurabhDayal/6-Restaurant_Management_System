use actix_web::{web};
use actix_web::{put, post, delete, Responder};
use actix_web::{web::Data};
use sqlx;

use crate::error::MyError;
use crate::model::*;

// Add Order by dish id from Restaurant
#[post("/order/{dish_id}")]
pub async fn add_order(state: Data<AppState>, path: web::Path<i32>, usr:UserAuth) -> Result<impl Responder, MyError> {

    // getting dish info from table
    let dish_id = path.into_inner();
    let dish_row= sqlx::query_as!( Dishes,
        "SELECT dish_id, dish_name, dish_cost, restaurant_id, user_id, time FROM dishes WHERE dish_id=$1", dish_id         
    )
    .fetch_one(&state.db)
    .await?;
    
    // adding order to table
    let row = sqlx::query_as!( Orders,
            "INSERT INTO orders (d_id, time, user_id, is_delivered) VALUES ($1, $2, $3, $4) 
            RETURNING id, d_id, time, user_id, is_delivered",
            dish_row.dish_id, dish_row.time, usr.user_id, false
        )
        .fetch_one(&state.db)
        .await?;
    
    // subtracting credit from user
    sqlx::query_as!(Users,
        "UPDATE users SET credit = $1 WHERE user_id = $2 RETURNING user_id, user_name, user_password, user_email, credit",
        usr.credit - dish_row.dish_cost , usr.user_id
        )
        .fetch_one(&state.db)
        .await?;

    // getting dish owner info from table
    let dish_owner = sqlx::query_as!( Users, 
        "SELECT user_id, user_name, user_password, user_email, credit FROM users 
        WHERE user_id=$1", dish_row.user_id
        )
        .fetch_one(&state.db)
        .await?;

    // adding credit to dish owner
    sqlx::query_as!(Users,
        "UPDATE users SET credit = $1 WHERE user_id = $2 RETURNING user_id, user_name, user_password, user_email, credit",
        dish_owner.credit + dish_row.dish_cost , dish_row.user_id
        )
        .fetch_one(&state.db)
        .await?;

    Ok(actix_web::web::Json(row))

}


// Delete Order from order_id
#[delete("/order/{order_id}")]
pub async fn delete_order(state: Data<AppState>, path: web::Path<i32>, usr:UserAuth) -> Result<impl Responder, MyError> {

    // getting order info from table 
    let order_id = path.into_inner();
    let order_row= sqlx::query_as!( Orders,
        "SELECT id, d_id, time, user_id, is_delivered FROM orders WHERE id=$1", order_id         
        )
        .fetch_one(&state.db)
        .await?;

    println!("1");
    
    // getting dish info from table
    let dish_row= sqlx::query_as!( Dishes,
        "SELECT dish_id, dish_name, dish_cost, restaurant_id, user_id, time FROM dishes 
        WHERE dish_id=$1", order_row.d_id         
    )
    .fetch_one(&state.db)
    .await?;

    println!("2");

// checking if ordering person and deleting person are same 
    if usr.user_id==order_row.user_id {
    
        println!("3");

        // adding user credit after cancellation
        sqlx::query_as!(Users,
            "UPDATE users SET credit = $1 WHERE user_id = $2 
            RETURNING user_id, user_name, user_password, user_email, credit",
            usr.credit + dish_row.dish_cost , usr.user_id
            )
            .fetch_one(&state.db)
            .await?;
        
        // finding dish owner credit
        let dish_owner = sqlx::query_as!( Users, 
            "SELECT user_id, user_name, user_password, user_email, credit FROM users 
            WHERE user_id=$1", dish_row.user_id
            )
            .fetch_one(&state.db)
            .await?;
        
        // subtracting user credit after cancellation
        sqlx::query_as!(Users,
            "UPDATE users SET credit = $1 WHERE user_id = $2 
            RETURNING user_id, user_name, user_password, user_email, credit",
            dish_owner.credit - dish_row.dish_cost , dish_row.user_id
            )
            .fetch_one(&state.db)
            .await?;

        // deleting order row
        let row = sqlx::query_as!( Orders,
            "DELETE FROM orders WHERE id=$1 
            RETURNING id, d_id, time, user_id, is_delivered",
            order_row.id
            )
            .fetch_one(&state.db)
            .await?;
        
        Ok(actix_web::web::Json(row))
    } else{
        Err(MyError::UnAuthorized)
    }



}

// Order Done by order_id
#[put("/order/{order_id}")]
pub async fn ok_order(state: Data<AppState>, path: web::Path<i32>, usr:UserAuth) -> Result<impl Responder, MyError> {

    // getting order info from table 
    let order_id = path.into_inner();
    let order_row= sqlx::query_as!( Orders,
        "SELECT id, d_id, time, user_id, is_delivered FROM orders WHERE id=$1", order_id         
        )
        .fetch_one(&state.db)
        .await?;
    
    // checking if ordering person and deleting person are same 
    if usr.user_id==order_row.user_id {

        // 
        let row = sqlx::query_as!(Orders,
            "UPDATE orders SET is_delivered = $1 WHERE id = $2 
            RETURNING id, d_id, time, user_id, is_delivered",
            true, order_row.id
        )
        .fetch_one(&state.db)
        .await?;

        Ok(actix_web::web::Json(row))
    } else{
        Err(MyError::UnAuthorized)
    }
}