use uuid::Uuid;

pub fn create_random_id<'a>(prefix: String) -> String {
    let uuid = Uuid::new_v4().to_string();
    let mut result = prefix.clone();
    result.push_str(uuid.as_str());
    result
}
