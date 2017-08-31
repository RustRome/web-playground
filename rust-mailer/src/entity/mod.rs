use super::schema::orders;


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct Order {
    id : i64,
    name : String
}




#[derive(Insertable)]
#[table_name="orders"]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct NewOrder {
    name : String
}
