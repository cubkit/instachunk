use crate::HEIGHTMAX;
pub use crate::biomes::Biome::*;

pub const SHOREHEIGHT: u8 = 120; // was 120 (or 80, 60)
pub const BEACHHEIGHT: u8 = SHOREHEIGHT + 8;
pub const MINLAND:     u8 = BEACHHEIGHT + 8;

#[derive(Clone, Debug, PartialEq, Copy)]
#[repr(u8)]
pub enum Biome {
    Blank,
    Plains,
    Forest,
    Tundra,
    ColdForest,
    Desert,
    Jungle,
    Mountain,
    Ocean,
    Shore,
    Beach,
    Savanna,
    Swamp,
    CoralReef,
    FrozenOcean,
    FrozenShore,
    River,
    FrozenRiver,
    Glacier,
    RockyMountain,
    SparseJungle,
    CherryForest,
    Badlands
}

impl Biome {
    // pub fn to_u8(&self) -> u8 {
    //     return match self {
    //         &Self::Blank         =>  0,
    //         &Self::Plains        =>  1,
    //         &Self::Forest        =>  2,
    //         &Self::Tundra        =>  3,
    //         &Self::ColdForest    =>  4,
    //         &Self::Desert        =>  5,
    //         &Self::Jungle        =>  6,
    //         &Self::Mountain      =>  7,
    //         &Self::Ocean         =>  8,
    //         &Self::Shore         =>  9,
    //         &Self::Beach         => 10,
    //         &Self::Savanna       => 11,
    //         &Self::Swamp         => 12,
    //         &Self::CoralReef     => 13,
    //         &Self::FrozenOcean   => 14,
    //         &Self::FrozenShore   => 15,
    //         &Self::River         => 16,
    //         &Self::FrozenRiver   => 17,
    //         &Self::Glacier       => 18,
    //         &Self::RockyMountain => 19,
    //         &Self::SparseJungle  => 20,
    //         &Self::CherryForest  => 21
    //     };
    // }
    pub fn from_u8(n: u8) -> Option<Self> {
        return match n {
             0 => Some(Self::Blank),
             1 => Some(Self::Plains),
             2 => Some(Self::Forest),
             3 => Some(Self::Tundra),
             4 => Some(Self::ColdForest),
             5 => Some(Self::Desert),
             6 => Some(Self::Jungle),
             7 => Some(Self::Mountain),
             8 => Some(Self::Ocean),
             9 => Some(Self::Shore),
            10 => Some(Self::Beach),
            11 => Some(Self::Savanna),
            12 => Some(Self::Swamp),
            13 => Some(Self::CoralReef),
            14 => Some(Self::FrozenOcean),
            15 => Some(Self::FrozenShore),
            16 => Some(Self::River),
            17 => Some(Self::FrozenRiver),
            18 => Some(Self::Glacier),
            19 => Some(Self::RockyMountain),
            20 => Some(Self::SparseJungle),
            21 => Some(Self::CherryForest),
            22 => Some(Self::Badlands),
            _  => None
        };
    }

    /// Return the biome's colour
    pub fn colour(&self) -> [u8; 3] {
        return match self {
            &Self::Blank         => [0xff, 0x00, 0x00], // blank
            &Self::Plains        => [0x00, 0xaa, 0x00], // plains
            &Self::Forest        => [0x00, 0x88, 0x00], // forest
            &Self::Tundra        => [0xaa, 0xaa, 0xaa], // tundra
            &Self::ColdForest    => [0x88, 0x88, 0x88], // cold-forest
            &Self::Desert        => [0xaa, 0xaa, 0x00], // desert
            &Self::Jungle        => [0x00, 0x44, 0x00], // jungle
            &Self::Mountain      => [0xcc, 0xcc, 0xcc], // mountain
            &Self::Ocean         => [0x22, 0x22, 0xaa], // ocean
            &Self::Shore         => [0x22, 0x44, 0xaa], // shore
            &Self::Beach         => [0xaa, 0xaa, 0x44], // beach
            &Self::Savanna       => [0x88, 0xaa, 0x00], // savanna
            &Self::Swamp         => [0x44, 0xaa, 0x88], // swamp
            &Self::CoralReef     => [0x88, 0x66, 0xaa], // coral reef
            &Self::FrozenOcean   => [0x7b, 0xb2, 0xe9], // frozen-ocean
            &Self::FrozenShore   => [0x8e, 0xbb, 0xe7], // frozen-shore
            &Self::River         => [0x22, 0x66, 0xcc], // river
            &Self::FrozenRiver   => [0x62, 0x97, 0xcc], // frozen-river
            &Self::Glacier       => [0xaa, 0xcc, 0xee], // glacier
            &Self::RockyMountain => [0x66, 0x66, 0x66], // rocky-mountain
            &Self::SparseJungle  => [0x00, 0x66, 0x00], // sparse-jungle
            &Self::CherryForest  => [0xff, 0xaa, 0xff], // cherry-forest
            &Self::Badlands      => [0xcc, 0x88, 0x00], // Badlands
            //                   => [0x00, 0x00, 0x00], // error
        };
    }

    /// Is the biome a forest?
    pub fn is_forested(&self) -> bool {
        return self == &Self::Forest || self == &Self::ColdForest || self == &Self::Jungle || self == &Self::CherryForest;
    }

    /// Is the biome a mountain?
    pub fn is_mountain(&self) -> bool {
        return self == &Self::Mountain || self == &Self::Glacier;
    }

    /// Is the biome a ocean?
    pub fn is_ocean(&self) -> bool {
        return self == &Self::Ocean || self == &Self::Shore || self == &Self::CoralReef || self == &Self::FrozenOcean || self == &Self::FrozenShore;
    }

    /// Is the biome treeless?
    pub fn is_treeless(&self) -> bool {
        return self.is_mountain() || self.is_ocean() || self.is_river() || self == &Self::Beach || self == &Self::Desert || self == &Self::Badlands;
    }

    /// Is the biome a semi-forest?
    pub fn is_semi_forested(&self) -> bool {
        return self == &Self::Savanna || self == &Self::SparseJungle;
    }

    /// Is the biome a river?
    pub fn is_river(&self) -> bool {
        return self == &Self::River || self == &Self::FrozenRiver;
    }

    pub fn edit_height(&self, height: u8) -> u8 {
        match self {
            &Self::Badlands => (height / 10) * 10,
            _               => height
        }
    }

}

/// Get the biome from the height, heat, follage, moisture, riverness, & oddness.
pub fn biome(height: u8, heat: u8, follage: u8, moisture: u8, riverness: u8, oddness: u8) -> Biome {
    let max = HEIGHTMAX;
    let height_max = HEIGHTMAX;
    // Temperatue
    let cold = (max / 2) - (max / 10);
    let warm = max - cold;
    // Moisture
    let arid  = max / 3;
    let moist = max - arid;
    // Follage
    let treeless = max / 2;
    let forested = max - treeless;
    // Oddness
    let odd = max / 2;
    if height > height_max-(height_max/3) {
        if moisture > arid {
            if riverness > (max/2)-3 && riverness < (max/2)+3 {
                return Glacier;
            } else {
                return Mountain;
            }
        } else {
            return RockyMountain;
        }
    } else if height < MINLAND {
        if height > SHOREHEIGHT {
            if height > BEACHHEIGHT {
                if moisture > moist {
                    return Swamp;
                } else {
                    if riverness > (max/2)-5 && riverness < (max/2)+5 {
                        return River;
                    } else {
                        return Beach;
                    }
                }
            } else {
                if heat < cold {
                    return FrozenShore;
                } else {
                    return Shore;
                }
            }
        } else {
            if height > MINLAND-(height_max/12) && heat > warm {
                return CoralReef;
            } else if heat < cold {
                return FrozenOcean;
            } else {
                return Ocean;
            }
        }
    } else {
        if riverness > (max/2)-3 && riverness < (max/2)+3 {
            if heat < cold {
                return FrozenRiver;
            } else {
                return River;
            }
        } else {
            if heat < cold {
                if follage < treeless {
                    return Tundra;
                } else {
                    return ColdForest;
                }
            } else if heat > warm {
                if moisture > moist {
                    if follage > forested {
                        return Jungle;
                    } else {
                        return SparseJungle;
                    }
                } else {
                    if follage < treeless {
                        if oddness > odd {
                            return Badlands;
                        } else {
                            return Desert;
                        }
                    } else {
                        return Savanna;
                    }
                }
            } else {
                if follage < treeless {
                    return Plains;
                } else {
                    if oddness > odd && height < BEACHHEIGHT+10 {
                        return CherryForest;
                    } else {
                        return Forest;
                    }
                }
            }
        }
    }
}

