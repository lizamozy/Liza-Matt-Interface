#[derive(Debug, Serialize)]
pub struct IDLFunction {
    name: String,
    params: Vec<String>,
    returnType: String,
}