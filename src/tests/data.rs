use crate::entity;

pub fn events(user_id: &String) -> Vec<entity::event::EventEntity> {
    vec![
        entity::event::EventEntity {
            id: None,
            user_id: user_id.to_string(),
            title: "Example Event".to_string(),
            description: "This is an example description".to_string(),
            location: "Example Location".to_string(),
            all_day: false,
            start: chrono::offset::Utc::now(),
            end: chrono::offset::Utc::now(),
            event_uid: Some("EventUID".to_string()),
        },
        entity::event::EventEntity {
            id: None,
            user_id: user_id.to_string(),
            title: "Second example Event".to_string(),
            description: "This is the second example description".to_string(),
            location: "Second e Location".to_string(),
            all_day: false,
            start: chrono::offset::Utc::now(),
            end: chrono::offset::Utc::now(),
            event_uid: Some("EventUID".to_string()),
        },
    ]
}

pub fn users(auth_id: &String) -> Vec<entity::user::User> {
    vec![entity::user::User {
        id: None,
        auth_id: auth_id.to_string(),
        name: "John Doe".to_string(),
        sources: vec![],
    }]
}
