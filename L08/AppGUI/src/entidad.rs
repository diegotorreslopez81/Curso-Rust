use std::{path::Path, fs::{File, self}, collections::HashMap};

use serde::{Deserialize, Serialize};

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
    pub numHabitaciones: String
}



impl ScreenOutput for Vivienda {
    fn toScreen(&self) -> String {
        // format!("{:?},{:?},{:?}",self.identificacion,self.nombres,self.apellidos)
        format!("{:?},{:?},{:?},{:?},{:?},{:?},{:?}",self.id, self.calle, self.numPiso, self.cp, self.m2, self.numBanos, self.numHabitaciones)
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

    pub fn refresh(&mut self)  {
        let path_json =  Path::new("./src/json/viviendas.json");
        let data_str = fs::read_to_string(path_json).expect("Unable to read file");
        let viviendas : Vec<Vivienda> = serde_json::from_str(&data_str).expect("JSON does not have correct format.");
        self.indice.clear();
        for p in viviendas {
            self.indice.insert(p.clone().id,p);
        }
    }

    pub fn save_state(&self) {
        let datos = self.indice.values().cloned().collect::<Vec<Vivienda>>();
        self.save(&datos);
    }

    pub fn save(&self, datos : &Vec<Vivienda>) {
        let path_json =  Path::new("./src/json/viviendas.json");
        std::fs::write(
            path_json,
            serde_json::to_string_pretty(&datos).unwrap(),
        )
        .unwrap();        
    }

    pub fn save_and_refresh(&mut self, datos: &Vec<Vivienda>) {
        self.save(datos);
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
