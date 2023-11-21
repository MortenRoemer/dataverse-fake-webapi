use crate::version::Version;

#[delete("/<version>/<entity_id>")]
pub async fn delete(version: Version, entity_id: &str) {
    
}