//Contains all possible ingredients and types of orders

//Fries
#[derive(Copy,Clone,Debug)] //This allows the following item to be printed out and also be duplicated
pub enum FryType{
    Small,
    Medium,
    Large,
    Curly,
}

#[derive()]
pub enum Fry{
    Normal,
    Curly,
}

//Burgers
