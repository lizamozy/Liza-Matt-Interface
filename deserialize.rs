#[derive(Debug, Deserialize)]
pub struct IDLFunction {
    name: String,
    params: Vec<String>,
    returnType: String,
}