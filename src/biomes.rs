pub const MINLAND: u8 = 136;
pub const SHOREHEIGHT: u8 = 120;
pub const BEACHHEIGHT: u8 = 128;


pub const BLANK: u8 = 0;
pub const PLAINS: u8 = 1;
pub const FOREST: u8 = 2;
pub const TUNDRA: u8 = 3;
pub const COLDFOREST: u8 = 4;
pub const DESERT: u8 = 5;
pub const JUNGLE: u8 = 6;
pub const MOUNTAIN: u8 = 7;
pub const OCEAN: u8 = 8;
pub const SHORE: u8 = 9;
pub const BEACH: u8 = 10;
pub const SAVANNA: u8 = 11;
pub const SWAMP: u8 = 12;
pub const CORALREEF: u8 = 13;
pub const FROZENOCEAN: u8 = 14;
pub const FROZENSHORE: u8 = 15;
pub const RIVER: u8 = 16;
pub const FROZENRIVER: u8 = 17;
pub const GLACIER: u8 = 18;
pub const ROCKYMOUNTAIN: u8 = 19;
pub const SPARSEJUNGLE: u8 = 20;
pub const CHERRYFOREST: u8 = 21;

pub const MAXBIOMES: u8 = 20;


pub fn biome(max:u8, height:u8, heat:u8, follage:u8, moisture:u8, riverness:u8, oddness:u8) -> u8 {
    if height > max-(max/3) {
        if moisture > max/4 {
            if riverness > (max/2)-3 && riverness < (max/2)+3 {
                return GLACIER;
            } else {
                return MOUNTAIN;
            }
        } else {
            return ROCKYMOUNTAIN;
        }
    } else if height < MINLAND {
        if height > SHOREHEIGHT {
            if height > BEACHHEIGHT {
                if moisture > max/2 {
                    return SWAMP;
                } else {
                    if riverness > (max/2)-5 && riverness < (max/2)+5 {
                        return RIVER;
                    } else {
                        return BEACH;
                    }
                }
            } else {
                if heat < max/3 {
                    return FROZENSHORE;
                } else {
                    return SHORE;
                }
            }
        } else {
            if height > MINLAND-(max/12) && heat > max-(max/3) {
                return CORALREEF;
            } else if heat < max/3 {
                return FROZENOCEAN;
            } else {
                return OCEAN;
            }
        }
    } else {
        if riverness > (max/2)-3 && riverness < (max/2)+3 {
            if heat < max/3 {
                return FROZENRIVER;
            } else {
                return RIVER;
            }
        } else {
            if heat < max/3 {
                if follage < max/2 {
                    return TUNDRA;
                } else {
                    return COLDFOREST;
                }
            } else if heat > max-(max/3) {
                if moisture < max/3 {
                    if follage < max/2 {
                        return DESERT;
                    } else {
                        return SAVANNA;
                    }
                } else if moisture > max/2 {
                    if follage > max-(max/3) {
                        return JUNGLE;
                    } else {
                        return SPARSEJUNGLE;
                    }
                } else {
                    return SAVANNA;
                }
            } else {
                if follage < max/2 {
                    return PLAINS;
                } else {
                    if oddness > max/2 && height < BEACHHEIGHT+10 {
                        return CHERRYFOREST;
                    } else {
                        return FOREST;
                    }
                }
            }
        }
    }
}


pub fn isforest(biome: u8) -> bool {
    return biome == FOREST || biome == COLDFOREST || biome == JUNGLE || biome == CHERRYFOREST;
}

pub fn ismountain(biome: u8) -> bool {
    return biome == MOUNTAIN || biome == GLACIER;
}

pub fn isocean(biome: u8) -> bool {
    return biome == OCEAN || biome == SHORE || biome == CORALREEF || biome == FROZENOCEAN || biome == FROZENSHORE;
}

pub fn istreeless(biome: u8) -> bool {
    return ismountain(biome) || isocean(biome) || isriver(biome) || biome == BEACH || biome == DESERT;
}
pub fn issemiforest(biome: u8) -> bool {
    return biome == SAVANNA || biome == SPARSEJUNGLE;
}

pub fn isriver(biome: u8) -> bool {
    return biome == RIVER || biome == FROZENRIVER;
}

#[allow(dead_code)]
pub fn biomecolour(biome: u8) -> [u8; 3] {
    return match biome {
        BLANK         => [0xff, 0x00, 0x00], // blank
        PLAINS        => [0x00, 0xaa, 0x00], // plains
        FOREST        => [0x00, 0x88, 0x00], // forest
        TUNDRA        => [0xaa, 0xaa, 0xaa], // tundra
        COLDFOREST    => [0x88, 0x88, 0x88], // cold-forest
        DESERT        => [0xaa, 0xaa, 0x00], // desert
        JUNGLE        => [0x00, 0x44, 0x00], // jungle
        MOUNTAIN      => [0xcc, 0xcc, 0xcc], // mountain
        OCEAN         => [0x22, 0x22, 0xaa], // ocean
        SHORE         => [0x22, 0x44, 0xaa], // shore
        BEACH         => [0xaa, 0xaa, 0x44], // beach
        SAVANNA       => [0x88, 0xaa, 0x00], // savanna
        SWAMP         => [0x44, 0xaa, 0x88], // swamp
        CORALREEF     => [0x88, 0x66, 0xaa], // coral reef
        FROZENOCEAN   => [0x7b, 0xb2, 0xe9], // frozen-ocean
        FROZENSHORE   => [0x8e, 0xbb, 0xe7], // frozen-shore
        RIVER         => [0x22, 0x66, 0xcc], // river
        FROZENRIVER   => [0x62, 0x97, 0xcc], // frozen-river
        GLACIER       => [0xaa, 0xcc, 0xee], // glacier
        ROCKYMOUNTAIN => [0x66, 0x66, 0x66], // rocky-mountain
        SPARSEJUNGLE  => [0x00, 0x66, 0x00], // sparse-jungle
        CHERRYFOREST  => [0xff, 0xaa, 0xff], // cherry-forest
        _             => [0x00, 0x00, 0x00], // error
    };
}