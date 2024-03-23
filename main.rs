//importing tree sitter crate
use tree_sitter::Parser;
use tree_sitter::Tree;

#[derive(Debug)]
struct ArgField{
    arg_type: String,
    arg_name: String,
}
//Struct for funcitons
#[derive(Debug)]
struct Function{
    f_name: String,
    f_ret_type: String,
    arg_count: u32,
    args: Vec<ArgField>,
}
struct Funcitons{ //strict to hold vector of fucntion structs
    functions: Vec<Function>,
}
//later possobly need to add struct for typedefs\

fn traverse(tree: Option<Tree>, walk: String) -> Function {

    //create a struct to hold all of the important stuff we want like funciton type, name, vector of args and type
    let mut _arg=ArgField{ //intializing arg struct 
        arg_name:"arg".to_string(),
        arg_type:"type".to_string(),
    };

    let dummy_function = Function{     //initializing fucntion struct
        f_name: String::from("func"),
        f_ret_type: String::from("void"),
        arg_count: 0,
        args: vec![_arg],
    };

    //now need to traverse the tree and input all of the information into the structs
    dummy_function
}

fn main() {
    //inputting fie path 
    let file_path = String::from("/Users/lizamozolyuk/Desktop/Systems Research/tree-sitter-exp/header.h");
    //parsing the file_path
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_c::language()).expect("Error loading C grammar");
    let parsed = parser.parse(file_path, None);
    println!("ast: {:?}", parsed);  //now have the AST, parsed

    let mut _walk: String = "".to_string();     //create an empty string toadd characters 

    //funtion to traverse teh tree (later can possibly loop until weve reached all of the nodes and each time we loop, we add to the functions struct and then have a vetor of all of the funciton structs)
    let parsed_function: Function = traverse(parsed, _walk);
    print!("the parsed funciton is {:?}", parsed_function);

    //NEXT STEP: traverese the tree to get the actual funciton def data 
    
    //once we return from parsing the tree, gonna need to get the struct information, put it into new structs that we will use for serialization

}


