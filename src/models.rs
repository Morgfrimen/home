pub mod models{
    pub struct Soket{
        pub description: String,
        pub is_enable: bool,
        pub power_now: f32    
    }
    
    impl Soket {
        pub fn get_description(&self) -> String{
            todo!()
        }
    
        pub fn change_enable(mut self) -> bool{
            todo!()
        }
    
        pub fn get_power_now(&self) -> f32{
            todo!()
        }
    }
}