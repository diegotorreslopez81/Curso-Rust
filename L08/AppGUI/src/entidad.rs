#![allow(non_snake_case)]

use std::{path::Path, fs::{File, self}, collections::HashMap};
use serde::{Deserialize, Serialize};
use std::error::Error;

pub trait ScreenOutput {
    fn toScreen(&self) -> String;
}


#[derive(Debug, Deserialize,Serialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct Vivienda {
    pub id: String,
    pub calle : String,
    pub numPiso : String,
    pub cp : String,
    pub m2: String,
    pub numBanos: String,
    pub numHabitaciones: String,
    pub tipoVivienda: String
}

impl ScreenOutput for Vivienda {
    fn toScreen(&self) -> String {
        format!("{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",self.id, self.calle, self.numPiso, self.cp, self.m2, self.numBanos, self.numHabitaciones, self.tipoVivienda)
    }
}

pub struct ViviendaDAO {
    indice : HashMap<String,Vivienda>
}

impl ScreenOutput for ViviendaDAO {
    fn toScreen(&self) -> String {
        format!("{:?}",self.indice)
    }
}

impl ViviendaDAO {

    pub fn new() -> ViviendaDAO {
        let mut p = ViviendaDAO { indice : HashMap::new() };

        p.refresh();
        p
    }

    pub fn refresh(&mut self) {
        let path_csv =  Path::new("./src/files/viviendas.csv");
        let file_csv = File::open(path_csv).expect("Cannot read csv file");
        let mut rdr = csv::Reader::from_reader(file_csv);
        
        for result in rdr.deserialize() {
            let v : Vivienda= result.expect("Error in deserialize");
            self.indice.insert(v.clone().id, v); 
        }
    }

    pub fn save_state(&self) {
        let datos = self.indice.values().cloned().collect::<Vec<Vivienda>>();
    
        self.save(&datos);
    }

    pub fn save(&self, datos : &Vec<Vivienda>) {
        // let path_json =  Path::new("./src/files/viviendas.json");
        let path_csv =  Path::new("./src/files/viviendas.csv");
        let file = File::create(path_csv).expect("Cannot read");
        let mut wtr = csv::Writer::from_writer(file);

        for dato in datos {
            wtr.serialize(&dato);
        }
        wtr.flush();
        // // std::fs::write(
        // //     path_json,
        // //     serde_json::to_string_pretty(&datos).unwrap(),
        // // )
        // // .unwrap();        

        // let mut wtr = csv::Writer::from_path(path_csv);
        // // for dato in datos {
        // //     wtr.serialize(&dato)?;
        // // }
        // // wtr.flush();
        // // Ok(());

        // wtr.serialize(Record {
        //     city: "Southborough".to_string(),
        //     region: "MA".to_string(),
        //     country: "United States".to_string(),
        //     population: Some(9686),
        // })?;
        // wtr.serialize(Record {
        //     city: "Northbridge".to_string(),
        //     region: "MA".to_string(),
        //     country: "United States".to_string(),
        //     population: Some(14061),
        // })?;
        // wtr.flush()?;
    }


    fn example() -> Result<(), Box<dyn Error>> {
        // let mut wtr = csv::Writer::from_writer(io::stdout());
    
        let mut wtr = csv::Writer::from_path("./examples/prueba.csv")?;
    
        // When writing records with Serde using structs, the header row is written
        // automatically.
        // wtr.serialize(Record {
        //     city: "Southborough".to_string(),
        //     region: "MA".to_string(),
        //     country: "United States".to_string(),
        //     population: Some(9686),
        // })?;
        // wtr.serialize(Record {
        //     city: "Northbridge".to_string(),
        //     region: "MA".to_string(),
        //     country: "United States".to_string(),
        //     population: Some(14061),
        // })?;
        wtr.flush()?;
        Ok(())
    }

    pub fn save_and_refresh(&mut self, datos: &Vec<Vivienda>) {
        self.save(&datos);
        self.refresh();
    }


    pub fn asVector(&self) -> Vec<Vivienda> {
        let datos = self.indice.values().cloned().collect::<Vec<Vivienda>>();
        datos
    }

    pub fn add(&mut self, p : Vivienda) {
        if !self.indice.contains_key(&p.id) {
            self.indice.insert(p.clone().id, p);
        }
    } 

    pub fn update(&mut self, p : Vivienda) {
        if self.indice.contains_key(&p.id) {
            self.indice.insert(p.clone().id, p);
        }
    } 

    pub fn remove(&mut self, key : &String) -> Option<Vivienda> {
        self.indice.remove(key)
    }  



}
