const RAMP_W: u32 = 1;
const RAMP_H: u32 = 28;

fn draw_solid_ramp(canvas: Canvas) {
    const c: f32 = 1.0 / 512;
    const d: f32 = 1.0 / 256;

    // const struct {
    //     GColor  fC0, fDC;
    // } rec[] = {
    //     { GColor::MakeARGB(1,   c,   c,   c), GColor::MakeARGB(0,  d,  d,  d) },   // grey
    //     { GColor::MakeARGB(1, 1-c,   0,   0), GColor::MakeARGB(0, -d,  0,  0) },   // red
    //     { GColor::MakeARGB(1,   0,   c,   c), GColor::MakeARGB(0,  0,  d,  d) },   // cyan
    //     { GColor::MakeARGB(1,   0, 1-c,   0), GColor::MakeARGB(0,  0, -d,  0) },   // green
    //     { GColor::MakeARGB(1,   c,   0,   c), GColor::MakeARGB(0,  d,  0,  d) },   // magenta
    //     { GColor::MakeARGB(1,   0,   0, 1-c), GColor::MakeARGB(0,  0,  0, -d) },   // blue
    //     { GColor::MakeARGB(1,   c,   c,   0), GColor::MakeARGB(0,  d,  d,  0) },   // yellow
    // };

    
    // for (int y = 0; y < GARRAY_COUNT(rec); ++y) {
    //     GColor color = rec[y].fC0;
    //     GColor delta = rec[y].fDC;
    //     for (int x = 0; x < 256; x++) {
    //         const GRect rect = GRect::MakeXYWH(x * RAMP_W, y * RAMP_H, RAMP_W, RAMP_H);
    //         canvas->fillRect(rect, color);
    //         color.fA += delta.fA;
    //         color.fR += delta.fR;
    //         color.fG += delta.fG;
    //         color.fB += delta.fB;
    //     }
    // }
}

fn run_test() {
    unimplemented!()
}

fn main() {
    // Make bitmap
    let mut bitmap = RgbaImage::new(256 * RAMP_W, 7 * RAMP_H);

    let canvas = Canvas::new(&mut bitmap);
    
    draw_solid_ramp(&canvas);
}
