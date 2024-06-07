use super::ButtSets;
use eframe::{
    egui::{
        style::{Spacing, WidgetVisuals, Widgets},
        Button, CentralPanel, Color32, Context, Frame, Grid, Key, RichText, Stroke, Style, ViewportCommand, Visuals,
    },
    epaint::{FontId, Rounding, Shadow},
    App,
};

pub struct ButtBox {
    butts: ButtSets,
    sel: usize,
    psel: usize,
}

impl App for ButtBox {
    // {{{
    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        Color32::from_rgb(self.butts.bg[0], self.butts.bg[1], self.butts.bg[2])
            .gamma_multiply(self.butts.opacity)
            .to_normalized_gamma_f32()
    }
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().frame(Frame::window(&ctx.style())).show(ctx, |ui| {
            Grid::new("Butts").show(ui, |ui| {
                let mut run = None;
                let mut rids = Vec::new();

                for (n, b) in self.butts.commands.iter().enumerate() {
                    if self.butts.wrap == 0 {
                    } else if n % self.butts.wrap == 0 && n != 0 {
                        ui.end_row()
                    }

                    let res = ui.add_sized(
                        (self.butts.butt_width * self.butts.scale, self.butts.butt_height * self.butts.scale),
                        Button::new(RichText::from(&b.0).font(FontId::proportional(self.butts.font_size * self.butts.scale))),
                    );

                    // override sel if focus changed
                    ui.memory(|u| {
                        if u.has_focus(res.id) {
                            if self.psel != n {
                                self.sel = n
                            }
                        }
                    });

                    if res.clicked() {
                        run = Some(n)
                    };

                    rids.push(res);
                }

                rids.into_iter().enumerate().for_each(|(n, r)| {
                    ui.memory_mut(|m| {
                        if n == self.sel {
                            m.request_focus(r.id)
                        }
                    })
                });

                self.psel = self.sel;

                if let Some(n) = run {
                    self.run(n, ctx)
                }
            });
        });

        ctx.input(|i| {
            if i.key_pressed(Key::ArrowRight) {
                self.right()
            } else if i.key_pressed(Key::ArrowLeft) {
                self.left()
            } else if i.key_pressed(Key::ArrowDown) {
                self.down()
            } else if i.key_pressed(Key::ArrowUp) {
                self.up()
            }
        });
    }
} // }}}

impl ButtBox {
    // {{{
    pub fn new(cc: &eframe::CreationContext<'_>, butts: ButtSets) -> Self {
        let rounding = Rounding::ZERO;
        let style = cc.egui_ctx.style().as_ref().clone();
        let s = butts.scale;
        let fg = Color32::from_rgb(butts.fg[0], butts.fg[1], butts.fg[2]);
        let bg = Color32::from_rgb(butts.bg[0], butts.bg[1], butts.bg[2]);
        let fgs = Stroke {
            color: fg,
            width: butts.butt_stroke * s,
        };
        let bgs = Stroke {
            color: bg,
            width: butts.butt_stroke * s,
        };
        cc.egui_ctx.set_style(Style {
            wrap: Some(true),
            visuals: Visuals {
                widgets: Widgets {
                    active: WidgetVisuals {
                        rounding,
                        bg_stroke: bgs,
                        fg_stroke: bgs,
                        bg_fill: fg.gamma_multiply(butts.opacity),
                        weak_bg_fill: fg.gamma_multiply(butts.opacity),
                        ..style.visuals.widgets.active
                    },
                    inactive: WidgetVisuals {
                        rounding,
                        bg_stroke: Stroke {
                            width: butts.butt_stroke * s,
                            color: fg.gamma_multiply(0.5),
                        },
                        fg_stroke: fgs,
                        bg_fill: bg.gamma_multiply(butts.opacity),
                        weak_bg_fill: bg.gamma_multiply(butts.opacity),
                        ..style.visuals.widgets.inactive
                    },
                    hovered: WidgetVisuals {
                        rounding,
                        bg_stroke: fgs,
                        fg_stroke: fgs,
                        bg_fill: bg.gamma_multiply(butts.opacity),
                        weak_bg_fill: bg.gamma_multiply(butts.opacity),
                        ..style.visuals.widgets.hovered
                    },
                    ..style.visuals.widgets
                },
                window_rounding: rounding,
                window_fill: Color32::TRANSPARENT,
                window_shadow: Shadow::NONE,
                window_stroke: Stroke::NONE,
                ..style.visuals
            },
            spacing: Spacing {
                item_spacing: (butts.butt_stroke * s, butts.butt_stroke * s).into(),
                window_margin: (butts.butt_stroke * s * 0.5).into(),
                button_padding: (0.0, 0.0).into(),
                // menu_margin: 0.0.into(),
                // indent: 0.0,
                // interact_size: (0.0, 0.0).into(),
                // slider_width: 0.0,
                // combo_width: 0.0,
                // text_edit_width: 0.0,
                // icon_width: 0.0,
                // icon_width_inner: 0.0,
                // icon_spacing: 0.0,
                // tooltip_width: 0.0,
                // indent_ends_with_horizontal_line: false,
                // combo_height: 0.0,
                // scroll_bar_width: 0.0,
                // scroll_handle_min_length: 0.0,
                // scroll_bar_inner_margin:  0.0,
                // scroll_bar_outer_margin:  0.0,
                ..style.spacing
            },
            ..style
        });

        Self { butts, sel: 0, psel: 0 }
    }

    fn right(&mut self) {
        self.sel = (self.sel + 1)
            .min(self.butts.commands.len() - 1)
            .min((self.sel.checked_div(self.butts.wrap).unwrap_or(0) + 1) * self.butts.wrap - 1);
    }
    fn left(&mut self) {
        self.sel = self
            .sel
            .saturating_sub(1)
            .max(self.sel.checked_div(self.butts.wrap).unwrap_or(0) * (self.butts.wrap));
    }
    fn down(&mut self) {
        let n = self.sel + self.butts.wrap;
        if n < self.butts.commands.len() {
            self.sel = n
        };
    }
    fn up(&mut self) {
        if self.sel >= self.butts.wrap {
            self.sel -= self.butts.wrap
        };
    }
    fn run(&mut self, n: usize, ctx: &Context) {
        if !self.butts.commands[n].1.get_program().is_empty() {
            self.butts.commands[n]
                .1
                .spawn()
                .expect(&format!("Command {} failed", self.butts.commands[n].1.get_program().to_string_lossy(),));
        }
        ctx.send_viewport_cmd(ViewportCommand::Close)
    }
} // }}}
