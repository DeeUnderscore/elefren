/// Used for ser/de of list resources
#[derive(Clone, Debug, Deserialize)]
pub struct List {
    id: String,
    title: String,
}
