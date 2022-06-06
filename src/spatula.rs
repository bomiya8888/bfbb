use strum_macros::{EnumCount, EnumIter};

/// Convenient definition for every spatula in the game
///
/// # Menu coordinates:
/// `Spatula` has support for interpeting a `(usize, usize)` value as a coordinate from the in-game menu,
/// where the first coordinate is the level-index and the second coordinate is the spatula-index,
/// and convert it to a `Spatula` value (or vice-versa).
///
/// ```
/// use bfbb::Spatula;
///
/// let coord = (0,3);
/// let spatula = Spatula::try_from(coord).expect("'(0,3)' should be Spongebob's Closet");
/// assert_eq!(spatula, Spatula::SpongebobsCloset);
///
/// let closet = Spatula::SpongebobsCloset;
/// let coord: (usize, usize) = closet.into();
/// assert_eq!(coord, (0,3))
/// ```

#[derive(EnumIter, EnumCount, Hash, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Spatula {
    // Bikini Bottom
    #[doc(hidden)]
    OnTopOfThePineapple,
    #[doc(hidden)]
    OnTopOfShadyShoals,
    #[doc(hidden)]
    OnTopOfTheChumBucket,
    #[doc(hidden)]
    SpongebobsCloset,
    #[doc(hidden)]
    AnnoySquidward,
    #[doc(hidden)]
    AmbushAtTheTreeDome,
    #[doc(hidden)]
    InfestationAtTheKrustyKrab,
    #[doc(hidden)]
    AWallJumpInTheBucket,

    // Jellyfish Fields
    #[doc(hidden)]
    TopOfTheHill,
    #[doc(hidden)]
    CowaBungee,
    #[doc(hidden)]
    Spelunking,
    #[doc(hidden)]
    PatricksDilemma,
    #[doc(hidden)]
    NavigateTheCanyonsAndMesas,
    #[doc(hidden)]
    DrainTheLake,
    #[doc(hidden)]
    SlideLeap,
    #[doc(hidden)]
    DefeatKingJellyfish,

    // Downtown Bikini Bottom
    #[doc(hidden)]
    EndOfTheRoad,
    #[doc(hidden)]
    LearnSandysMoves,
    #[doc(hidden)]
    TikisGoBoom,
    #[doc(hidden)]
    AcrossTheRooftops,
    #[doc(hidden)]
    SwinginSandy,
    #[doc(hidden)]
    AmbushInTheLighthouse,
    #[doc(hidden)]
    ExtremeBungee,
    #[doc(hidden)]
    ComeBackWithTheCruiseBubble,

    // Goo Lagoon
    #[doc(hidden)]
    KingOfTheCastle,
    #[doc(hidden)]
    ConnectTheTowers,
    #[doc(hidden)]
    SaveTheChildren,
    #[doc(hidden)]
    OverTheMoat,
    #[doc(hidden)]
    ThroughTheSeaCaves,
    #[doc(hidden)]
    CleanOutTheBumperBoats,
    #[doc(hidden)]
    SlipAndSlideUnderThePier,
    #[doc(hidden)]
    TowerBungee,

    // Poseidome
    #[doc(hidden)]
    RumbleAtThePoseidome,

    // Rock Bottom
    #[doc(hidden)]
    GetToTheMuseum,
    #[doc(hidden)]
    SlipSlidingAway,
    #[doc(hidden)]
    ReturnTheMuseumsArt,
    #[doc(hidden)]
    SwingalongSpatula,
    #[doc(hidden)]
    PlunderingRobotsInTheMuseum,
    #[doc(hidden)]
    AcrossTheTrenchOfDarkness,
    #[doc(hidden)]
    LasersAreFunAndGoodForYou,
    #[doc(hidden)]
    HowInTarnationDoYouGetThere,

    // Mermalair
    #[doc(hidden)]
    TopOfTheEntranceAreaML,
    #[doc(hidden)]
    TopOfTheComputerArea,
    #[doc(hidden)]
    ShutDownTheSecuritySystem,
    #[doc(hidden)]
    TheFunnelMachines,
    #[doc(hidden)]
    TheSpinningTowersOfPower,
    #[doc(hidden)]
    TopOfTheSecurityTunnel,
    #[doc(hidden)]
    CompleteTheRollingBallRoom,
    #[doc(hidden)]
    DefeatPrawn,

    // Sand Mountain
    #[doc(hidden)]
    FrostyBungee,
    #[doc(hidden)]
    TopOfTheLodge,
    #[doc(hidden)]
    DefeatRobotsOnGuppyMound,
    #[doc(hidden)]
    BeatMrsPuffsTime,
    #[doc(hidden)]
    DefeatRobotsOnFlounderHill,
    #[doc(hidden)]
    BeatBubbleBuddysTime,
    #[doc(hidden)]
    DefeatRobotsOnSandMountain,
    #[doc(hidden)]
    BeatLarrysTime,

    // Industrial Park
    #[doc(hidden)]
    RoboPatrickAhoy,

    // Kelp Forest
    #[doc(hidden)]
    ThroughTheWoods,
    #[doc(hidden)]
    FindAllTheLostCampers,
    #[doc(hidden)]
    TikiRoundup,
    #[doc(hidden)]
    DownInTheSwamp,
    #[doc(hidden)]
    ThroughTheKelpCaves,
    #[doc(hidden)]
    PowerCrystalCrisis,
    #[doc(hidden)]
    KelpVineSlide,
    #[doc(hidden)]
    BeatMermaidMansTime,

    // Flying Dutchman's Graveyard
    #[doc(hidden)]
    TopOfTheEntranceAreaFDG,
    #[doc(hidden)]
    APathThroughTheGoo,
    #[doc(hidden)]
    GooTankerAhoy,
    #[doc(hidden)]
    TopOfTheStackOfShips,
    #[doc(hidden)]
    ShipwreckBungee,
    #[doc(hidden)]
    DestroyTheRobotShip,
    #[doc(hidden)]
    GetAloftThereMatey,
    #[doc(hidden)]
    DefeatTheFlyingDutchman,

    // SpongeBob's Dream
    #[doc(hidden)]
    AcrossTheDreamscape,
    #[doc(hidden)]
    FollowTheBouncingBall,
    #[doc(hidden)]
    SlidingTexasStyle,
    #[doc(hidden)]
    SwingersAhoy,
    #[doc(hidden)]
    MusicIsInTheEarOfTheBeholder,
    #[doc(hidden)]
    KrabbyPattyPlatforms,
    #[doc(hidden)]
    SuperBounce,
    #[doc(hidden)]
    HereYouGo,

    // Chum Bucket Lab
    #[doc(hidden)]
    KahRahTae,
    #[doc(hidden)]
    TheSmallShallRuleOrNot,
}

impl Spatula {
    /// In-game, spatula objects are stored within an array of entities.
    /// `get_offset` gets the index of a spatula within that entity array.
    ///
    /// **NOTE:** These indexes are only valid while the spatula's level is loaded.
    ///
    /// **NOTE:** This list is currently only validated for the Gamecube version of the game.
    pub fn get_offset(&self) -> Option<usize> {
        use Spatula::*;
        match *self {
            OnTopOfThePineapple => Some(0xa8),
            OnTopOfShadyShoals => Some(0xcf),
            OnTopOfTheChumBucket => Some(0xd0),
            SpongebobsCloset => Some(0x5d),
            AnnoySquidward => Some(0x26),
            AmbushAtTheTreeDome => Some(0x3a),
            InfestationAtTheKrustyKrab => Some(0xce),
            AWallJumpInTheBucket => Some(0x2a),
            TopOfTheHill => Some(0xc8),
            CowaBungee => Some(0xc9),
            Spelunking => Some(0xd8),
            PatricksDilemma => Some(0xd7),
            NavigateTheCanyonsAndMesas => Some(0xfa),
            DrainTheLake => Some(0xea),
            SlideLeap => Some(0x58),
            DefeatKingJellyfish => Some(0x128),
            EndOfTheRoad => Some(0xba),
            LearnSandysMoves => Some(0xb9),
            TikisGoBoom => Some(0x111),
            AcrossTheRooftops => Some(0xab),
            SwinginSandy => Some(0xac),
            AmbushInTheLighthouse => Some(0x53),
            ExtremeBungee => Some(0x99),
            ComeBackWithTheCruiseBubble => Some(0x9a),
            KingOfTheCastle => Some(0x12a),
            ConnectTheTowers => Some(0x154),
            SaveTheChildren => Some(0x153),
            OverTheMoat => Some(0x12b),
            ThroughTheSeaCaves => Some(0x5c),
            CleanOutTheBumperBoats => Some(0xff),
            SlipAndSlideUnderThePier => Some(0xfd),
            TowerBungee => Some(0xfe),
            RumbleAtThePoseidome => Some(0x28),
            GetToTheMuseum => Some(0xff),
            SlipSlidingAway => Some(0xfe),
            ReturnTheMuseumsArt => Some(0x105),
            SwingalongSpatula => Some(0x107),
            PlunderingRobotsInTheMuseum => Some(0x76),
            AcrossTheTrenchOfDarkness => Some(0xa5),
            LasersAreFunAndGoodForYou => Some(0xa4),
            HowInTarnationDoYouGetThere => Some(0xa3),
            TopOfTheEntranceAreaML => Some(0x72),
            TopOfTheComputerArea => Some(0x6a),
            ShutDownTheSecuritySystem => Some(0x6b),
            TheFunnelMachines => Some(0x68),
            TheSpinningTowersOfPower => Some(0x69),
            TopOfTheSecurityTunnel => Some(0x9a),
            CompleteTheRollingBallRoom => Some(0x45),
            DefeatPrawn => Some(0x39),
            FrostyBungee => Some(0x5d),
            TopOfTheLodge => Some(0x5e),
            DefeatRobotsOnGuppyMound => Some(0x91),
            BeatMrsPuffsTime => Some(0x92),
            DefeatRobotsOnFlounderHill => Some(0xa8),
            BeatBubbleBuddysTime => Some(0xa9),
            DefeatRobotsOnSandMountain => Some(0xcd),
            BeatLarrysTime => Some(0xcc),
            RoboPatrickAhoy => Some(0x28),
            ThroughTheWoods => Some(0x94),
            FindAllTheLostCampers => Some(0x8d),
            TikiRoundup => Some(0x83),
            DownInTheSwamp => Some(0x84),
            ThroughTheKelpCaves => Some(0x5a),
            PowerCrystalCrisis => Some(0x53),
            KelpVineSlide => Some(0x53),
            BeatMermaidMansTime => Some(0x54),
            TopOfTheEntranceAreaFDG => Some(0x70),
            APathThroughTheGoo => Some(0x71),
            GooTankerAhoy => Some(0x6f),
            TopOfTheStackOfShips => Some(0x86),
            ShipwreckBungee => Some(0x87),
            DestroyTheRobotShip => Some(0x5f),
            GetAloftThereMatey => Some(0x60),
            DefeatTheFlyingDutchman => Some(0x35),
            AcrossTheDreamscape => Some(0x5e),
            FollowTheBouncingBall => Some(0x5f),
            SlidingTexasStyle => Some(0xa1),
            SwingersAhoy => Some(0xa3),
            MusicIsInTheEarOfTheBeholder => Some(0x22e),
            KrabbyPattyPlatforms => Some(0x7f),
            SuperBounce => Some(0x6e),
            HereYouGo => Some(0x32),
            KahRahTae => None,
            TheSmallShallRuleOrNot => None,
        }
    }

    /// Returns the level this spatula is in.
    ///
    /// ```
    /// use bfbb::{Level, Spatula};
    ///
    /// let closet = Spatula::SpongebobsCloset;
    /// let level = closet.get_level();
    /// assert_eq!(level, Level::SpongebobHouse);
    /// ```
    pub fn get_level(&self) -> crate::Level {
        use crate::Level::*;
        use Spatula::*;
        match *self {
            OnTopOfThePineapple => BikiniBottom,
            OnTopOfShadyShoals => BikiniBottom,
            OnTopOfTheChumBucket => BikiniBottom,
            SpongebobsCloset => SpongebobHouse,
            AnnoySquidward => SquidwardHouse,
            AmbushAtTheTreeDome => Treedome,
            InfestationAtTheKrustyKrab => BikiniBottom,
            AWallJumpInTheBucket => ChumBucket,
            TopOfTheHill => JellyfishRock,
            CowaBungee => JellyfishRock,
            Spelunking => JellyfishCaves,
            PatricksDilemma => JellyfishCaves,
            NavigateTheCanyonsAndMesas => JellyfishLake,
            DrainTheLake => JellyfishLake,
            SlideLeap => JellyfishMountain,
            DefeatKingJellyfish => JellyfishRock,
            EndOfTheRoad => DowntownStreets,
            LearnSandysMoves => DowntownStreets,
            TikisGoBoom => DowntownStreets,
            AcrossTheRooftops => DowntownRooftops,
            SwinginSandy => DowntownRooftops,
            AmbushInTheLighthouse => DowntownLighthouse,
            ExtremeBungee => DowntownSeaNeedle,
            ComeBackWithTheCruiseBubble => DowntownSeaNeedle,
            KingOfTheCastle => GooLagoonBeach,
            ConnectTheTowers => GooLagoonBeach,
            SaveTheChildren => GooLagoonBeach,
            OverTheMoat => GooLagoonBeach,
            ThroughTheSeaCaves => GooLagoonCaves,
            CleanOutTheBumperBoats => GooLagoonPier,
            SlipAndSlideUnderThePier => GooLagoonPier,
            TowerBungee => GooLagoonPier,
            RumbleAtThePoseidome => Poseidome,
            GetToTheMuseum => RockBottomDowntown,
            SlipSlidingAway => RockBottomDowntown,
            ReturnTheMuseumsArt => RockBottomDowntown,
            SwingalongSpatula => RockBottomDowntown,
            PlunderingRobotsInTheMuseum => RockBottomMuseum,
            AcrossTheTrenchOfDarkness => RockBottomTrench,
            LasersAreFunAndGoodForYou => RockBottomTrench,
            HowInTarnationDoYouGetThere => RockBottomTrench,
            TopOfTheEntranceAreaML => MermalairEntranceArea,
            TopOfTheComputerArea => MermalairMainChamber,
            ShutDownTheSecuritySystem => MermalairMainChamber,
            TheFunnelMachines => MermalairMainChamber,
            TheSpinningTowersOfPower => MermalairMainChamber,
            TopOfTheSecurityTunnel => MermalairSecurityTunnel,
            CompleteTheRollingBallRoom => MermalairBallroom,
            DefeatPrawn => MermalairVillianContainment,
            FrostyBungee => SandMountainHub,
            TopOfTheLodge => SandMountainHub,
            DefeatRobotsOnGuppyMound => SandMountainSlide1,
            BeatMrsPuffsTime => SandMountainSlide1,
            DefeatRobotsOnFlounderHill => SandMountainSlide2,
            BeatBubbleBuddysTime => SandMountainSlide2,
            DefeatRobotsOnSandMountain => SandMountainSlide3,
            BeatLarrysTime => SandMountainSlide3,
            RoboPatrickAhoy => IndustrialPark,
            ThroughTheWoods => KelpForest,
            FindAllTheLostCampers => KelpForest,
            TikiRoundup => KelpSwamps,
            DownInTheSwamp => KelpSwamps,
            ThroughTheKelpCaves => KelpCaves,
            PowerCrystalCrisis => KelpCaves,
            KelpVineSlide => KelpVines,
            BeatMermaidMansTime => KelpVines,
            TopOfTheEntranceAreaFDG => GraveyardLake,
            APathThroughTheGoo => GraveyardLake,
            GooTankerAhoy => GraveyardLake,
            TopOfTheStackOfShips => GraveyardShipwreck,
            ShipwreckBungee => GraveyardShipwreck,
            DestroyTheRobotShip => GraveyardShip,
            GetAloftThereMatey => GraveyardShip,
            DefeatTheFlyingDutchman => GraveyardBoss,
            AcrossTheDreamscape => SpongebobsDream,
            FollowTheBouncingBall => SpongebobsDream,
            SlidingTexasStyle => SandysDream,
            SwingersAhoy => SandysDream,
            MusicIsInTheEarOfTheBeholder => SquidwardsDream,
            KrabbyPattyPlatforms => KrabsDream,
            SuperBounce => SpongebobsDream,
            HereYouGo => PatricksDream,
            KahRahTae => ChumBucketLab,
            TheSmallShallRuleOrNot => ChumBucketBrain,
        }
    }
}

impl TryFrom<(usize, usize)> for Spatula {
    type Error = &'static str;

    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        use Spatula::*;
        match value {
            (0, 0) => Ok(OnTopOfThePineapple),
            (0, 1) => Ok(OnTopOfShadyShoals),
            (0, 2) => Ok(OnTopOfTheChumBucket),
            (0, 3) => Ok(SpongebobsCloset),
            (0, 4) => Ok(AnnoySquidward),
            (0, 5) => Ok(AmbushAtTheTreeDome),
            (0, 6) => Ok(InfestationAtTheKrustyKrab),
            (0, 7) => Ok(AWallJumpInTheBucket),

            // Jellyfish Fields
            (1, 0) => Ok(TopOfTheHill),
            (1, 1) => Ok(CowaBungee),
            (1, 2) => Ok(Spelunking),
            (1, 3) => Ok(PatricksDilemma),
            (1, 4) => Ok(NavigateTheCanyonsAndMesas),
            (1, 5) => Ok(DrainTheLake),
            (1, 6) => Ok(SlideLeap),
            (1, 7) => Ok(DefeatKingJellyfish),

            // Downtown Bikini Bottom
            (2, 0) => Ok(EndOfTheRoad),
            (2, 1) => Ok(LearnSandysMoves),
            (2, 2) => Ok(TikisGoBoom),
            (2, 3) => Ok(AcrossTheRooftops),
            (2, 4) => Ok(SwinginSandy),
            (2, 5) => Ok(AmbushInTheLighthouse),
            (2, 6) => Ok(ExtremeBungee),
            (2, 7) => Ok(ComeBackWithTheCruiseBubble),

            // Goo Lagoon
            (3, 0) => Ok(KingOfTheCastle),
            (3, 1) => Ok(ConnectTheTowers),
            (3, 2) => Ok(SaveTheChildren),
            (3, 3) => Ok(OverTheMoat),
            (3, 4) => Ok(ThroughTheSeaCaves),
            (3, 5) => Ok(CleanOutTheBumperBoats),
            (3, 6) => Ok(SlipAndSlideUnderThePier),
            (3, 7) => Ok(TowerBungee),

            // Poseidome
            (4, 0) => Ok(RumbleAtThePoseidome),

            // Rock Bottom
            (5, 0) => Ok(GetToTheMuseum),
            (5, 1) => Ok(SlipSlidingAway),
            (5, 2) => Ok(ReturnTheMuseumsArt),
            (5, 3) => Ok(SwingalongSpatula),
            (5, 4) => Ok(PlunderingRobotsInTheMuseum),
            (5, 5) => Ok(AcrossTheTrenchOfDarkness),
            (5, 6) => Ok(LasersAreFunAndGoodForYou),
            (5, 7) => Ok(HowInTarnationDoYouGetThere),

            // Mermalair
            (6, 0) => Ok(TopOfTheEntranceAreaML),
            (6, 1) => Ok(TopOfTheComputerArea),
            (6, 2) => Ok(ShutDownTheSecuritySystem),
            (6, 3) => Ok(TheFunnelMachines),
            (6, 4) => Ok(TheSpinningTowersOfPower),
            (6, 5) => Ok(TopOfTheSecurityTunnel),
            (6, 6) => Ok(CompleteTheRollingBallRoom),
            (6, 7) => Ok(DefeatPrawn),

            // Sand Mountain
            (7, 0) => Ok(FrostyBungee),
            (7, 1) => Ok(TopOfTheLodge),
            (7, 2) => Ok(DefeatRobotsOnGuppyMound),
            (7, 3) => Ok(BeatMrsPuffsTime),
            (7, 4) => Ok(DefeatRobotsOnFlounderHill),
            (7, 5) => Ok(BeatBubbleBuddysTime),
            (7, 6) => Ok(DefeatRobotsOnSandMountain),
            (7, 7) => Ok(BeatLarrysTime),

            // Industrial Park
            (8, 0) => Ok(RoboPatrickAhoy),

            // Kelp Forest
            (9, 0) => Ok(ThroughTheWoods),
            (9, 1) => Ok(FindAllTheLostCampers),
            (9, 2) => Ok(TikiRoundup),
            (9, 3) => Ok(DownInTheSwamp),
            (9, 4) => Ok(ThroughTheKelpCaves),
            (9, 5) => Ok(PowerCrystalCrisis),
            (9, 6) => Ok(KelpVineSlide),
            (9, 7) => Ok(BeatMermaidMansTime),

            // Flying Dutchman's Graveyard
            (10, 0) => Ok(TopOfTheEntranceAreaFDG),
            (10, 1) => Ok(APathThroughTheGoo),
            (10, 2) => Ok(GooTankerAhoy),
            (10, 3) => Ok(TopOfTheStackOfShips),
            (10, 4) => Ok(ShipwreckBungee),
            (10, 5) => Ok(DestroyTheRobotShip),
            (10, 6) => Ok(GetAloftThereMatey),
            (10, 7) => Ok(DefeatTheFlyingDutchman),

            // SpongeBob's Dream
            (11, 0) => Ok(AcrossTheDreamscape),
            (11, 1) => Ok(FollowTheBouncingBall),
            (11, 2) => Ok(SlidingTexasStyle),
            (11, 3) => Ok(SwingersAhoy),
            (11, 4) => Ok(MusicIsInTheEarOfTheBeholder),
            (11, 5) => Ok(KrabbyPattyPlatforms),
            (11, 6) => Ok(SuperBounce),
            (11, 7) => Ok(HereYouGo),

            // Chum Bucket Lab,
            (12, 0) => Ok(KahRahTae),
            (12, 1) => Ok(TheSmallShallRuleOrNot),
            _ => Err("Menu coordinate did not correspond to a Spatula"),
        }
    }
}

impl From<Spatula> for (usize, usize) {
    fn from(spatula: Spatula) -> Self {
        use Spatula::*;
        match spatula {
            // Bikini Bottom
            OnTopOfThePineapple => (0, 0),
            OnTopOfShadyShoals => (0, 1),
            OnTopOfTheChumBucket => (0, 2),
            SpongebobsCloset => (0, 3),
            AnnoySquidward => (0, 4),
            AmbushAtTheTreeDome => (0, 5),
            InfestationAtTheKrustyKrab => (0, 6),
            AWallJumpInTheBucket => (0, 7),

            // Jellyfish Fields
            TopOfTheHill => (1, 0),
            CowaBungee => (1, 1),
            Spelunking => (1, 2),
            PatricksDilemma => (1, 3),
            NavigateTheCanyonsAndMesas => (1, 4),
            DrainTheLake => (1, 5),
            SlideLeap => (1, 6),
            DefeatKingJellyfish => (1, 7),

            // Downtown Bikini Bottom
            EndOfTheRoad => (2, 0),
            LearnSandysMoves => (2, 1),
            TikisGoBoom => (2, 2),
            AcrossTheRooftops => (2, 3),
            SwinginSandy => (2, 4),
            AmbushInTheLighthouse => (2, 5),
            ExtremeBungee => (2, 6),
            ComeBackWithTheCruiseBubble => (2, 7),

            // Goo Lagoon
            KingOfTheCastle => (3, 0),
            ConnectTheTowers => (3, 1),
            SaveTheChildren => (3, 2),
            OverTheMoat => (3, 3),
            ThroughTheSeaCaves => (3, 4),
            CleanOutTheBumperBoats => (3, 5),
            SlipAndSlideUnderThePier => (3, 6),
            TowerBungee => (3, 7),

            // Poseidome
            RumbleAtThePoseidome => (4, 0),

            // Rock Bottom
            GetToTheMuseum => (5, 0),
            SlipSlidingAway => (5, 1),
            ReturnTheMuseumsArt => (5, 2),
            SwingalongSpatula => (5, 3),
            PlunderingRobotsInTheMuseum => (5, 4),
            AcrossTheTrenchOfDarkness => (5, 5),
            LasersAreFunAndGoodForYou => (5, 6),
            HowInTarnationDoYouGetThere => (5, 7),

            // Mermalair
            TopOfTheEntranceAreaML => (6, 0),
            TopOfTheComputerArea => (6, 1),
            ShutDownTheSecuritySystem => (6, 2),
            TheFunnelMachines => (6, 3),
            TheSpinningTowersOfPower => (6, 4),
            TopOfTheSecurityTunnel => (6, 5),
            CompleteTheRollingBallRoom => (6, 6),
            DefeatPrawn => (6, 7),

            // Sand Mountain
            FrostyBungee => (7, 0),
            TopOfTheLodge => (7, 1),
            DefeatRobotsOnGuppyMound => (7, 2),
            BeatMrsPuffsTime => (7, 3),
            DefeatRobotsOnFlounderHill => (7, 4),
            BeatBubbleBuddysTime => (7, 5),
            DefeatRobotsOnSandMountain => (7, 6),
            BeatLarrysTime => (7, 7),

            // Industrial Park
            RoboPatrickAhoy => (8, 0),

            // Kelp Forest
            ThroughTheWoods => (9, 0),
            FindAllTheLostCampers => (9, 1),
            TikiRoundup => (9, 2),
            DownInTheSwamp => (9, 3),
            ThroughTheKelpCaves => (9, 4),
            PowerCrystalCrisis => (9, 5),
            KelpVineSlide => (9, 6),
            BeatMermaidMansTime => (9, 7),

            // Flying Dutchman's Graveyard
            TopOfTheEntranceAreaFDG => (10, 0),
            APathThroughTheGoo => (10, 1),
            GooTankerAhoy => (10, 2),
            TopOfTheStackOfShips => (10, 3),
            ShipwreckBungee => (10, 4),
            DestroyTheRobotShip => (10, 5),
            GetAloftThereMatey => (10, 6),
            DefeatTheFlyingDutchman => (10, 7),

            // Spongebob's Dream
            AcrossTheDreamscape => (11, 0),
            FollowTheBouncingBall => (11, 1),
            SlidingTexasStyle => (11, 2),
            SwingersAhoy => (11, 3),
            MusicIsInTheEarOfTheBeholder => (11, 4),
            KrabbyPattyPlatforms => (11, 5),
            SuperBounce => (11, 6),
            HereYouGo => (11, 7),

            // Chum Bucket Lab
            KahRahTae => (12, 0),
            TheSmallShallRuleOrNot => (12, 1),
        }
    }
}
