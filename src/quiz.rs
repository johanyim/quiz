use crate::question::*;

// object for a quiz, which defines a container structure of questions

enum QuizPart {
    Question(Box<dyn Question>),
    Branch(Vec<QuizPart>)
}

pub struct Quiz {
    title: String,
    parts: Vec<QuizPart>,
}


// TODO: Think about the structure of generating a quiz, from the backend (endpoints and fns) and the frontend (htmx + forms)
impl Quiz {
    fn new(title: &str) -> Self {
        Quiz {
            title: title.to_string(),
            parts: vec![],
        }
    }



}


#[test]
fn make_quiz() {

    
    
}



