use std::fmt;
#[derive(Debug, PartialEq, Eq)]
    
pub struct Clock {
    hours: i32,
    minutes: i32, }

//todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * 60 + minutes).rem_euclid(24*60);

            Self { 
            hours: total_minutes/60,
            minutes: total_minutes.rem_euclid(60),
        }
        
        
    }
     //todo!("Add {minutes} minutes to existing Clock time");
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
       
    }
}

impl fmt::Display for Clock{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

fn main (){
    let clock = Clock::new(10, 30).add_minutes(45);

    println!("Clock time: {}", clock);
}
