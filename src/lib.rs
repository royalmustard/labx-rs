use pyo3::prelude::*;
 


use std::path::Path;
use std::{collections::HashMap, io::Read};

use quick_xml::events::Event;
use quick_xml::reader::Reader;


#[derive(Debug)]
#[pyclass]
struct CassyDaten {
    messungen: Vec<Messung>,
}

#[derive(Debug, Clone)]
#[pyclass]
struct Messung {
    zeitpunkt: String,
    beschreibung: String,
    datenreihen: Vec<Datenreihe>,
}

#[derive(Debug, Clone)]
#[pyclass]
struct Datenreihe {
    #[pyo3(get)]
    quantity: String,
    #[pyo3(get)]
    symbol: String,
    #[pyo3(get)]
    unit: String,
    #[pyo3(get)]
    values: Vec<f64>,
}

#[pymethods]
impl CassyDaten
{
    fn messung(&self, number: usize) -> Messung
    {
        self.messungen.get(number-1).unwrap().clone()
    }
}

#[pymethods]
impl Messung
{
    fn datenreihe(&self, symbol: String) -> Datenreihe
    {
        self.datenreihen.iter().filter(|d| d.symbol == symbol).cloned().collect::<Vec<Datenreihe>>().get(0).unwrap().to_owned()
    }

    fn info(&self)
    {
        println!("{:?}", self.datenreihen.iter().map(|d| d.symbol.clone()).collect::<Vec<String>>());
    }
}

// impl Display for Vec<f64>
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let out: String = format!("[{}, ..., {}] ({} values)", self.first(), self.last(), self.len());
//         write!(f, "{}", out)
//     }
// }
#[pyfunction]
fn read_labx(filename: String) -> CassyDaten {

    let file = std::fs::File::open(Path::new(&filename)).expect("Error opening file!");
    let mut buf = String::new();
    zip::ZipArchive::new(file).expect("Error parsing zipfile!").by_name("data.xml").expect("data.xml not found in zipfile").read_to_string(&mut buf).expect("Error reading data.xml to buffer");
    let mut reader = Reader::from_str(&buf);

    


    let mut cassy_daten = CassyDaten {
        messungen: Vec::new(),
    };
    let mut buf: Vec<u8> = Vec::new();
    let mut unit_indices: HashMap<String, usize> = HashMap::new();
    let mut current_symbol: String = "".to_string();
    let mut messung: Messung;
    let mut datenreihe_vec: Vec<f64> = Vec::new();
    let mut datenreihe: Datenreihe = Datenreihe { values: Vec::new(), quantity: "sus".to_string(), symbol: "bus".to_string(), unit: "ඞ".to_string() };
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"channel" => {
                if !datenreihe_vec.is_empty()
                {
                    datenreihe.values = datenreihe_vec.clone();
                    cassy_daten.messungen.get_mut(*unit_indices.get(&current_symbol).unwrap()).unwrap().datenreihen.push(datenreihe.clone());
                    //println!("{:?}", datenreihe);
                    datenreihe_vec.clear();
                }
                if let Ok(Some(_)) = e.try_get_attribute("datetime") 
                {
                    //Erstelle Neue Messung
                    messung = Messung{zeitpunkt: reader.decoder().decode(e.try_get_attribute("datetime").unwrap().unwrap().value.as_ref()).unwrap().to_string(),
                                    beschreibung: String::from(""), //sort it out later,
                                datenreihen: Vec::new()};
                    cassy_daten.messungen.push(messung);

                }
                
            }
            Ok(Event::Start(e)) if e.name().as_ref() == b"quantity" =>
            {
                datenreihe.quantity = reader.read_text(e.name()).unwrap().to_string();
            }
            Ok(Event::Start(e)) if e.name().as_ref() == b"symbol" =>
            {
                current_symbol = reader.read_text(e.name()).unwrap().to_string();
                datenreihe.symbol = current_symbol.to_string();
                unit_indices.entry(current_symbol.clone()).and_modify(|idx| *idx = *idx+1).or_insert(0);
                
            }
            Ok(Event::Start(e)) if e.name().as_ref() == b"unit" =>
            {
                datenreihe.unit = reader.read_text(e.name()).unwrap().to_string();
            }
            Ok(Event::Start(e)) if e.name().as_ref() == b"value" => {
                datenreihe_vec.push( fast_float::parse(reader.read_text(e.name()).unwrap().as_ref()).unwrap());
            }
            Ok(Event::Eof) => {
                if !datenreihe_vec.is_empty()
                {
                    datenreihe.values = datenreihe_vec.clone();
                    cassy_daten.messungen.get_mut(*unit_indices.get(&current_symbol).unwrap()).unwrap().datenreihen.push(datenreihe.clone());
                    //println!("{:?}", datenreihe);
                    datenreihe_vec.clear();
                }
                break
            }, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }
    return cassy_daten;
    //println!("{:?}", cassy_daten)
}

#[pymodule]
fn labx(_py: Python<'_>, m: &PyModule) -> PyResult<()>
{
    m.add_class::<CassyDaten>()?;
    m.add_class::<Messung>()?;
    m.add_class::<Datenreihe>()?;
    m.add_function(wrap_pyfunction!(read_labx, m)?)?;
    Ok(())
}