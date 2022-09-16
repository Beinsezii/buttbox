use serde::Deserialize;
use std::{env::args, fs::File, io::Read, process::Command};

mod front_egui;

pub struct ButtSets {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub commands: Vec<(String, Command)>,
    pub wrap: usize,
    pub butt_width: f32,
    pub butt_height: f32,
}

#[derive(Deserialize)]
struct ButtSer {
    commands: Vec<(String, String, Vec<String>)>,
    fg: Option<String>,
    bg: Option<String>,
    wrap: Option<usize>,
    butt_width: Option<f32>,
    butt_height: Option<f32>,
}

impl ButtSer {
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
    let mut buf = String::new();
    args()
        .last()
        .map(|p| File::open(p).ok())
        .flatten()
        .expect("No TOML provided!")
        .read_to_string(&mut buf)
        .expect("Could not read provided file!");

    let buttsets: ButtSets = toml::de::from_str::<ButtSer>(&buf)
        .expect("Invalid/malformed TOML!")
        .into_buttsets();

    let native_options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(
            (
                buttsets.wrap.min(buttsets.commands.len()) as f32 * (buttsets.butt_width + 1.0)
                    - 1.0,
                (buttsets.commands.len() as f32 / buttsets.wrap as f32).ceil()
                    * (buttsets.butt_height + 1.0)
                    - 1.0,
            )
                .into(),
        ),
        ..Default::default()
    };

    eframe::run_native(
        "Pixelbuster GUI",
        native_options,
        Box::new(|cc| Box::new(front_egui::ButtBox::new(cc, buttsets))),
    );
}
