use super::{Labyrinth2D, Tile};

use std::collections::HashMap;

use serde::Deserialize;

use serde::ser::{Serialize, SerializeStruct};

impl Labyrinth2D {
    /// constructs a mapstring representation of the internal tiles
    fn compress(&self) -> (Vec<String>, HashMap<char, Tile>) {
        let mut mapstr = vec![];
        let mut tiledict = HashMap::new();

        // add default values
        tiledict.insert(String::from("wall"), ('#', Tile::wall()));
        tiledict.insert(String::from("floor"), ('.', Tile::floor()));
        tiledict.insert(String::from("water"), ('~', Tile::water()));
        tiledict.insert(String::from("lava"), ('!', Tile::lava()));
        tiledict.insert(String::from("chasm"), (' ', Tile::chasm()));

        // These will be the possible keys/representations for the mapstring
        // goes from 0..9, a..z, A..Z. Hopefully that should be enough
        let mut key_iter = (b'0'..=b'9')
            .chain(b'a'..b'z')
            .chain(b'A'..=b'Z')
            .map(|c| c as char);

        // TODO: Make this better maybe?
        let mut tiles_copy: Vec<Tile> = self.tiles.clone();
        tiles_copy.dedup();

        for tile in tiles_copy.iter() {
            if !tiledict.values().any(|(_, val): &(char, Tile)| val == tile) {
                // find key to use
                let newkey = match key_iter.next() {
                    Some(key) => key,
                    None => {
                        // TODO: proper error handling here
                        panic!("too many custom tiles!")
                    }
                };

                // add tile to tiledict with the key
                tiledict.insert(tile.clone().kind, (newkey, tile.clone()));
            }
        }

        for row in self.iter_rows() {
            // TODO: check if row is complete here

            let mut mapstrrow = String::new();
            for tile in row {
                // get the representation of the tile
                let kind: &str = &tile.kind;

                let to_push = match tiledict.get(kind) {
                    Some((c, _)) => *c,
                    None => {
                        panic!("tile not found!")
                    }
                };

                mapstrrow.push(to_push);
            }
            mapstr.push(mapstrrow);
        }

        // generate the final char:tile dictionary
        let mut finaltiledict = HashMap::new();
        for (c, tile) in tiledict.into_values() {
            finaltiledict.insert(c, tile);
        }

        (mapstr, finaltiledict)
    }
}

impl Serialize for Labyrinth2D {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Labyrinth2D", 2)?;

        let (mapstring, dict) = self.compress();

        state.serialize_field("mapstring", &mapstring)?;
        state.serialize_field("tiledict", &dict)?;

        state.end()
    }
}
