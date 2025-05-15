use egui::*; 


const BG_COLOR: Color32 = Color32::from_rgb(54, 69, 79);
const BG_COLOR2: Color32 = Color32::from_rgb(126, 125, 156);
const HOVER_BG_COLOR: Color32 = Color32::from_rgb(245, 245, 245);
const LINE_COLOR: Color32 = Color32::WHITE;

pub struct BitWidget {
    on: bool,
}

impl BitWidget {
    pub const fn new(on: bool) -> Self {
        Self { on }
    }

    pub const fn new_on() -> Self {
        Self { on: true }
    }

    pub const fn on(self) -> bool {
        self.on
    }

    pub const fn new_off() -> Self {
        Self { on: false }
    }
}

impl Widget for BitWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, resp) = ui.allocate_exact_size(vec2(16.0, 20.0), Sense::click());
        let painter = ui.painter();
        let bg_color = if resp.hovered() {
            Color32::from_rgb(98, 93, 93)
        } else {
            Color32::from_rgb(54, 69, 79)
        };
        painter.rect_filled(rect, CornerRadius::ZERO, bg_color);
        painter.rect_stroke(rect, CornerRadius::ZERO, Stroke::new(1.0, Color32::WHITE), StrokeKind::Inside);
        painter.text(rect.center(), Align2::CENTER_CENTER, if self.on { "1" } else { "0" }, FontId::monospace(16.0), Color32::WHITE);
        resp
    }
}

pub struct MaskWidget<'a, T> {
    mask: T,
    interact_index: Option<&'a mut u32>,
}

impl<'a, T> MaskWidget<'a, T> {
    pub fn new(mask: T) -> Self {
        Self { 
            mask,
            interact_index: None,
        }
    }

    pub fn new_tracked(mask: T, click_index: &'a mut u32) -> Self {
        Self {
            mask,
            interact_index: Some(click_index),
        }
    }
}

macro_rules! mask_widget_impl {
    ($type:ty) => {
        mask_widget_impl!(@switch; mut $type);
        mask_widget_impl!(@switch; const $type);
    };
    (@generic; mut $type:ty) => {
        &'a mut $type
    };
    (@generic; const $type:ty) => {
        $type
    };
    (@on; mut $type:ty, $self_mask:expr, $bit_mask:expr) => {
        (*$self_mask & $bit_mask) != 0
    };
    (@on; const $type:ty, $self_mask:expr, $bit_mask:expr) => {
        ($self_mask & $bit_mask) != 0
    };
    (@click; mut $type:ty, $bit_resp:expr, $self_mask:expr, $bit_mask:expr, $resp:expr) => {
        if $bit_resp.clicked() {
            (*$self_mask) ^= $bit_mask;
            $resp.mark_changed();
        }
    };
    (@click; const $type:ty, $bit_resp:expr, $self_mask:expr, $bit_mask:expr, $resp:expr) => {};
    (@sense; mut $type:ty) => {
        Sense::click()
    };
    (@sense; const $type:ty) => {
        Sense::hover()
    };
    (@switch; $mutability:ident $type:ty) => {
        impl<'a> Widget for MaskWidget<'a, mask_widget_impl!(@generic; $mutability $type)> {
            fn ui(self, ui: &mut Ui) -> Response {
                const BITS: u32 = <$type>::BITS;
                const MAX_BIT_INDEX: u32 = BITS - 1;
                const LEFT_MASK: $type = 1 << MAX_BIT_INDEX;
                const BITS_F: f32 = BITS as f32;
                const SIZE: Vec2 = vec2(BITS_F * 16.0 + 2.0, 24.0);

                let (outer_rect, mut resp) = ui.allocate_exact_size(SIZE, Sense::empty());
                let inner_rect = outer_rect.shrink(1.0);
                let painter = ui.painter_at(outer_rect);
                // Fill with red color so it's easy to tell when it isn't covered.
                painter.rect_filled(inner_rect, CornerRadius::ZERO, Color32::RED);
                painter.rect_stroke(outer_rect, CornerRadius::ZERO, Stroke::new(1.0, LINE_COLOR), StrokeKind::Inside);

                let mut mask = LEFT_MASK;

                let mut _ci = 0u32;
                let interact_index = self.interact_index.unwrap_or_else(|| &mut _ci);
                for i in 0..BITS {
                    let bit_index = MAX_BIT_INDEX - i;
                    let on = mask_widget_impl!(@on; $mutability $type, self.mask, mask);
                    let x = (i * 16) as f32;
                    let x = inner_rect.left() + x;
                    let y = inner_rect.top();
                    let bit_rect = Rect::from_min_size(pos2(x, y), vec2(16.0, 22.0));
                    let bit_resp = ui.allocate_rect(bit_rect, mask_widget_impl!(@sense; $mutability $type));

                    let fourth = bit_index / 4;
                    let even = (fourth & 1) == 0;
                    let (bg_color, text_color) = if bit_resp.hovered() {
                        mask_widget_impl!(@click; $mutability $type, bit_resp, self.mask, mask, resp);
                        *interact_index = bit_index;
                        (
                            HOVER_BG_COLOR,
                            Color32::BLACK,
                        )
                    } else {
                        (
                            if even { BG_COLOR } else { BG_COLOR2 },
                            LINE_COLOR,
                        )
                    };
                    resp = resp.union(bit_resp);

                    painter.rect_filled(bit_rect, CornerRadius::ZERO, bg_color);
                    let bit_chr = if on {
                        let bottom_rect = Rect::from_min_size(pos2(bit_rect.left(), bit_rect.bottom() - 2.0), vec2(16.0, 4.0));
                        painter.rect_filled(bottom_rect, CornerRadius::ZERO, Color32::RED);
                        let bit_chr = "1";
                        painter.text(
                            bit_rect.center() + vec2(1.0, 1.0),
                            Align2::CENTER_CENTER,
                            bit_chr,
                            FontId::monospace(16.0),
                            Color32::RED,
                        );
                        bit_chr
                    } else {
                        "0"
                    };
                    painter.text(
                        bit_rect.center(),
                        Align2::CENTER_CENTER,
                        bit_chr,
                        FontId::monospace(16.0),
                        text_color,
                    );

                    mask >>= 1;
                }
                resp
            }
        }
    };
}

mask_widget_impl!(u8);
mask_widget_impl!(u16);
mask_widget_impl!(u32);
mask_widget_impl!(u64);
mask_widget_impl!(i8);
mask_widget_impl!(i16);
mask_widget_impl!(i32);
mask_widget_impl!(i64);

pub trait MaskWidgetUIExt {
    fn bitmask<'a, T>(&mut self, mask: T) -> Response
    where MaskWidget<'a, T>: Widget;
    fn tracked_bitmask<'a, T>(&mut self, mask: T, click_index: &'a mut u32) -> Response
    where MaskWidget<'a, T>: Widget;
}

impl MaskWidgetUIExt for Ui {
    fn bitmask<'a, T>(&mut self, mask: T) -> Response
        where MaskWidget<'a, T>: Widget {
        self.add(MaskWidget::new(mask))
    }
    fn tracked_bitmask<'a, T>(&mut self, mask: T, click_index: &'a mut u32) -> Response
        where MaskWidget<'a, T>: Widget {
        self.add(MaskWidget::new_tracked(mask, click_index))
    }
}