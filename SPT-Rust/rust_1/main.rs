use std::io::stdin;// allows me to read keyboard inputs from user
//incredibly long list of keywords

const KWRDS_DEF:[&str;39] = ["perform primitive casting, disambiguate the specific trait containing an item, or rename items in use statements", "return a Future instead of blocking the current thread", "suspend execution until the result of a Future is ready", "exit a loop immediately", "define constant items or constant raw pointers", "continue to the next loop iteration", "in a module path, refers to the crate root", "dynamic dispatch to a trait object", "fallback for if and if let control flow constructs", "define an enumeration", "link an external function or variable", "Boolean false literal", "define a function or the function pointer type", "loop over items from an iterator, implement a trait, or specify a higher-ranked lifetime", "branch based on the result of a conditional expression", "implement inherent or trait functionality", "part of for loop syntax", "bind a variable", "loop unconditionally", "match a value to patterns", "define a module", " make a closure take ownership of all its captures", "denote mutability in references, raw pointers, or pattern bindings", "denote public visibility in struct fields, impl blocks, or modules", "bind by reference", "return from function", "a type alias for the type we are defining or implementing", "method subject or current module", "global variable or lifetime lasting the entire program execution", "define a structure", "parent module of the current module", "define a trait", "Boolean true literal", "define a type alias or associated type", "define a union; is only a keyword when used in a union declaration", "denote unsafe code, functions, traits, or implementations"
,"use - bring symbols into scope",  "denote clauses that constrain a type", "loop conditionally based on the result of an expression"];
//var testing
const PRINT_S : &str = "Rust is Awesome!";
const INTR_KEYWRD : &str = "*EXTERN*";
//main function
fn main() {
    println!("Hello, world! {\n}", (PRINT_S));
    
    keywrd_1();
    lookup();
}
//testing function


// the only reason this app ios boring is because I cant seem to convert my strings to the correct format very. very.. Sad...
fn keywrd_1() {
    println!(" A really neat Keyword I chose was {} because it allows yo to call a variable from another function.", (INTR_KEYWRD));
    println!(" The Keyword Async has been really useful in the past when creating functions that involve a delayed feature");
    println!(" Break is a  really useful keyword to use when you want to stop a program in its tracks, Ive used it for debugging purposes before.");
    println!("The text and input you see below is an attempt to make a functio that takes in a keyword and outputs the definition. ");
}   
//searches through array and gives description of input KEYWORD
fn lookup(){
    // a last ditch attempt to fix an error that is not allowing a comparison between &str and String :(
    let list_of_kwrds = vec!["as".to_string(), "async".to_string(), "await".to_string(), "break".to_string(), "const".to_string(), "continue".to_string(), "crate".to_string(), "dyn".to_string(), "else".to_string(), "enum".to_string(), "extern".to_string(), "false".to_string(), "fn".to_string(), "for".to_string(), "if".to_string(), "impl".to_string(), "in".to_string(), "let".to_string(), "loop".to_string(), "match".to_string(), "mod".to_string(), "move".to_string(), "mut".to_string(), "pub".to_string(), "ref".to_string(), "return".to_string(), "Self".to_string(), "self".to_string(), "static".to_string(), "struct".to_string(), "super".to_string(), "trait".to_string(), "true".to_string(), "type".to_string(), "union".to_string(), "unsafe".to_string(), "use".to_string(), "where".to_string(), "while".to_string()];
    println!("Enter a keyword or type (help) for a list of keywords");


    // sets the user input to a var
    let mut k_wrd_input = String::new();
    stdin()
     .read_line(&mut k_wrd_input)
     .expect("Failed to read line");

     //ensureing the var is a String
     k_wrd_input.to_string();
     //below is supposed to parse throught the list of keywords and tell me if there is a match, sadly it does not work although i dont get any errors
    if let Some(k_wrd_input) = list_of_kwrds.into_iter().find(|s| s == &k_wrd_input){
       
        println!("!Found keyword {}", {k_wrd_input});
        
    } else {

        println!("!could not find Keyword ==> {}",k_wrd_input);
      
    }
    
  //dont mind this junk below
/*
    for x in LIST_OF_KWRDS{

        x.to_string();
        if k_wrd_input.to_string() == x{
            println!("!Found keyword ");


        }

        else {

            println!("!could not find Keyword!");
            
        }
        
*/

        

    
    
    


}