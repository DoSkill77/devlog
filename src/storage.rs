use crate::models::Entry;

pub fn load() -> Vec<Entry> {
    let mut path = dirs::home_dir().unwrap();
    path.push(".devlog.json");
    let file = std::fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&file).unwrap_or_default()
}
pub fn save(entry: Entry) {
    let mut path = dirs::home_dir().unwrap();
    path.push(".devlog.json");
    let mut entries = load();
    entries.push(entry);
    let contents = serde_json::to_string_pretty(&entries).unwrap();
    std::fs::write(path, &contents).unwrap();
}
use crate::models::Entry;

pub fn load() -> Vec<Entry> {
    let mut path = dirs::home_dir().unwrap();
    path.push(".devlog.json");
    let file = std::fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&file).unwrap_or_default()
}
pub fn save(entry: Entry) {
    let mut path = dirs::home_dir().unwrap();
    path.push(".devlog.json");
    let mut entries = load();
    entries.push(entry);
    let contents = serde_json::to_string_pretty(&entries).unwrap();
    std::fs::write(path, &contents).unwrap();
}
