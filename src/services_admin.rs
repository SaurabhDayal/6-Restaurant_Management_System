use actix_web::{web};
use actix_web::{get, post, Responder};
use actix_web::{web::Data, HttpResponse};
use sqlx;

use crate::error::MyError;
use crate::model::*;


// Create SubAdmin
#[post("/subadmin")]
async fn create_subadmin(state: Data<AppState>, user: web::Json<Users>, usr:Users) -> Result<impl Responder, MyError> {
    let user = user.into_inner();
    let b_id = usr.user_id;

    let role_row = sqlx::query_as!( Roles,"SELECT role_id, role_type, user_id FROM roles WHERE user_id=$1", b_id
    )
    .fetch_all(&state.db)
    .await?;

    let mut bool = false;
    for role in role_row{
        if role.role_type =="Admin".to_string(){
            bool = true;
        }
    }

    if bool  {
        let mut tx = state.db.begin().await.map_err(|_| MyError::InternalError )?;
        let row = sqlx::query_as!(Users,
            "INSERT INTO users (user_name, user_password, user_email) VALUES ($1, $2, $3) 
            RETURNING user_id, user_name, user_password, user_email",
            user.user_name, user.user_password, user.user_email
        )
        .fetch_one(&mut tx)
        .await?;
    
        sqlx::query_as!( Roles,"INSERT INTO roles (role_type, user_id) VALUES($1, $2) 
        RETURNING role_id, role_type, user_id", "SubAdmin".to_string(), row.user_id
        )
        .fetch_one(&mut tx)
        .await?;
        
        tx.commit().await.map_err(|_| MyError::InternalError)?;
    
        Ok(actix_web::web::Json(row))
    } else{
        Err(MyError::UnAuthorized)
    }
    
}

// Get SubAdmin list
#[get("/subadmin")]
async fn get_subadmin_list(state: Data<AppState>, usr:Users) -> Result<impl Responder, MyError> {

    let b_id = usr.user_id;

    let role_row = sqlx::query_as!( Roles,"SELECT role_id, role_type, user_id FROM roles WHERE user_id=$1", b_id
    )
    .fetch_all(&state.db)
    .await?;

    let mut bool = false;
    for role in role_row{
        if role.role_type =="Admin".to_string(){
            bool = true;
        }
    }

    if bool {
        let row = sqlx::query_as!( Users,
            "SELECT u.user_id, u.user_name, u.user_password, u.user_email FROM users u 
            INNER JOIN roles r ON u.user_id=r.user_id
            WHERE r.role_type='SubAdmin'"
        )
        .fetch_all(&state.db)
        .await?;
        Ok(HttpResponse::Ok().json(row))
    } else{
        Err(MyError::UnAuthorized)
    }
    
}

/*
1) Roles should be aquired in middleware
2) Try to use transactions where applicable
3) Migrations
 */
