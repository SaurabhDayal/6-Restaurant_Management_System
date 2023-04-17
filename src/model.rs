use std::pin::Pin;

use actix_web::{FromRequest, HttpRequest, dev::Payload, web::Data};
use futures_util::Future;
use sqlx::{Pool, Postgres};
use serde::{Deserialize, Serialize};

use crate::error::*;

pub struct AppState {
    pub db: Pool<Postgres>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Users {
    pub user_id: i32,
    pub user_name: String,
    pub user_password: String,
    pub user_email: String,
}


#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Auths {
    pub user_id: i32,
    pub user_token: String
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Roles {
    pub role_id: i32,
    pub role_type: String,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Addresses {
    pub address_id: i32,
    pub address_name: String,
    pub address_lat: f64,
    pub address_lng: f64,
    pub user_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Restaurants {
    pub restaurant_id: i32,
    pub restaurant_name: String,
    pub restaurant_address: i32,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Dishes {
    pub dish_id: i32,
    pub dish_name: String,
    pub dish_cost: i32,
    pub restaurant_id: i32,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct AddressDistance {
    pub distance: String
}

impl FromRequest for Users {
    type Error = MyError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            
            let db = req.app_data::<Data<AppState>>();
            if db .is_none() {
                return Err(MyError::InternalError);
            }
            
            let token = req.headers().get("Authorization");
            return match token {
                Some(data) => {

                    let state = db.unwrap().clone();

                    let auth_token = token.unwrap().to_str().unwrap().clone();
                    let x=&auth_token[7..];

                    let auth_row = sqlx::query_as!(Auths,"SELECT user_token, user_id FROM auths 
                        WHERE user_token =$1", x)
                        .fetch_one(&state.db)
                        .await;

                    match auth_row{
                        Ok(a)=>{

                            let user = sqlx::query_as!(Users, 
                                "SELECT u.user_id, u.user_name, u.user_password, u.user_email 
                                FROM Users u INNER JOIN Auths a ON u.user_id = a.user_id where u.user_id=$1", 
                                a.user_id)
                                .fetch_one(&state.db)
                                .await;

                            Ok(user.unwrap())
                        },
                        _=>Err(MyError::InternalError)
                    }
                   }

                _ => Err(MyError::UnAuthorized)
            }
        })
}}