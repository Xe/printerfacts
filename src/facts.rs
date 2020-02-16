use serde_json::*;
use std::sync::Arc;

fn load_facts() -> Result<Vec<String>> {
    let data: &[u8] = include_bytes!("./printerfacts.json");
    let result: Vec<String> = serde_json::from_slice(data)?;

    Ok(result)
}

pub type Facts = Arc<Vec<String>>;

pub fn make() -> Facts {
    let facts = load_facts().unwrap(); // could panic, i guess

    Arc::new(facts)
}

#[cfg(test)]
mod tests {
    #[test]
    fn load_facts() {
        assert!(super::load_facts().is_ok());
    }
}
