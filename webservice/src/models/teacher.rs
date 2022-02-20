use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Teacher {
    pub id: i32, // serial
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTeacher {
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTeacher {
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

impl From<web::Json<CreateTeacher>> for CreateTeacher {
    fn from(new_teacher: web::Json<CreateTeacher>) -> Self {
        CreateTeacher {
            name: new_teacher.name.clone(),
            picture_url: new_teacher.picture_url.clone(),
            profile: new_teacher.profile.clone(),
        }
    }
}

impl From<web::Json<UpdateTeacher>> for UpdateTeacher {
    fn from(teacher: web::Json<UpdateTeacher>) -> Self {
        UpdateTeacher {
            name: teacher.name.clone(),
            picture_url: teacher.picture_url.clone(),
            profile: teacher.profile.clone(),
        }
    }
}
