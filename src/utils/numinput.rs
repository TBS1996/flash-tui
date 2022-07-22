


#[derive(Clone)]
pub struct NumField {
    pub textrep: String,
    pub num: f32,
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub int: bool, 
}


impl NumField{
    pub fn new() -> Self{
        NumField{
            textrep: String::from(""),
            num: 0f32,
            min: Some(0f32),
            max: Some(100f32),
            int: false,
        }
    }


    pub fn addchar(&mut self, c: char){
        self.textrep.push(c);
        match self.textrep.parse::<f32>(){
            Ok(num) => self.num = num,
            Err(_) => self.backspace(),
        }
    }


    pub fn backspace(&mut self){
        if self.textrep.len() > 1{
            self.textrep.pop();
            self.num = self.textrep.parse::<f32>().unwrap();
        } else if self.textrep.len() == 1{
            self.textrep.pop();
            self.num = 0f32;
        }
    }

    pub fn clear(&mut self){
        self.textrep = String::from("");
        self.num = 0f32;
    }


    
}


