/// Convenient definition for every level in the game.
///
/// # Scene IDs
/// Battle for Bikini Bottom uses bytestrings encoded into a u32 to identify scenes.
/// Internally this was implemented with C/C++ multi-character constants (`u32 scene_id = 'HB01'`).
/// `Room` implements conversion to and from this format using a byte array (`[u8; 4]`).
///
/// # Examples
/// Using scene ids:
/// ```
/// use bfbb::Room;
///
/// // Attempting to get a Room from a scene_id
///
/// let scene_id = b"HB01";
/// let room = Room::try_from(*scene_id).expect("'HB01' should be the main hub level");
/// assert_eq!(room, Room::BikiniBottom);
///
/// // Converting a Room to it's scene_id
///
/// let hub = Room::BikiniBottom;
/// let scene_id: [u8; 4] = hub.into();
/// assert_eq!(b"HB01", &scene_id)
/// ```
///
/// Converting a Room to it's in-game name:
/// ```
/// use bfbb::Room;
///
/// let room = Room::SpongebobHouse;
/// println!("{room}"); // Prints "Spongebob's House"
/// assert_eq!(room.to_string().as_str(), "Spongebob's House");
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Room {
    MainMenu,
    IntroCutscene,

    BikiniBottom,
    SpongebobHouse,
    SquidwardHouse,
    PatrickHouse,
    ShadyShoals,
    PoliceStation,
    Treedome,
    KrustyKrab,
    ChumBucket,
    Theater,

    Poseidome,
    IndustrialPark,

    JellyfishRock,
    JellyfishCaves,
    JellyfishLake,
    JellyfishMountain,

    DowntownStreets,
    DowntownRooftops,
    DowntownLighthouse,
    DowntownSeaNeedle,

    GooLagoonBeach,
    GooLagoonCaves,
    GooLagoonPier,

    MermalairEntranceArea,
    MermalairMainChamber,
    MermalairSecurityTunnel,
    MermalairBallroom,
    MermalairVillianContainment,

    RockBottomDowntown,
    RockBottomMuseum,
    RockBottomTrench,

    SandMountainHub,
    SandMountainSlide1,
    SandMountainSlide2,
    SandMountainSlide3,

    KelpForest,
    KelpSwamps,
    KelpCaves,
    KelpVines,

    GraveyardLake,
    GraveyardShipwreck,
    GraveyardShip,
    GraveyardBoss,

    SpongebobsDream,
    SandysDream,
    SquidwardsDream,
    KrabsDream,
    PatricksDream,

    ChumBucketLab,
    ChumBucketBrain,

    SpongeballArena,
}

impl std::fmt::Display for Room {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Room::*;
        match *self {
            MainMenu => write!(fmt, "Main Menu"),
            IntroCutscene => write!(fmt, "Intro Cutscene"),
            BikiniBottom => write!(fmt, "Bikini Bottom"),
            SpongebobHouse => write!(fmt, "Spongebob's House"),
            SquidwardHouse => write!(fmt, "Squidward's House"),
            PatrickHouse => write!(fmt, "Patrick's House"),
            ShadyShoals => write!(fmt, "Shady Shoals"),
            PoliceStation => write!(fmt, "Police Station"),
            Treedome => write!(fmt, "Treedome"),
            KrustyKrab => write!(fmt, "Krusty Krab"),
            ChumBucket => write!(fmt, "Chum Bucket"),
            Theater => write!(fmt, "Theater"),
            Poseidome => write!(fmt, "Poseidome"),
            IndustrialPark => write!(fmt, "Industrial Park"),
            JellyfishRock => write!(fmt, "Jellyfish Rock"),
            JellyfishCaves => write!(fmt, "Jellyfish Caves"),
            JellyfishLake => write!(fmt, "Jellyfish Lake"),
            JellyfishMountain => write!(fmt, "Jellyfish Mountain"),
            DowntownStreets => write!(fmt, "Downtown Streets"),
            DowntownRooftops => write!(fmt, "Downtown Rooftops"),
            DowntownLighthouse => write!(fmt, "Downtown Lighthouse"),
            DowntownSeaNeedle => write!(fmt, "Downtown Sea Needle"),
            GooLagoonBeach => write!(fmt, "Goo Lagoon Beach"),
            GooLagoonCaves => write!(fmt, "Goo Lagoon Caves"),
            GooLagoonPier => write!(fmt, "Goo Lagoon Pier"),
            MermalairEntranceArea => write!(fmt, "Mermalair Entrance Area"),
            MermalairMainChamber => write!(fmt, "Mermalair Main Chamber"),
            MermalairSecurityTunnel => write!(fmt, "Mermalair Security Tunnel"),
            MermalairBallroom => write!(fmt, "Mermalair Ballroom"),
            MermalairVillianContainment => write!(fmt, "Mermalair Villian Containment"),
            RockBottomDowntown => write!(fmt, "Rock Bottom Downtown"),
            RockBottomMuseum => write!(fmt, "Rock Bottom Museum"),
            RockBottomTrench => write!(fmt, "Rock Bottom Trench"),
            SandMountainHub => write!(fmt, "Ski Lodge"),
            SandMountainSlide1 => write!(fmt, "Guppy Mound"),
            SandMountainSlide2 => write!(fmt, "Flounder Hill"),
            SandMountainSlide3 => write!(fmt, "Sand Mountain"),
            KelpForest => write!(fmt, "Kelp Forest"),
            KelpSwamps => write!(fmt, "Kelp Swamps"),
            KelpCaves => write!(fmt, "Kelp Caves"),
            KelpVines => write!(fmt, "Kelp Vines"),
            GraveyardLake => write!(fmt, "Graveyard Lake"),
            GraveyardShipwreck => write!(fmt, "Graveyard of Ships"),
            GraveyardShip => write!(fmt, "Dutchman's Ship"),
            GraveyardBoss => write!(fmt, "Flying Dutchman Battle"),
            SpongebobsDream => write!(fmt, "Spongebob's Dream"),
            SandysDream => write!(fmt, "Sandy's Dream"),
            SquidwardsDream => write!(fmt, "Squidward's Dream"),
            KrabsDream => write!(fmt, "Krab's Dream"),
            PatricksDream => write!(fmt, "Patrick's Dream"),
            ChumBucketLab => write!(fmt, "Chum Bucket Lab"),
            ChumBucketBrain => write!(fmt, "Chum Bucket Brain"),
            SpongeballArena => write!(fmt, "Spongeball Arena"),
        }
    }
}

impl TryFrom<[u8; 4]> for Room {
    type Error = &'static str;

    fn try_from(scene_id: [u8; 4]) -> Result<Self, Self::Error> {
        use Room::*;
        match &scene_id {
            b"MNU3" => Ok(MainMenu),
            b"HB00" => Ok(IntroCutscene),
            b"HB01" => Ok(BikiniBottom),
            b"HB02" => Ok(SpongebobHouse),
            b"HB03" => Ok(SquidwardHouse),
            b"HB04" => Ok(PatrickHouse),
            b"HB06" => Ok(ShadyShoals),
            b"HB09" => Ok(PoliceStation),
            b"HB05" => Ok(Treedome),
            b"HB07" => Ok(KrustyKrab),
            b"HB08" => Ok(ChumBucket),
            b"HB10" => Ok(Theater),
            b"B101" => Ok(Poseidome),
            b"B201" => Ok(IndustrialPark),
            b"JF01" => Ok(JellyfishRock),
            b"JF02" => Ok(JellyfishCaves),
            b"JF03" => Ok(JellyfishLake),
            b"JF04" => Ok(JellyfishMountain),
            b"BB01" => Ok(DowntownStreets),
            b"BB02" => Ok(DowntownRooftops),
            b"BB03" => Ok(DowntownLighthouse),
            b"BB04" => Ok(DowntownSeaNeedle),
            b"GL01" => Ok(GooLagoonBeach),
            b"GL02" => Ok(GooLagoonCaves),
            b"GL03" => Ok(GooLagoonPier),
            b"BC01" => Ok(MermalairEntranceArea),
            b"BC02" => Ok(MermalairMainChamber),
            b"BC03" => Ok(MermalairSecurityTunnel),
            b"BC04" => Ok(MermalairBallroom),
            b"BC05" => Ok(MermalairVillianContainment),
            b"RB01" => Ok(RockBottomDowntown),
            b"RB02" => Ok(RockBottomMuseum),
            b"RB03" => Ok(RockBottomTrench),
            b"SM01" => Ok(SandMountainHub),
            b"SM02" => Ok(SandMountainSlide1),
            b"SM03" => Ok(SandMountainSlide2),
            b"SM04" => Ok(SandMountainSlide3),
            b"KF01" => Ok(KelpForest),
            b"KF02" => Ok(KelpSwamps),
            b"KF04" => Ok(KelpCaves),
            b"KF05" => Ok(KelpVines),
            b"GY01" => Ok(GraveyardLake),
            b"GY02" => Ok(GraveyardShipwreck),
            b"GY03" => Ok(GraveyardShip),
            b"GY04" => Ok(GraveyardBoss),
            b"DB01" => Ok(SpongebobsDream),
            b"DB02" => Ok(SandysDream),
            b"DB03" => Ok(SquidwardsDream),
            b"DB04" => Ok(KrabsDream),
            b"DB06" => Ok(PatricksDream),
            b"B302" => Ok(ChumBucketLab),
            b"B303" => Ok(ChumBucketBrain),
            b"PG12" => Ok(SpongeballArena),
            _ => Err("Byte array did not correspond to a level."),
        }
    }
}

impl From<Room> for [u8; 4] {
    fn from(room: Room) -> [u8; 4] {
        use Room::*;
        *match room {
            Room::MainMenu => b"MNU3",
            IntroCutscene => b"HB00",
            BikiniBottom => b"HB01",
            SpongebobHouse => b"HB02",
            SquidwardHouse => b"HB03",
            PatrickHouse => b"HB04",
            ShadyShoals => b"HB06",
            PoliceStation => b"HB09",
            Treedome => b"HB05",
            KrustyKrab => b"HB07",
            ChumBucket => b"HB08",
            Theater => b"HB10",
            Poseidome => b"B101",
            IndustrialPark => b"B201",
            JellyfishRock => b"JF01",
            JellyfishCaves => b"JF02",
            JellyfishLake => b"JF03",
            JellyfishMountain => b"JF04",
            DowntownStreets => b"BB01",
            DowntownRooftops => b"BB02",
            DowntownLighthouse => b"BB03",
            DowntownSeaNeedle => b"BB04",
            GooLagoonBeach => b"GL01",
            GooLagoonCaves => b"GL02",
            GooLagoonPier => b"GL03",
            MermalairEntranceArea => b"BC01",
            MermalairMainChamber => b"BC02",
            MermalairSecurityTunnel => b"BC03",
            MermalairBallroom => b"BC04",
            MermalairVillianContainment => b"BC05",
            RockBottomDowntown => b"RB01",
            RockBottomMuseum => b"RB02",
            RockBottomTrench => b"RB03",
            SandMountainHub => b"SM01",
            SandMountainSlide1 => b"SM02",
            SandMountainSlide2 => b"SM03",
            SandMountainSlide3 => b"SM04",
            KelpForest => b"KF01",
            KelpSwamps => b"KF02",
            KelpCaves => b"KF04",
            KelpVines => b"KF05",
            GraveyardLake => b"GY01",
            GraveyardShipwreck => b"GY02",
            GraveyardShip => b"GY03",
            GraveyardBoss => b"GY04",
            SpongebobsDream => b"DB01",
            SandysDream => b"DB02",
            SquidwardsDream => b"DB03",
            KrabsDream => b"DB04",
            PatricksDream => b"DB06",
            ChumBucketLab => b"B302",
            ChumBucketBrain => b"B303",
            SpongeballArena => b"PG12",
        }
    }
}
