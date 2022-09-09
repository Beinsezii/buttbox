use super::ButtSets;
use eframe::{
    egui::{
        self,
        style::{Margin, Spacing, WidgetVisuals, Widgets},
        Button, CentralPanel, Color32, Context, Grid, Key, Stroke, Style, Visuals,
    },
    epaint::Rounding,
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
                Grid::new("Butts").show(ui, |ui| {
                    let mut run = None;

                    for (n, b) in self.butts.commands.iter().enumerate() {
                        if self.butts.wrap == 0 {
                        } else if n % self.butts.wrap == 0 && n != 0 {
                            ui.end_row()
                        }

                        let res = ui.add_sized(
                            (self.butts.butt_width, self.butts.butt_height),
                            Button::new(&b.0),
                        );

                        if n == self.sel {
                            ui.memory().request_focus(res.id)
                        }

                        if res.clicked() {
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
        }
    }
}

impl ButtBox {
    pub fn new(cc: &eframe::CreationContext<'_>, butts: ButtSets) -> Self {
        // General Style
        let rounding = Rounding::none();
        let bg_stroke = Stroke::none();
        let style = cc.egui_ctx.style().as_ref().clone();
        cc.egui_ctx.set_style(Style {
            visuals: Visuals {
                widgets: Widgets {
                    active: WidgetVisuals {
                        rounding,
                        bg_stroke,
                        ..style.visuals.widgets.active
                    },
                    inactive: WidgetVisuals {
                        rounding,
                        bg_stroke,
                        ..style.visuals.widgets.inactive
                    },
                    hovered: WidgetVisuals {
                        rounding,
                        ..style.visuals.widgets.hovered
                    },
                    noninteractive: WidgetVisuals {
                        rounding,
                        bg_stroke,
                        ..style.visuals.widgets.noninteractive
                    },
                    ..style.visuals.widgets
                },
                window_rounding: rounding,
                clip_rect_margin: 0.0,
                ..style.visuals
            },
            spacing: Spacing {
                item_spacing: (2.0, 2.0).into(),
                window_margin: Margin::same(0.0),
                ..style.spacing
            },
            ..style
        });

        // set FG
        if let Some(fg) = butts
            .fg
            .as_ref()
            .map(|fg| colcon::hex_to_irgb(fg).ok())
            .flatten()
        {
            let style = cc.egui_ctx.style().as_ref().clone();
            let fg = Color32::from_rgb(fg[0], fg[1], fg[2]);
            cc.egui_ctx.set_style(Style {
                visuals: Visuals {
                    widgets: Widgets {
                        active: WidgetVisuals {
                            bg_fill: fg,
                            ..style.visuals.widgets.active
                        },
                        inactive: WidgetVisuals {
                            fg_stroke: Stroke {
                                width: 1.0,
                                color: fg,
                            },
                            ..style.visuals.widgets.inactive
                        },
                        hovered: WidgetVisuals {
                            fg_stroke: Stroke {
                                width: 1.0,
                                color: fg,
                            },
                            bg_stroke: Stroke {
                                width: 1.0,
                                color: fg,
                            },
                            ..style.visuals.widgets.hovered
                        },
                        noninteractive: WidgetVisuals {
                            bg_fill: fg,
                            ..style.visuals.widgets.noninteractive
                        },
                        ..style.visuals.widgets
                    },
                    ..style.visuals
                },
                ..style
            });
        }

        // set BG
        if let Some(bg) = butts
            .bg
            .as_ref()
            .map(|bg| colcon::hex_to_irgb(bg).ok())
            .flatten()
        {
            let style = cc.egui_ctx.style().as_ref().clone();
            let bg = Color32::from_rgb(bg[0], bg[1], bg[2]);
            cc.egui_ctx.set_style(Style {
                visuals: Visuals {
                    widgets: Widgets {
                        active: WidgetVisuals {
                            fg_stroke: Stroke {
                                width: 1.0,
                                color: bg,
                            },
                            ..style.visuals.widgets.active
                        },
                        inactive: WidgetVisuals {
                            bg_fill: bg,
                            bg_stroke: Stroke {
                                width: 2.0,
                                color: bg,
                            },
                            ..style.visuals.widgets.inactive
                        },
                        hovered: WidgetVisuals {
                            bg_fill: bg,
                            ..style.visuals.widgets.hovered
                        },
                        ..style.visuals.widgets
                    },
                    ..style.visuals
                },
                ..style
            });
        }

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
