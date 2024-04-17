#![allow(unused)]

use crate::utils::*;
use std::fmt::{self, format, Debug, Display};
use askama::Template;
// use askama_axum::IntoResponse;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use std::hash;

// use crate::error::QuestionError::{self, *};
// pub type Result<T> = core::result::Result<T, QuestionError>;

use anyhow::{anyhow, Result};


pub trait Question  {
    // associated type for answer to this question, used in the check fn
    // answer must be hashable as the user response is hashed and compared to the hashed answer
    fn show(&self);

    fn check(&self, answer: u64) -> bool;
}


#[derive(Template)] 
#[template(path = "multiplechoice.html")] 
pub struct MultipleChoiceRadio<T: Display>{
    question_text: String,
    options: Vec<T>,
    correct_answer: u64, 
}



impl<T: Display> MultipleChoiceRadio<T> {
    pub fn new(question_text: &str, options: Vec<T>, correct_answer: usize) -> Self {

        MultipleChoiceRadio {
            question_text: question_text.to_string(),
            options, 
            correct_answer: calculate_hash(&correct_answer)
        }
    }
}


impl<T: Display> Question for MultipleChoiceRadio<T> {
    fn show(&self) {
        todo!();
    }
    fn check(&self, answer: u64) -> bool {
        answer == self.correct_answer 
    }
}


// IDEA: Might later make a generic integer type 

#[derive(Template)] 
#[template(path = "integer.html")] 
pub struct Integer<'a>{
    question_text: String,
    correct_answer: u64, 
    range: Option<(i32, i32)>,
    tolerance: Option<i32>,
    units: Option<&'a str>,
}

impl Integer<'_> {
    pub fn new(question_text: &str, correct_answer: i32) -> Self {
        Integer {
            question_text: question_text.to_string(),
            correct_answer: calculate_hash(&correct_answer),
            range: None,
            tolerance: None, 
            units: None,
        }
    }
}



impl<'a> Integer<'a> {
    pub fn with_range(mut self, range: (i32, i32)) -> Result<Self> {
        self.range = Some(range);
        Ok(self)
    }

    pub fn with_tolerance(mut self, tolerance: i32)  -> Result<Self> {
        if (tolerance < 0) {
            return Err(anyhow!("Tolerance cannot be zero"));
        }
        self.tolerance = Some(tolerance);
        Ok(self)
    }
    
    //IDEA
    pub fn with_units(mut self, units: &'a str) -> Result<Self>{
        self.units = Some(&units);
        Ok(self)
    }
}


impl Question for Integer<'_> {
    // type Answer = i32;
    fn show(&self) {
        todo!();
    }
    fn check(&self, answer: u64) -> bool {
        answer == self.correct_answer 
    }
}



#[derive(Template)]
#[template(path = "text.html")]
pub struct Text{
    question_text: String,
    correct_answer: u64, 
    // character_count: usize
}

impl Text {
    pub fn new(question_text: &str, correct_answer: &str) -> Self {
        Text {
            question_text: question_text.to_string(),
            correct_answer: calculate_hash(&correct_answer),
            // character_count: 500,
        }
    }
}


impl Question for Text {
    // type Answer = String;
    fn show(&self) {
        todo!();
    }
    fn check(&self, answer: u64) -> bool {
        answer == self.correct_answer 
    }
}






#[cfg(test)]
mod tests {
    use crate::question::{self, Question};
    // type Result<T> = std::result::Result<T, crate::error::QuestionError>;
    use anyhow::Result;

    mod multi_choice_radio {
        use super::*;

        #[test]
        fn check_default_type() {

            let question = question::MultipleChoiceRadio::new("What is the captical of China?", 
                                                           vec!["Hong kong", "Beijing", "Guangzhou", "Taiwan", "Thailand"]
                                                           .into_iter().map(String::from)
                                                           .collect::<Vec<String>>(), 
                                                           1);
            assert!(!question.check(0));
            assert!(question.check(1));
            assert!(!question.check(2));
            assert!(!question.check(3));
        }

        #[test]
        fn check_defined_type() {


            let question = question::MultipleChoiceRadio::<u32>::new("How many teeth does the average adult male have?", 
                                                        vec![42,24,32,28,40,36], 
                                                        2);
            assert!(!question.check(0));
            assert!(!question.check(1));
            assert!(question.check(2));
            assert!(!question.check(3));
            assert!(!question.check(4));
            assert!(!question.check(5));
        }
    }

    mod integer {
        use super::*;

        #[test]
        fn check_default_type() {
            let question = question::Integer::new("What is 10+10?", 20);
            let correct = question.check(20);
            // let incorrect = question.check();
            // let incorrect2 = question.check(choice);
        }
        
        #[test]
        #[ignore]
        fn tolerance() -> Result<()> {
            let question = question::Integer::new("How tall is Donald Trump (cm)?", 190)
                .with_tolerance(2)?
                .with_range((100,200))?;
            
            let corrects = (185..195);
            for correct in corrects {
                println!("{}",question.check(correct));
                assert!(question.check(correct))
            }


            return Ok(());
        }

    }

}

