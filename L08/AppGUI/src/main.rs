// use entidad::Vivienda;

use crate::entidad::ViviendaDAO;

mod entidad;
mod presentacion;

fn main() {
    let mut viviendaDAO = ViviendaDAO::new();
    let mut gui = presentacion::GUI::new();
    gui.build();
    gui.show();    
}