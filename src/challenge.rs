use crate::technique::Technique;

pub struct Challenge{
    pub steps: Vec<Technique>,
    pub hint: Option<String>,
}

impl Challenge{
    pub fn new() -> Challenge{
        Challenge{
            steps: Vec::new(),
            hint: None,
        }
    }

    pub fn add(&mut self, technique: Technique){
        self.steps.push(technique);
    }

    pub fn hint(&mut self, hint: &str){
        self.hint = Some(String::from(hint));
    }
}
