use std::io::Read;

use quick_xml::events::Event;
use quick_xml::reader::Reader;

use serde::{Deserialize, Serialize};
struct CassyDaten
{
    messungen: Vec<Messung>
}


struct Messung
{
    zeitpunkt: String,
    beschreibung: String,
    datenreihen: Vec<Datenreihe>
}
#[derive(Deserialize, Debug)]
#[serde(rename="channel")]
struct Datenreihe
{
    #[serde(rename="values")]
    werte: Vec<Value>,
    #[serde(rename="$value")]
    quantity: String,
    #[serde(rename="$value")]
    symbol: String,
    #[serde(rename="$value")]
    unit: String
}
#[derive(Deserialize, Debug)]
struct Value
{
    #[serde(rename="$value")]
    wert: f64
}
#[derive(Deserialize, Debug, Serialize)]
struct channel
{

    quantity: String,
    symbol: String,
    unit: String,
    values: Vec<f64>
}
fn main(){
    let cassy_daten = CassyDaten{messungen: Vec::new()};
    // let file = std::fs::read_to_string("data_fat.xml").unwrap();
    // let mut reader = Reader::from_str(&file);
    // let mut buf:Vec<u8> = Vec::new();
    // // loop {
    //     match reader.read_event_into(&mut buf) {
    //         Ok(Event::Start(e)) if e.name().as_ref() == b"channel" => {
    //             if let Ok(Some(_)) = e.try_get_attribute("datetime")
    //             {
    //                 let txt = reader.read_text(e.name()).unwrap();
    //                 println!("{}", txt);
    //                 let datenreihe: Datenreihe = serde_xml_rs::from_str(&format!("<channel>{}</channel>", txt)).unwrap();
    //                 println!("{:?}", datenreihe)
    //             }
    //         }
    //         Ok(Event::Eof) => break, // exits the loop when reaching end of file
    //         Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
    //         _ => (), // There are several other `Event`s we do not consider here
    //     }
    // }

    let test = r##"<channel><quantity>Index</quantity><symbol>n</symbol><unit /><range min="0" max="100001" /><values count="100001"></values></channel>"##;
    println!("{}", test);
    let owo: Datenreihe = serde_xml_rs::from_str(test).unwrap();//channel { quantity: "owo".to_string(), symbol: "sus".to_string(), unit: "bus".to_string(), values: vec![420.69f64] };
    //println!("{}", serde_xml_rs::to_string(&owo).unwrap());
}