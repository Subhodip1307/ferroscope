pub struct NotificationData {
    pub category: String,
    pub sujbect: String,
    pub unique_id: String,
}
impl NotificationData {
    pub fn get_message(&self) -> String {
        if self.category == "NODE" {
            return format!("Node Offline {}", self.unique_id);
        } else if self.category == "SERVICE" {
            return format!("Service Offline {}", self.unique_id); //need to imporve
        }
        "".to_string()
    }
}
