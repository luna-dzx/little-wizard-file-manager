use std::{
    fs,
    io::ErrorKind,
    path::Path,
    collections::BTreeMap
};
use notan::{
    prelude::*,
    draw::*
};

pub struct Theme {
    pub colors : BTreeMap<String,Color>
}
impl Theme{
    pub fn defaults() -> Theme {

        let mut colors:BTreeMap<String,Color> = BTreeMap::new();

        colors.insert("primary".to_string(),Color::from(0x24232eff));
        colors.insert("secondary".to_string(),Color::from(0x34324aff));
        colors.insert("button".to_string(),Color::from(0x66647dff));
        colors.insert("button_hover".to_string(),Color::from(0x8b89a1ff));
        colors.insert("button_click".to_string(),Color::from(0xb4b2cfff));


        Theme {
            colors
        }
    }

    fn to_toml(&self)->String{
        format!(
        "[Colors]\n\
        {}",
        self.colors.iter()
            .map(|(key,value)| format!("{} = 0x{:08x}\n",key,value.hex()))
            .collect::<String>())
    }

    pub fn load_color_str(&mut self, line: &str) {
        let line : Vec<&str> = line.split("=").collect();
        if let Ok(line_value) = line[1].parse::<u32>(){

            let entry = self.colors.entry(line[0].to_string()).or_insert(Color::from(line_value));
            *entry = Color::from(line_value);
        }
    }

    fn from_string(string:String)->Theme{
        let mut theme = Theme::defaults();
        let mut file_section = "";

        for line in string.lines(){
            let line = line.trim();

            // get section name
            if line.chars().next().unwrap() == '['{
                file_section = &line[1..line.len()-1];
                continue;
            }

            // based on section name, call appropriate load
            match file_section{
                "Colors"=> theme.load_color_str(line),
                _=>()
            }
        }

        theme
    }

    pub fn from_path(path:&str)->Theme{
        // attempt to load from file
        if let Ok(config_content) = fs::read_to_string(path) {
            println!("read");
            let theme = Theme::from_string(config_content);
            theme.save_to_path(path);
            return theme
        }

        println!("default thingy");
        // load default
        let theme = Theme::defaults();
        theme.save_to_path(path);
        theme
    }

    pub fn save_to_path(&self, path:&str){
        let toml_string = self.to_toml();
        fs::File::create(path).unwrap();
        fs::write(path,toml_string).unwrap();
    }
}