
#[derive(FromForm)]
pub struct ProposedEvent {
  pub name: String,
  pub description: String,
  pub location: String,
  pub host: String
}