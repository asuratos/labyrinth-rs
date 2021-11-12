use super::{Labyrinth2D, Point, Tile};

use std::fmt;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use serde::de::{Deserializer, Error, MapAccess, Visitor};
use serde::ser::SerializeStruct;

impl Labyrinth2D {
    /// Constructs a mapstring and tiledict representation of the internal tiles
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

    /// Constructs a Labyrinth2D from a mapstring and tiledict representation
    fn unpack(
        mapstring: Vec<String>,
        tiledict: HashMap<char, Tile>,
    ) -> Result<Labyrinth2D, String> {
        // check if mapstring was valid
        // All rows must have same length
        if mapstring.iter().map(|str| str.len()).min()
            != mapstring.iter().map(|str| str.len()).max()
        {
            return Err(String::from("Row lengths do not match!"));
        }

        // then construct the Vec<Tiles> from the joined mapstr and the dict
        let width = mapstring[1].len() as i32;
        let height = mapstring.len() as i32;
        let dimensions = Point {
            x: width,
            y: height,
        };

        let joinedstr = mapstring.join("");

        let tiles = joinedstr
            .chars()
            .map(|c| tiledict.get(&c).cloned())
            .collect::<Option<Vec<Tile>>>();

        if tiles.is_none() {
            return Err(String::from("Tiledict incomplete, could not construct map"));
        }

        Ok(Labyrinth2D {
            tiles: tiles.unwrap(),
            dimensions,
            _filter: vec![],
        })
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

impl<'de> Deserialize<'de> for Labyrinth2D {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Mapstring,
            Tiledict,
        }

        struct Labyrinth2DVisitor;

        impl<'de> Visitor<'de> for Labyrinth2DVisitor {
            type Value = Labyrinth2D;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Labyrinth2D")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Labyrinth2D, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut mapstr: Option<Vec<String>> = None;
                let mut tiledict: Option<HashMap<char, Tile>> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Mapstring => {
                            if mapstr.is_some() {
                                return Err(Error::duplicate_field("mapstring"));
                            }
                            mapstr = Some(map.next_value()?);
                        }
                        Field::Tiledict => {
                            if tiledict.is_some() {
                                return Err(Error::duplicate_field("tiledict"));
                            }
                            tiledict = Some(map.next_value()?);
                        }
                    }
                }
                let mapstr = mapstr.ok_or_else(|| Error::missing_field("mapstring"))?;
                let tiledict = tiledict.ok_or_else(|| Error::missing_field("tiledict"))?;

                Ok(Labyrinth2D::unpack(mapstr, tiledict)
                    .map_err(|msg| Error::custom(format!("Unpack: {}", msg)))?)
            }
        }

        const FIELDS: &'static [&'static str] = &["mapstring", "tiledict"];
        deserializer.deserialize_struct("Labyrinth2D", FIELDS, Labyrinth2DVisitor)
    }
}
