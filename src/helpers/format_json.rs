pub type Welcome = Vec<WelcomeElement>;

#[derive(Serialize, Deserialize)]
pub struct WelcomeElement {
    #[serde(rename = "__type__")]
    welcome_type: String,
    fields: Vec<String>,
    guid: String,
    note_model_uuid: String,
    tags: Vec<Option<serde_json::Value>>,
}

fn test(){
    let json = fs::read_to_string("./german.json")
        .unwrap();
    let welcomes: Welcome = serde_json::from_str(&json)
        .unwrap();
    let mut cards = Vec::<Card>::new();
    for w in welcomes{
        let n_card = Card{
            front: w.fields[0].clone(),
            back: w.fields[3].clone(),
            frequencey: w.fields[2].clone(),
            front_example: w.fields[1].clone(),
            back_example: w.fields[5].clone(),
        };
        cards.push(n_card);
    }
    fs::write("german_2.json", serde_json::to_string_pretty(&cards).unwrap().as_bytes());
}

