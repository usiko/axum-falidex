use mongodb::{ 
	bson::{Document, doc},
	Client,
	Collection 
};

pub struct MongoDB {
    pub client: Client,
    pub db: Database,
}
pub async fn connect() -> mongodb::error::Result<Self> {
    // Replace the placeholder with your Atlas connection string
    let uri = "mongodb+srv://quentinusiko_db_user:58sErWBbkymGdUHb@cluster0.1vbyumm.mongodb.net/?appName=Cluster0";
    // Create a new client and connect to the server
     let client = Client::with_uri_str(uri).await?;
        let db = client.database("axum");

        Ok(Self { client, db })
   /*  let my_coll: Collection<Document> = database.collection("movies");
    // Find a movie based on the title value
    let my_movie = my_coll.find_one(doc! { "title": "The Perils of Pauline" }).await?;
    // Print the document
    println!("Found a movie:\n{:#?}", my_movie);
    Ok(())*/
}

fn get_uri()->String
{
    String::from("mongodb+srv://...")
}


fn get_col()-> Collection<Document>
{
    let db = Self::db;
    db.collection("text-persist")
}

fn get(key:String, value:String)//:mongodb::error::Result<()>
{
    
    let col:Collection<Document> = get_col();
    let persist = col.find_one(doc! { "name": key }).await?;
}
fn set(key:String, value:String)//:mongodb::error::Result<()>
{
    
}