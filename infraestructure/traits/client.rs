mod infraestructure{    
  
    pub trait IClient{

        fn set_database(data:String) -> Result<(), Box<dyn Error>>;
        fn set_collection(collection:$str) -> Result<(), Box<dyn Error>>;
        fn insert<T:Vec<T>>(data:T) -> Result<(), Box<dyn Error>>;
    }      

}