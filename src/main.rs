use serde::{Deserialize, Serialize};
use std::{process::Command, fs::File, env::args};

mod front_egui;

pub struct ButtSets {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub commands: Vec<(String, Command)>,
    pub wrap: usize,
}

#[derive(Serialize, Deserialize)]
struct ButtJson {
    commands: Vec<(String, String, Vec<String>)>,
    fg: Option<String>,
    bg: Option<String>,
    wrap: Option<usize>,
}

impl ButtJson {
    fn into_buttsets(self) -> ButtSets {
        ButtSets {
            fg: self.fg,
            bg: self.bg,
            wrap: self.wrap.unwrap_or(0),
            commands: self
                .commands
                .into_iter()
                .map(|(name, com, args)| {
                    let mut command = Command::new(com);
                    command.args(args);
                    (name, command)
                })
                .collect(),
        }
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();

    let buttsets = args().last().map(|p| File::open(p).ok()).flatten().expect("No JSON provided!");
    let buttsets = serde_json::from_reader::<_, ButtJson>(buttsets).expect("Bad JSON").into_buttsets();

    eframe::run_native(
        "Pixelbuster GUI",
        native_options,
        Box::new(|cc| Box::new(front_egui::ButtBox::new(cc, buttsets))),
    );
}
