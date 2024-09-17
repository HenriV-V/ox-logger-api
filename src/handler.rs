use crate::{
    model::{AppState, QueryOption, Quest, UpdateQuestSchema},
    response::{GenericResponse, QuestData, QuestListResponse, SingleQuestResponse},
    utils::days_left,
};

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use uuid::Uuid;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "working";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

#[get("/quests")]
pub async fn quests_list_handler(
    opts: web::Query<QueryOption>,
    data: web::Data<AppState>,
) -> impl Responder {
    let quests = data.quest_db.lock().unwrap();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let quests: Vec<Quest> = quests
        .clone()
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();

    let json_response = QuestListResponse {
        status: "success".to_string(),
        results: quests.len(),
        quests,
    };
    HttpResponse::Ok().json(json_response)
}

#[post("/quests")]
async fn create_quest_handler(
    mut body: web::Json<Quest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut vec = data.quest_db.lock().unwrap();

    let quest = vec.iter().find(|quest| quest.title == body.title);

    if quest.is_some() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Quest with title: '{}' already exists", body.title),
        };
        return HttpResponse::Conflict().json(error_response);
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Local::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    if body.deadline.is_none() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: "Deadline is required.".to_string(),
        };
        return HttpResponse::Conflict().json(error_response);
    }

    let quest = body.to_owned();

    vec.push(body.into_inner());

    let days_left_count = if let Some(deadline) = quest.deadline {
        Some(days_left(deadline))
    } else {
        None
    };

    let json_response = SingleQuestResponse {
        status: "success".to_string(),
        data: QuestData {
            quest,
            days_left: days_left_count,
        },
    };

    HttpResponse::Ok().json(json_response)
}

#[get("/quests/{id}")]
async fn get_quest_handler(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let vec = data.quest_db.lock().unwrap();

    let id = path.into_inner();
    let quest = vec.iter().find(|quest| quest.id == Some(id.to_owned()));

    if quest.is_none() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Quest with ID: {} not found", id),
        };
        return HttpResponse::NotFound().json(error_response);
    }

    let quest = quest.unwrap();

    let days_left_count = if let Some(deadline) = quest.deadline {
        Some(days_left(deadline))
    } else {
        None
    };
    let json_response = SingleQuestResponse {
        status: "success".to_string(),
        data: QuestData {
            quest: quest.clone(),
            days_left: days_left_count,
        },
    };

    HttpResponse::Ok().json(json_response)
}

#[patch("/quests/{id}")]
async fn edit_quest_handler(
    path: web::Path<String>,
    body: web::Json<UpdateQuestSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut vec = data.quest_db.lock().unwrap();

    let id = path.into_inner();
    let quest = vec.iter_mut().find(|quest| quest.id == Some(id.to_owned()));

    if quest.is_none() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Quest with ID: {} not found", id),
        };
        return HttpResponse::NotFound().json(error_response);
    }

    let quest = quest.unwrap();
    let datetime = Local::now();
    let title = body.title.to_owned().unwrap_or(quest.title.to_owned());
    let content = body.content.to_owned().unwrap_or(quest.content.to_owned());
    let payload = Quest {
        id: quest.id.to_owned(),
        title: if !title.is_empty() {
            title
        } else {
            quest.title.to_owned()
        },
        content: if !content.is_empty() {
            content
        } else {
            quest.content.to_owned()
        },
        deadline: body.deadline.or(quest.deadline),
        completed: if body.completed.is_some() {
            body.completed
        } else {
            quest.completed
        },
        createdAt: quest.createdAt,
        updatedAt: Some(datetime.into()),
    };
    *quest = payload;

    let days_left_count = if let Some(deadline) = quest.deadline {
        Some(days_left(deadline))
    } else {
        None
    };

    let json_response = SingleQuestResponse {
        status: "success".to_string(),
        data: QuestData {
            quest: quest.clone(),
            days_left: days_left_count,
        },
    };

    HttpResponse::Ok().json(json_response)
}

#[delete("/quests/{id}")]
async fn delete_quest_handler(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut vec = data.quest_db.lock().unwrap();

    let id = path.into_inner();
    let quest = vec.iter_mut().find(|quest| quest.id == Some(id.to_owned()));

    if quest.is_none() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Quest with ID: {} not found", id),
        };
        return HttpResponse::NotFound().json(error_response);
    }

    vec.retain(|quest| quest.id != Some(id.to_owned()));
    HttpResponse::NoContent().finish()
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(quests_list_handler)
        .service(create_quest_handler)
        .service(get_quest_handler)
        .service(edit_quest_handler)
        .service(delete_quest_handler);

    conf.service(scope);
}
