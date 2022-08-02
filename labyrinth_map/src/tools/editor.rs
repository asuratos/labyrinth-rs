use bracket_lib::prelude::*;
use labyrinth_map::prelude::*;

type LTile = labyrinth_map::prelude::Tile;

enum TileType {
    Wall,
    Floor,
    Water,
    Lava,
    Chasm,
    Custom(String),
}

struct State {
    map: Labyrinth2D,
    brush_state: TileType,
    painting: bool,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear
        ctx.cls();

        // draw panel
        // TODO: put this in a function
        ctx.draw_hollow_box_double(51, 1, 28, 48, RGBA::named(WHITE), RGBA::new());

        let tilename = match &self.brush_state {
            TileType::Wall => "wall",
            TileType::Floor => "floor",
            TileType::Water => "water",
            TileType::Lava => "lava",
            TileType::Chasm => "chasm",
            TileType::Custom(str) => &str,
        };

        ctx.print(52, 2, format!("Click to set tile to {}", tilename));
        ctx.print(52, 4, "Controls:");
        ctx.print(52, 5, "1: Wall");
        ctx.print(52, 6, "2: Floor");
        ctx.print(52, 7, "3: Water");
        ctx.print(52, 8, "4: Lava");
        ctx.print(52, 9, "5: Chasm");
        ctx.print(52, 20, "e: export map");
        ctx.print(52, 21, "i: import map");

        draw_map(&self.map, ctx);

        // process user input
        let mut input = INPUT.lock();

        while let Some(ev) = input.pop() {
            match ev {
                BEvent::MouseButtonDown { button: 0 } => self.painting = true,
                BEvent::MouseButtonUp { button: 0 } => self.painting = false,
                BEvent::CursorMoved { .. } => try_paint_tile(self, ctx),
                BEvent::Character { c } => process_character(self, c),
                BEvent::CloseRequested => ctx.quit(),
                _ => (),
            }
        }
    }
}

fn process_character(gs: &mut State, c: char) {
    // TODO: import and export map files/strings
    if let Some(newtile) = match c {
        '1' => Some(TileType::Wall),
        '2' => Some(TileType::Floor),
        '3' => Some(TileType::Water),
        '4' => Some(TileType::Lava),
        '5' => Some(TileType::Chasm),
        '6' => Some(TileType::Custom(String::from("phasing"))),
        'e' => {
            export(gs);
            None
        }
        'i' => {
            import(gs);
            None
        }
        _ => None,
    } {
        gs.brush_state = newtile;
    }
}

fn export(gs: &State) {
    match gs.map.dump_ron("map.ron") {
        Err(e) => {
            println!("{}", e)
        }
        _ => {}
    };
}

fn import(gs: &mut State) {
    match Labyrinth2D::read_ron("map.ron") {
        Ok(map) => gs.map = map,
        Err(e) => {
            println!("{}", e)
        }
    };
}

fn try_paint_tile(gs: &mut State, ctx: &mut BTerm) {
    if !gs.painting {
        return;
    }

    let loc = ctx.mouse_point();

    let newtile = match &gs.brush_state {
        TileType::Floor => LTile::floor(),
        TileType::Wall => LTile::wall(),
        TileType::Water => LTile::water(),
        TileType::Lava => LTile::lava(),
        TileType::Chasm => LTile::chasm(),
        TileType::Custom(tilekind) => LTile::new(tilekind, true, []),
    };

    gs.map.set_tile_at(loc, newtile)
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
        "floor" => ('.', RGBA::named(GRAY), RGBA::named(BLACK)),
        "water" => ('~', RGBA::named(LIGHT_BLUE), RGBA::named(BLUE)),
        "lava" => ('~', RGBA::named(ORANGE), RGBA::named(YELLOW)),
        "chasm" => (' ', RGBA::named(BLACK), RGBA::named(DARK_BLUE)),
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
        brush_state: TileType::Wall,
        painting: false,
    };

    main_loop(context, gs)
}
