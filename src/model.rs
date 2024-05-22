use crate::{Error};
use serde::{Deserialize,Serialize};
use std::sync::{Arc,Mutex};

#[derive(Clone,Debug,Serialize)]
 pub struct User{
     pub id:u64,
     pub name:String,
     pub surname:String,
     pub email:String,
 }

#[derive(Deserialize)]
pub struct UserDto{
    pub name:String,
    pub surname:String,
    pub email:String,
}

#[derive(Clone)]
pub struct ModelController{
    users_store:Arc<Mutex<Vec<Option<User>>>>
}

impl ModelController {
    pub async fn new() -> Result<Self,Error>{
        Ok(Self{ users_store:Arc::default()})
    }
}

impl ModelController {

    pub async fn create_user(
        &self,
        user_dto:UserDto
    ) -> Result<User,Error> {
        let mut store = self.users_store.lock().unwrap();

        let id = store.len() as u64;
        let user = User{
            id,
            name: user_dto.name,
            surname: user_dto.surname,
            email:user_dto.email
        };
        store.push(Some(user.clone()));

        Ok(user)
    }

    pub async fn list_users(&self)->Result<Vec<User>,Error>{
        let store= self.users_store.lock().unwrap();
        let tickets=store.iter().filter_map(|user| user.clone()).collect();
        Ok(tickets)
    }

    pub async fn delete_user(&self,id:u64)->Result<User,Error>{
        let mut store= self.users_store.lock().unwrap();
        let user=store.get_mut(id as usize).and_then(|user| user.take());

        user.ok_or(Error::UserDeleteFailIdNotFound {id})
    }

}