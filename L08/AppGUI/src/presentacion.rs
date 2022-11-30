#![allow(non_snake_case)]
// use std::{io::SeekFrom, intrinsics::caller_location};
use std::{io::SeekFrom, os::unix::prelude};


use fltk::{
    app::{self, App}, enums,
    prelude::{GroupExt, WidgetExt},
    window::{self, DoubleWindow}, button::Button, menu::{Choice, MenuItem}, 
};
use fltk_table::{SmartTable, TableOpts};

use fltk::{app::*, browser::*, enums::*, input::*, prelude::*, window::*};
use serde::__private::de;

const WIDGET_WIDTH: i32 = 100;
const WIDGET_HEIGHT: i32 = 25;
const WIDGET_PADDING: i32 = 10;

#[derive(Clone, Copy)]
enum Message {
    Create,
    Update,
    Delete,
    Select,
    Filter,
    Save,
}

#[derive(Debug)]
enum TipoVivienda {
    Piso,
    Casa,
    Trastero,
    Local
}



impl TipoVivienda {
    fn as_str(&self) -> &'static str {
        match self {
            TipoVivienda::Piso => "Piso",
            TipoVivienda::Casa => "Casa",
            TipoVivienda::Trastero => "Trastero",
            TipoVivienda::Local => "Local"
        }
    }
}


use crate::entidad::{Vivienda, ScreenOutput};
use crate::entidad::ViviendaDAO;

pub struct GUI{
    app : App,
    wind : DoubleWindow,
    sender : Sender<Message>,
    receiver : Receiver<Message>,
    model : Vec<Vivienda>,
    viviendaDAO : ViviendaDAO,
    filter_input : Input,
    list_browser : HoldBrowser,
    id_input : Input,
    calle_input : Input,
    numPis_input : Input,
    cp_input : Input,
    m2_input : Input,
    numBanos_input : Input,
    numHabitaciones_input : Input,
    tipoVivienda_input : Choice,
    create_button : Button,
    update_button : Button,
    delete_button : Button,
    save_button : Button
}

impl GUI {
    
    pub fn new() -> GUI {
        let mut app = app::App::default().with_scheme(app::Scheme::Gtk);
        let mut wind = Window::default().with_label("CRUD");
        let (sender, receiver) = channel::<Message>();

        let mut filter_input = Input::default().with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(WIDGET_PADDING + WIDGET_WIDTH * 2, WIDGET_PADDING)
        .with_label("Filter prefix:");

        let mut list_browser = HoldBrowser::default()
        .with_pos(
            WIDGET_PADDING,
            filter_input.y() + filter_input.height() + WIDGET_PADDING,
        )
        .with_size(filter_input.width() * 6, filter_input.height() * 10);

        let id_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(
            list_browser.x() + list_browser.width() + WIDGET_PADDING + WIDGET_WIDTH,
            list_browser.y(),
        )
        .with_label("Id:");

        let calle_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&id_input, WIDGET_PADDING)
        .with_label("Calle:");

        let numPis_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&calle_input, WIDGET_PADDING)
        .with_label("Número Piso:");

        let cp_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&numPis_input, WIDGET_PADDING)
        .with_label("CP:");

        let m2_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&cp_input, WIDGET_PADDING)
        .with_label("m2:");

        let numBanos_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&m2_input, WIDGET_PADDING)
        .with_label("Baños:");

        let numHabitaciones_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&numBanos_input, WIDGET_PADDING)
        .with_label("Habitaciones:");


        let mut tipoVivienda_input = Choice::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&numHabitaciones_input, WIDGET_PADDING)
        .with_label("Tipo Vivienda:");
        // tipoVivienda_input.add_choice("Choice 1| Choice 2| choice 3");

        let piso = TipoVivienda::Piso;
        let strPiso = format!("{:?}", piso);
        let casa = TipoVivienda::Casa;
        let strCasa = format!("{:?}", casa);
        let local = TipoVivienda::Local;
        let strLocal = format!("{:?}", local);
        let trastero = TipoVivienda::Trastero;
        let strTrastero = format!("{:?}", trastero);
        
        tipoVivienda_input.add_choice("No definido");
        tipoVivienda_input.add_choice(&strPiso.to_string());
        tipoVivienda_input.add_choice(&strCasa.to_string());
        tipoVivienda_input.add_choice(&strTrastero.to_string());
        tipoVivienda_input.add_choice(&strLocal.to_string());

        let mut create_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .with_pos(
                WIDGET_PADDING,
                list_browser.y() + list_browser.height() + WIDGET_PADDING,
            )
            .with_label("Crear");

        let mut update_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&create_button, WIDGET_PADDING)
            .with_label("Modificar");

        let mut delete_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&update_button, WIDGET_PADDING)
            .with_label("Borrar");

        let mut save_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&delete_button, WIDGET_PADDING)
            .with_label("Guardar");

        let viviendaDAO = ViviendaDAO::new();
        let model = viviendaDAO.asVector();
        
        GUI {
            app : app,
            wind : wind,
            sender : sender,
            receiver : receiver,
            filter_input : filter_input,
            list_browser : list_browser,
            viviendaDAO : viviendaDAO,
            model : model,
            id_input : id_input,
            calle_input : calle_input,
            numPis_input : numPis_input,
            cp_input : cp_input,
            m2_input : m2_input,
            numBanos_input : numBanos_input,
            numHabitaciones_input : numHabitaciones_input,
            tipoVivienda_input : tipoVivienda_input,
            create_button : create_button,
            update_button : update_button,
            delete_button : delete_button,
            save_button : save_button
        }
    }

    pub fn build(&mut self) {
        self.filter_input.set_trigger(CallbackTrigger::Changed);
        self.filter_input.emit(self.sender, Message::Filter);

        self.list_browser.emit(self.sender, Message::Select);        

        self.create_button.emit(self.sender, Message::Create);

        self.update_button.emit(self.sender, Message::Update);
        self.update_button.deactivate();

        self.delete_button.emit(self.sender, Message::Delete);
        self.delete_button.deactivate();

        self.save_button.emit(self.sender, Message::Save);

        self.wind.set_size(
            self.id_input.x() + self.id_input.width() + WIDGET_PADDING,
            self.create_button.y() + self.create_button.height() + WIDGET_PADDING,
        );

        self.sender.send(Message::Filter);

    }

    fn clear_edit(&mut self) {
        self.id_input.set_value("");
        self.calle_input.set_value("");
        self.numPis_input.set_value("");
        self.cp_input.set_value("");
        self.m2_input.set_value("");
        self.numBanos_input.set_value("");
        self.numHabitaciones_input.set_value("");
        self.tipoVivienda_input.set_value(0);
    }

    pub fn show(&mut self) {
        self.wind.end();
        self.wind.show();
        while self.app.wait() {
            match self.receiver.recv() {
                Some(Message::Create) => {
                    self.model.push(Vivienda { 
                        id: self.id_input.value(),
                        calle: self.calle_input.value(),
                        numPiso: self.numPis_input.value(), 
                        cp: self.cp_input.value(), 
                        m2: self.m2_input.value(), 
                        numBanos: self.numBanos_input.value(), 
                        numHabitaciones: self.numHabitaciones_input.value(),
                        tipoVivienda: self.tipoVivienda_input.value().to_string()
                    });
                    self.clear_edit();
                    self.sender.send(Message::Filter);
                }
                Some(Message::Update) => {
                    if self.list_browser.value() > 0 {
                        let text_selection = self.list_browser.text(self.list_browser.value()).unwrap();
                        let search_result = self.model.iter_mut().filter(|e| {
                            return e.toScreen().eq_ignore_ascii_case(&text_selection)
                        }).next();
                        match search_result {
                            Some(vivienda) => {
                                vivienda.id = self.id_input.value();
                                vivienda.calle = self.calle_input.value();
                                vivienda.numPiso = self.numPis_input.value();
                                vivienda.cp = self.cp_input.value();
                                vivienda.m2 = self.m2_input.value();
                                vivienda.numBanos = self.numBanos_input.value();
                                vivienda.numHabitaciones = self.numHabitaciones_input.value();
                                let tipoV= self.tipoVivienda_input.value();
                                match tipoV {
                                    0 => vivienda.tipoVivienda = "Piso".to_string(),
                                    1 => vivienda.tipoVivienda = "Casa".to_string(),
                                    2 => vivienda.tipoVivienda = "Trastero".to_string(),
                                    3 => vivienda.tipoVivienda = "Local".to_string(),
                                    _ => vivienda.tipoVivienda = "No definido".to_string()
                                }
                                // vivienda.tipoVivienda = self.tipoVivienda_input.value().to_string();

                                self.clear_edit();
                                self.sender.send(Message::Filter);
                                self.sender.send(Message::Select);
                            },
                            _ => {
                                println!("ELEMENTO NO ENCONTRADO!!!");
                            } 
                        }
                    } else {
                        println!("NO HAY ELEMENTO PARA MODIFICAR!!!");
                    }
                }
                Some(Message::Delete) => {
                    if self.list_browser.value() > 0 {
                        let text_selection = self.list_browser.text(self.list_browser.value()).unwrap();
                        let search_result = self.model.iter().enumerate().filter(|e| {
                            return e.1.toScreen().eq_ignore_ascii_case(&text_selection)
                        }).next();
                        match search_result {
                            Some((index,vivienda)) => {
                                self.model.remove(index);
                                self.clear_edit();
                                self.sender.send(Message::Filter);
                                self.sender.send(Message::Select);
                            },
                            _ => {
                                println!("ELEMENTO NO ENCONTRADO!!!");
                            } 
                        }
                    } else {
                        println!("NO HAY ELEMENTO PARA ELIMINAR!!!");
                    }
                }
                Some(Message::Save) => {
                    self.viviendaDAO.save_and_refresh(&self.model);
                    self.model = self.viviendaDAO.asVector();
                    self.clear_edit();
                    self.sender.send(Message::Filter);
                    self.sender.send(Message::Select);
                }
                Some(Message::Select) => {
                    if self.list_browser.value() == 0 {
                        self.update_button.deactivate();
                        self.delete_button.deactivate();
                    } else {
                        let text_selection = self.list_browser.text(self.list_browser.value()).unwrap();
                        let search_result = self.model.iter().filter(|e| {
                            return e.toScreen().eq_ignore_ascii_case(&text_selection)
                        }).next();

                        match search_result {
                            Some(vivienda) => {
                                self.id_input.set_value(&vivienda.id);
                                self.calle_input.set_value(&vivienda.calle);
                                self.numPis_input.set_value(&vivienda.numPiso);
                                self.cp_input.set_value(&vivienda.cp);
                                self.m2_input.set_value(&vivienda.m2);
                                self.numBanos_input.set_value(&vivienda.numBanos);
                                self.numHabitaciones_input.set_value(&vivienda.numHabitaciones);
                                
                                let tipoV: String= vivienda.tipoVivienda.to_owned().to_string();
                                
                                match tipoV.as_str() {
                                    "Piso" => self.tipoVivienda_input.set_value(1),
                                    "Casa" => self.tipoVivienda_input.set_value(2),
                                    "Local" => self.tipoVivienda_input.set_value(3),
                                    "Trastero" => self.tipoVivienda_input.set_value(4),
                                    _ => self.tipoVivienda_input.set_value(0),
                                };

                                self.update_button.activate();
                                self.delete_button.activate();
                            },
                            _ => {
                                println!("ELEMENTO NO ENCONTRADO!!!");
                            } 
                        }                        
                    }
                }
                Some(Message::Filter) => {
                    let prefix = self.filter_input.value().to_lowercase();
                    let filter_empty = prefix.trim().eq_ignore_ascii_case("");
                    self.list_browser.clear();
                    for (i,p) in self.model.iter().enumerate() {
                        if (p.id.eq_ignore_ascii_case(prefix.as_str()) && !filter_empty) || (filter_empty)  {
                            let item = p.toScreen();
                            self.list_browser.add(&item);    
                        }
                    }                                 
                    self.sender.send(Message::Select);    
                }
                None => {},
                _ => {}
            }
        }
    }
    
    pub fn refresh(&mut self, data : Vec<Vivienda>) {
        for (i,p) in data.iter().enumerate() {
            println!("{} {:?} ",i, p);
        }    
    }

}
