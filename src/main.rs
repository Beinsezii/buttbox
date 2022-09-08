use serde::{Deserialize, Serialize};
use std::{env::args, fs::File, process::Command};

mod front_egui;

pub struct ButtSets {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub commands: Vec<(String, Command)>,
    pub wrap: usize,
    butt_width: f32,
    butt_height: f32,
}

#[derive(Serialize, Deserialize)]
struct ButtJson {
    commands: Vec<(String, String, Vec<String>)>,
    fg: Option<String>,
    bg: Option<String>,
    wrap: Option<usize>,
    butt_width: Option<f32>,
    butt_height: Option<f32>,
}

impl ButtJson {
    fn into_buttsets(self) -> ButtSets {
        ButtSets {
            fg: self.fg,
            bg: self.bg,
            wrap: self.wrap.unwrap_or(0),
            butt_width: self.butt_width.unwrap_or(100.0),
            butt_height: self.butt_height.unwrap_or(100.0),
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
    let buttsets = args()
        .last()
        .map(|p| File::open(p).ok())
        .flatten()
        .expect("No JSON provided!");
    let buttsets = serde_json::from_reader::<_, ButtJson>(buttsets)
        .expect("Bad JSON")
        .into_buttsets();

    let native_options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(
            (
                buttsets.wrap.min(buttsets.commands.len()) as f32 * (buttsets.butt_width + 1.0) - 1.0,
                (buttsets.commands.len() as f32 / buttsets.wrap as f32).ceil() * (buttsets.butt_height + 1.0) - 1.0,
            ).into()
        ),
        ..Default::default()
    };

    eframe::run_native(
        "Pixelbuster GUI",
        native_options,
        Box::new(|cc| Box::new(front_egui::ButtBox::new(cc, buttsets))),
    );
}
