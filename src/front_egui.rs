use super::ButtSets;
use eframe::{
    egui::{self, Button, CentralPanel, Color32, Context, Grid, Key, Stroke},
    App, Frame,
};

pub struct ButtBox {
    butts: ButtSets,
    sel: usize,
}

impl App for ButtBox {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default()
            .frame(
                egui::containers::Frame::window(&ctx.style())
                    .inner_margin(0.0)
                    .outer_margin(0.0),
            )
            .show(ctx, |ui| {
            Grid::new("Butts").spacing((1.0, 1.0)).show(ui, |ui| {
                let mut run = None;
                for (n, b) in self.butts.commands.iter().enumerate() {
                    if self.butts.wrap == 0 {
                    } else if n % self.butts.wrap == 0 && n != 0 {
                        ui.end_row()
                    }
                    let mut butt = Button::new(&b.0);
                    if n == self.sel {
                        butt = butt.stroke(Stroke {
                            width: 1.0,
                            color: Color32::WHITE,
                        });
                    }
                    if ui.add_sized((self.butts.butt_width, self.butts.butt_height), butt).clicked() {
                        run = Some(n)
                    };
                }
                if let Some(n) = run {
                    self.run(n, frame)
                }
            });
        });
        if ctx.input().key_pressed(Key::ArrowRight) {
            self.right()
        } else if ctx.input().key_pressed(Key::ArrowLeft) {
            self.left()
        } else if ctx.input().key_pressed(Key::ArrowDown) {
            self.down()
        } else if ctx.input().key_pressed(Key::ArrowUp) {
            self.up()
        } else if ctx.input().key_pressed(Key::Enter) {
            self.run(self.sel, frame)
        }
    }
}

impl ButtBox {
    pub fn new(cc: &eframe::CreationContext<'_>, butts: ButtSets) -> Self {
        Self { butts, sel: 0 }
    }

    fn right(&mut self) {
        self.sel = (self.sel + 1)
            .min(self.butts.commands.len() - 1)
            .min((self.sel.checked_div(self.butts.wrap).unwrap_or(0) + 1) * self.butts.wrap - 1)
    }
    fn left(&mut self) {
        self.sel = self
            .sel
            .saturating_sub(1)
            .max(self.sel.checked_div(self.butts.wrap).unwrap_or(0) * (self.butts.wrap))
    }
    fn down(&mut self) {
        let n = self.sel + self.butts.wrap;
        if n < self.butts.commands.len() {
            self.sel = n
        }
    }
    fn up(&mut self) {
        if self.sel >= self.butts.wrap {
            self.sel -= self.butts.wrap
        }
    }
    fn run(&mut self, n: usize, frame: &mut Frame) {
        self.butts.commands[n].1.spawn().expect(&format!(
            "Command {} failed",
            self.butts.commands[n].1.get_program().to_string_lossy(),
        ));
        frame.close();
    }
}
