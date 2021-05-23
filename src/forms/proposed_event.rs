use rocket::request::{FromForm, FormItems};

#[derive(Debug)]
pub struct ProposedEvent {
  pub name: String,
  pub description: String,
  pub location: String,
  pub host: String,
  pub invitees: Vec<String>,
  pub options: Vec<String>,

}

impl<'f> FromForm<'f> for ProposedEvent {
  type Error = String;

  fn from_form(form_items: &mut FormItems<'f>, strict: bool) -> Result<ProposedEvent, Self::Error> {
    let mut update = ProposedEvent {
        name: String::new(),
        description: String::new(),
        location: String::new(),
        host: String::new(),
        invitees: Vec::new(),
        options: Vec::new()
    };

    for item in form_items {
        let key: &str = item.key;
        let value = item.value.to_string();
        println!("{:?}", key);
        println!("{:?}", value);

        match key {
            "name" => update.name = value,
            "description" => update.description = value,
            "location" => update.location = value,
            "host" => update.host = value,
            "invitee%5B%5D" => { update.invitees.push(value); },
            "option%5B%5D" => { update.options.push(value); },
            _ => {
                //return Err(Error::InvalidData(format!["invalid form data name: '{}'", key]));
            },
        }
    }

    Ok(update)
  }
}
  