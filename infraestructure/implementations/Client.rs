use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;

mod infraestructure{

    pub struct Client{

        client:T;
        database:String,
        collection:String,
    };

    impl Client{

        fn new() -> self{

            let client_uri =
            env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

            let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await.unwrap();
            let client = Client::with_options(options).unwrap();
            DbClient{
                client:client
            }
        }

    };

    impl IClient for Client{

        pub fn set_database(data:String) -> Result<(), Box<dyn Error>>{

        }

        fn set_collection(collection:$str) -> Result<(), Box<dyn Error>>{

        }

        fn insert<T:Vec<T>>(data:T) -> Result<(), Box<dyn Error>>{

            
        }

    };
}