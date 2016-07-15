use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::{io, fs};
use rustc_serialize::json;
use discord;

pub type State = Arc<RwLock<StateData>>;

#[derive(RustcEncodable, RustcDecodable, Clone, Debug)]
pub struct StateData {
    pub musical_treats: HashMap<String, String>,
}

pub struct Bot {
    pub discord: Arc<discord::Discord>,
    pub connection: discord::Connection,
    pub ready: discord::model::ReadyEvent,
}

impl StateData {
    pub fn load() -> State {
        StateData::try_load().unwrap_or_else(|_| {
            println!("discord-settings.json did not exist.");
            println!("You should consider adding musical treats ;)");
            Arc::new(RwLock::new(StateData {
                musical_treats: HashMap::new(),
            }))
        })
    }

    pub fn save(&self) -> io::Result<()> {
        let data = json::encode(self).expect("Failed to serialize discord-settings.txt");
        let mut file = try!(StateData::get_file(true));
        try!(file.write_all(data.as_bytes()));
        Ok(())
    }

    fn try_load() -> io::Result<State> {
        let mut file = try!(StateData::get_file(false));
        let metadata = try!(file.metadata());

        let mut data = String::with_capacity(metadata.len() as usize);
        try!(file.read_to_string(&mut data));
        let data = json::decode(&data).expect("Invalid data in discord-settings.json");

        Ok(Arc::new(RwLock::new(data)))
    }

    fn get_file(write: bool) -> io::Result<fs::File> {
        let mut opts = fs::OpenOptions::new();
        if write { opts.create(true).truncate(true).write(true); }
        opts.open("discord-settings.json")
    }
}
