use bracket_lib::prelude::*;
use labyrinth_map::prelude::*;

static MAP: &str = include_str!("pathfinding_map.ron");

enum TileType {
    Wall,
    Floor,
    Water,
    Lava,
    Chasm,
}

struct State {
    map: Labyrinth2D,
    movetypes: Vec<MoveType>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear
        ctx.cls();

        // draw panel
        // TODO: put this in a function
        ctx.draw_hollow_box_double(51, 1, 28, 48, RGBA::named(WHITE), RGBA::new());

        ctx.print(52, 4, "Move Types:");
        ctx.print(52, 5, "1: Walk");
        ctx.print(52, 6, "2: Fly");
        ctx.print(52, 7, "3: Swim");

        draw_map(&self.map, ctx);

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
    // TODO: import and export map files/strings
    match c {
        _ => {}
    }
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

    let map = Labyrinth2D::read_ron_from_str(MAP)
        .map_err(|msg| format!("Deserialize failed!: {}", msg))?;

    let gs: State = State {
        map,
        movetypes: vec![],
    };

    main_loop(context, gs)
}
