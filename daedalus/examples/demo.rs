use bracket_pathfinding::prelude::*;
use bracket_terminal::prelude::*;
use std::collections::HashSet;

use daedalus::prelude::*;
use labyrinth_map::prelude::*;

struct State {
    mapbuilder: MapGenerator2D,
    debug: bool,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // draw current map
        ctx.cls();

        draw_map(&self.mapbuilder.map(), ctx);
        draw_center(ctx);
        draw_doors(&self.mapbuilder, ctx);
        draw_panel(ctx);
        if self.debug {
            draw_debug(&mut self.mapbuilder, ctx);
        }

        // process user input
        let mut input = INPUT.lock();

        while let Some(ev) = input.pop() {
            match ev {
                BEvent::Character { c } => process_character(self, c),
                BEvent::CloseRequested => ctx.quit(),
                _ => (),
            }
        }

        self.mapbuilder.update_rooms();
    }
}

fn process_character(gs: &mut State, c: char) {
    // TODO: this should call different actions on the mapbuilder, allowing
    //       for a demo of each of the main methods in it
    // println!("{:?}", c);
    match c {
        'n' => gs.mapbuilder.flush_map(),
        'w' => {
            gs.mapbuilder.walled_map();
        }
        'r' => { // apply just one room here
        }
        'd' => {
            gs.debug = !gs.debug;
        }
        '1' => {
            gs.mapbuilder.generate(FloorGenAlg::Basic);
        }
        '0' => {
            generate_rooms_debug(gs);
        }
        _ => {}
    }
}

fn generate_rooms_debug(gs: &mut State) {
    // let map = gs.mapbuilder.map();
    gs.mapbuilder.flush_map();

    let mut rooms: Vec<Box<dyn Room>> = vec![];

    let mut room1 = RectRoom::new(3, 5);
    room1.shift(Point::new(10, 10));
    println!("Room1 entries: {:?}", room1.entries());
    println!("Room1 center: {:?}", room1.center());
    rooms.push(Box::new(room1));

    let mut room2 = RectRoom::new(3, 5);
    room2.rotate_right();
    room2.shift(Point::new(20, 10));
    rooms.push(Box::new(room2));
    let mut room3 = RectRoom::new(3, 5);
    room3.rotate_right();
    room3.rotate_right();
    room3.shift(Point::new(30, 10));
    rooms.push(Box::new(room3));
    let mut room4 = RectRoom::new(3, 5);
    room4.rotate_right();
    room4.rotate_right();
    room4.rotate_right();
    room4.shift(Point::new(40, 10));
    rooms.push(Box::new(room4));

    // for r in rooms {
    //     gs.mapbuilder.add_rooms(*r);
    // }
    gs.mapbuilder.extend_rooms(rooms);
}

fn draw_center(ctx: &mut BTerm) {
    ctx.set(25, 25, RGBA::named(WHITE), RGBA::named(BLUE), to_cp437(' '));
}

fn draw_panel(ctx: &mut BTerm) {
    ctx.draw_hollow_box_double(50, 1, 28, 48, RGBA::named(WHITE), RGBA::new());

    ctx.print(52, 4, "Controls");
    ctx.print(52, 5, "n: new (filled) map");
    ctx.print(52, 6, "w: new (walled) map");
    ctx.print(52, 7, "r: add room");
    ctx.print(52, 8, "d: toggle debug");

    ctx.print(52, 15, "1: generate basic map");
}

fn draw_debug(mapgen: &mut MapGenerator2D, ctx: &mut BTerm) {
    for pt in mapgen.rooms().borders() {
        ctx.set(
            pt.x,
            pt.y,
            RGBA::named(WHITE),
            RGBA::named(RED),
            to_cp437(' '),
        );
    }
    for pt in mapgen.rooms().walls() {
        ctx.set(
            pt.x,
            pt.y,
            RGBA::named(WHITE),
            RGBA::named(BLUE),
            to_cp437(' '),
        );
    }
    for pt in mapgen.rooms_mut().entries() {
        ctx.set(
            pt.x,
            pt.y,
            RGBA::named(WHITE),
            RGBA::named(GREEN),
            to_cp437(' '),
        );
    }

    // print each room number
    for (i, room) in mapgen.rooms().rooms().iter().enumerate() {
        // println!("Room number: {:?}", i);
        for pt in room.floor() {
            let val = f32::powf(0.9, i as f32);
            ctx.set(
                pt.x,
                pt.y,
                RGBA::from_f32(val, val, val, 1.0),
                RGBA::named(BLACK),
                to_cp437(
                    char::from_digit((i as u32) % 16, 16)
                        .expect(&format!("from_digit failed with i = {:?}", i)),
                ),
            );
        }
    }
}

fn draw_doors(mapgen: &MapGenerator2D, ctx: &mut BTerm) {
    for pt in mapgen.connections().iter() {
        ctx.set(
            pt.x,
            pt.y,
            RGBA::named(WHITE),
            RGBA::named(BURLYWOOD),
            to_cp437('+'),
        );
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
        "wall" => ('#', RGBA::named(GRAY), RGBA::named(BLACK)),
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

    let mut mapbuilder = MapGenerator2D::new(50, 50);

    let gs: State = State {
        mapbuilder,
        debug: false,
    };

    main_loop(context, gs)
}
