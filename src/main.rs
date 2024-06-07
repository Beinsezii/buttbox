use clap::{ArgAction, Parser};
use colcon::Space;
use serde::Deserialize;
use std::{env, path::PathBuf, process::Command};

mod front_egui;

fn col_parse(s: &str) -> Result<[u8; 3], String> {
    colcon::str2space(s, Space::SRGB)
        .map(|p| colcon::srgb_to_irgb(p))
        .ok_or_else(|| format!("Could not parse\n{}\nas a color in a known format", s))
}

fn scale_factor() -> f32 {
    if let Ok(val) = env::var("GDK_DPI_SCALE") {
        val.parse::<f32>().expect("Bad GDK_DPI_SCALE value")
    } else if let Ok(val) = env::var("GDK_SCALE") {
        val.parse::<f32>().expect("Bad GDK_SCALE value")
    } else {
        1.0
    }
}

#[derive(Deserialize)]
struct ButtSer {
    commands: Vec<(String, String, Vec<String>)>,
    fg: Option<String>,
    bg: Option<String>,
    opacity: Option<f32>,
    font_size: Option<f32>,
    wrap: Option<usize>,
    butt_width: Option<f32>,
    butt_height: Option<f32>,
    butt_stroke: Option<f32>,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None, disable_help_flag = true)]
struct ButtClap {
    /// Show this help
    #[arg(short='?', long, action=ArgAction::Help)]
    help: bool,
    /// TOML file to read from
    file: PathBuf,
    /// Override foreground color
    #[arg(long, value_parser=col_parse)]
    fg: Option<[u8; 3]>,
    /// Override foreground color
    #[arg(long, value_parser=col_parse)]
    bg: Option<[u8; 3]>,
    /// Override opacity
    #[arg(short = 'o', long)]
    opacity: Option<f32>,
    /// Override font size
    #[arg(short = 'f', long)]
    font_size: Option<f32>,
    /// Override butt wrap
    #[arg(long, short = 'W')]
    wrap: Option<usize>,
    /// Override butt width
    #[arg(short = 'w', long)]
    butt_width: Option<f32>,
    /// Override butt height
    #[arg(short = 'h', long)]
    butt_height: Option<f32>,
    /// Override butt stroke
    #[arg(short = 's', long)]
    butt_stroke: Option<f32>,
    /// Override environment scaling
    #[arg(long, short = 'S')]
    scale: Option<f32>,
}

pub struct ButtSets {
    commands: Vec<(String, Command)>,
    fg: [u8; 3],
    bg: [u8; 3],
    opacity: f32,
    font_size: f32,
    wrap: usize,
    butt_width: f32,
    butt_height: f32,
    butt_stroke: f32,
    scale: f32,
}

fn main() {
    // {{{
    let buttclap = ButtClap::parse();

    let buttser: ButtSer =
        toml::de::from_str(&std::fs::read_to_string(&buttclap.file).expect("Could not open TOML file!")).expect("Invalid/malformed TOML!");

    let commands: Vec<(String, Command)> = buttser
        .commands
        .into_iter()
        .map(|(name, com, args)| {
            let mut command = Command::new(com);
            command.args(args);
            (name, command)
        })
        .collect();

    let mut wrap = buttclap.wrap.unwrap_or(buttser.wrap.unwrap_or(0));
    if wrap == 0 {
        let square = (commands.len() as f32).sqrt().floor() as usize;
        for y in (2..=square).rev() {
            if commands.len() % y == 0 {
                wrap = commands.len() / y;
                break;
            }
        }
        if wrap == 0 {
            wrap = commands.len()
        }
    }

    let buttsets = ButtSets {
        commands,

        fg: buttclap.fg.unwrap_or(
            buttser
                .fg
                .map(|s| colcon::hex_to_irgb(&s).expect("Invalid foreground hex!"))
                .unwrap_or([255, 255, 255]),
        ),

        bg: buttclap.bg.unwrap_or(
            buttser
                .bg
                .map(|s| colcon::hex_to_irgb(&s).expect("Invalid foreground hex!"))
                .unwrap_or([0, 0, 0]),
        ),

        opacity: buttclap.opacity.unwrap_or(buttser.opacity.unwrap_or(1.0)).min(1.0).max(0.0),

        font_size: buttclap.font_size.unwrap_or(buttser.font_size.unwrap_or(12.0)),

        wrap,

        butt_width: buttclap.butt_width.unwrap_or(buttser.butt_width.unwrap_or(100.0)),

        butt_height: buttclap.butt_height.unwrap_or(buttser.butt_height.unwrap_or(100.0)),

        butt_stroke: buttclap.butt_stroke.unwrap_or(buttser.butt_stroke.unwrap_or(2.0)),

        scale: buttclap.scale.unwrap_or(scale_factor()),
    };

    let w = (buttsets.butt_width + buttsets.butt_stroke) * buttsets.scale;
    let h = (buttsets.butt_height + buttsets.butt_stroke) * buttsets.scale;

    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_active(true)
            .with_transparent(if buttsets.opacity < 1.0 { true } else { false })
            .with_window_level(eframe::egui::WindowLevel::AlwaysOnTop)
            .with_decorations(false)
            .with_inner_size((
                buttsets.wrap.min(buttsets.commands.len()) as f32 * w,
                (buttsets.commands.len() as f32 / buttsets.wrap as f32).ceil() * h,
            )),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Pixelbuster GUI",
        native_options,
        Box::new(|cc| Box::new(front_egui::ButtBox::new(cc, buttsets))),
    )
    .expect("EGUI exited in error");
} // }}}
