use bracket_pathfinding::prelude::*;
use bracket_terminal::prelude::*;

use daedalus::prelude::*;
use labyrinth_map::prelude::*;

struct State {
    mapbuilder: MapGenerator2D,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // draw current map
        draw_map(&self.mapbuilder.map(), ctx);
        draw_panel(ctx);

        // process user input
        let mut input = INPUT.lock();

        while let Some(ev) = input.pop() {
            match ev {
                BEvent::Character { c } => process_character(self, c),
                BEvent::CloseRequested => ctx.quit(),
                _ => (),
            }
        }
    }
}

fn process_character(gs: &mut State, c: char) {
    // TODO: this should call different actions on the mapbuilder, allowing
    //       for a demo of each of the main methods in it
    println!("{:?}", c);
    match c {
        'n' => gs.mapbuilder.flush_map(),
        'w' => gs.mapbuilder.walled_map(),
        _ => {}
    }
}

fn draw_panel(ctx: &mut BTerm) {
    ctx.draw_hollow_box_double(51, 1, 28, 48, RGBA::named(WHITE), RGBA::new());

    ctx.print(52, 4, "Controls");
    ctx.print(52, 5, "n: new (filled) map");
    ctx.print(52, 6, "w: new (walled) map");
}

fn draw_map(map: &Labyrinth2D, ctx: &mut BTerm) {
    (0..map.size()).for_each(|idx| {
        let pt = map.index_to_point2d(idx);
        draw_tile(pt, map.tile_kind(pt), ctx);
    });
}

fn draw_tile(pt: Point, kind: &str, ctx: &mut BTerm) {
    let x = pt.x;
    let y = pt.y;
    let (glyph, fg, bg) = match kind {
        "wall" => ('#', RGBA::named(WHITE), RGBA::named(BLACK)),
        "floor" => (' ', RGBA::named(GRAY), RGBA::named(BLACK)),
        "water" => ('~', RGBA::named(LIGHT_BLUE), RGBA::named(BLUE)),
        "lava" => ('~', RGBA::named(ORANGE), RGBA::named(YELLOW)),
        "chasm" => (' ', RGBA::named(BLACK), RGBA::named(DARK_BLUE)),
        _ => ('?', RGBA::named(RED), RGBA::named(RED)),
    };

    ctx.set(x, y, fg, bg, to_cp437(glyph));
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("mapenerator Demo/Debug")
        .with_tile_dimensions(16, 16)
        .with_advanced_input(true)
        .build()?;

    let gs: State = State {
        mapbuilder: MapGenerator2D::new(50, 50),
    };

    main_loop(context, gs)
}
