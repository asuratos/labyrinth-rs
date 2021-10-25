#[cfg(feature = "tools")]
use bracket_lib::prelude::*;

use bracket_geometry::prelude::Point;
use bracket_pathfinding::prelude::Algorithm2D;
use labyrinth_map::prelude::*;

enum TileType {
    Wall,
    Floor,
    Water,
    Lava,
    Chasm,
}

struct State {
    map: Labyrinth2D,
    brush_state: TileType,
    painting: bool,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // draw everything
        ctx.cls();
        ctx.draw_hollow_box_double(51, 1, 28, 48, RGBA::named(WHITE), RGBA::new());

        let tilename = match self.brush_state {
            TileType::Floor => "floor",
            TileType::Wall => "wall",
            TileType::Chasm => "chasm",
            TileType::Lava => "lava",
            TileType::Water => "water",
        };

        ctx.print(52, 2, format!("Click to set tile to {}", tilename));

        draw_map(&self.map, ctx);

        // wait for user input
        let mut input = INPUT.lock();

        while let Some(ev) = input.pop() {
            match ev {
                BEvent::MouseButtonDown { button: 0 } => self.painting = true,
                BEvent::MouseButtonUp { button: 0 } => self.painting = false,
                BEvent::CursorMoved { position: _ } if self.painting => paint_tile(self, ctx),
                BEvent::Character { c } => process_character(self, c),
                BEvent::CloseRequested => ctx.quit(),
                _ => (),
            }
        }
    }
}

fn process_character(gs: &mut State, c: char) {
    if let Some(newtile) = match c {
        '1' => Some(TileType::Wall),
        '2' => Some(TileType::Floor),
        '3' => Some(TileType::Water),
        '4' => Some(TileType::Lava),
        '5' => Some(TileType::Chasm),
        _ => None,
    } {
        gs.brush_state = newtile;
    }
}

fn paint_tile(gs: &mut State, ctx: &mut BTerm) {
    let loc = ctx.mouse_point();

    match gs.brush_state {
        TileType::Floor => gs.map.set_floor(loc),
        TileType::Wall => gs.map.set_wall(loc),
        TileType::Water => gs.map.set_water(loc),
        TileType::Lava => gs.map.set_lava(loc),
        TileType::Chasm => gs.map.set_chasm(loc),
        _ => (),
    }
}

fn draw_map(map: &Labyrinth2D, ctx: &mut BTerm) {
    // TODO: map iterator
    (0..map.tiles().len()).for_each(|idx| {
        let pt = map.index_to_point2d(idx);
        draw_tile(pt, map.tile_kind(pt), ctx);
    });
}

fn draw_tile(pt: Point, kind: &str, ctx: &mut BTerm) {
    let x = pt.x;
    let y = pt.y;
    let (glyph, fg, bg) = match kind {
        "wall" => ('#', RGBA::named(WHITE), RGBA::named(BLACK)),
        "floor" => ('.', RGBA::named(GRAY), RGBA::named(BLACK)),
        "water" => ('~', RGBA::named(LIGHT_BLUE), RGBA::named(BLUE)),
        "lava" => ('~', RGBA::named(ORANGE), RGBA::named(YELLOW)),
        "chasm" => (' ', RGBA::named(BLACK), RGBA::named(BLACK)),
        _ => ('?', RGBA::named(RED), RGBA::named(RED)),
    };

    ctx.set(x, y, fg, bg, to_cp437(glyph));
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Basic Map Editor (50x50)")
        .with_advanced_input(true)
        .build()?;

    let gs: State = State {
        map: Labyrinth2D::new_walled(50, 50),
        brush_state: TileType::Water,
        painting: false,
    };

    main_loop(context, gs)
}
