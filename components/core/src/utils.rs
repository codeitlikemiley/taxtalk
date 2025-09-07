use uuid::Uuid;

pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn class_names(classes: &[(&str, bool)]) -> String {
    classes
        .iter()
        .filter_map(|(class, condition)| {
            if *condition {
                Some(*class)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn format_currency(amount: f64) -> String {
    format!("₱{:.2}", amount)
}

pub fn format_percentage(value: f64) -> String {
    format!("{:.2}%", value * 100.0)
}

pub fn parse_amount(input: &str) -> Option<f64> {
    let cleaned = input
        .replace("₱", "")
        .replace(",", "")
        .replace(" ", "");
    cleaned.parse().ok()
}