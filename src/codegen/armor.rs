// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::armor.schema;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: armor.schema = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmorSchema {
    /// Absorption values for the Armor.
    absorptions: Absorptions,
    /// Name of the altered (or reversed) armor piece, empty if unalterable.
    altered: String,
    /// Category of the Armor.
    category: ArmorCategory,
    /// Array of lines of the in-game description, each element is separated by
    /// a new line. A line may contain multiple sentences, or be empty if in
    /// between paragraphs.
    description: Vec<String>,
    /// Additional effects of the Armor.
    effects: Vec<Effect>,
    /// Full ID of the Item in capital hexadecimal form. IDs differ per affinity
    /// or upgrade level.
    full_hex_id: String,
    /// ID of the icon which can be shared across many items. Icons can be
    /// sourced from the game files using ERDB.
    icon: i64,
    /// Icon ID to the female version of the Armor, `icon` field specifies the
    /// male version which is usually the same.
    icon_fem: i64,
    /// ID of the Item in its individual class. IDs differ per affinity or
    /// upgrade level.
    id: i64,
    /// Specifies whether the Item is visible to other players if dropped.
    is_tradable: bool,
    /// List of locations in which this Item appears.
    locations: Option<Vec<LocationDetail>>,
    /// The maximum amount of the Item that a player can have on them.
    max_held: i64,
    /// The maximum amount of the Item that can be stored in the sort chest.
    max_stored: i64,
    /// Name of the Item.
    name: String,
    /// The amount of Runes the Item is sold for, 0 if not applicabe.
    price_sold: i64,
    /// Rarity of the Item.
    rarity: GoodsRarity,
    /// List of remarks and trivia about this item.
    remarks: Option<Vec<String>>,
    /// Resistance values for the Armor.
    resistances: Resistances,
    /// Short description of the Item.
    summary: String,
    /// Weight of the Armor.
    weight: f64,
}

/// Absorption values for the Armor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Absorptions {
    fire: f64,
    holy: f64,
    lightning: f64,
    magic: f64,
    physical: f64,
    pierce: f64,
    slash: f64,
    strike: f64,
}

/// Category of the Armor.
///
/// An enumeration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArmorCategory {
    Arms,
    Body,
    Head,
    Legs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    /// Specific attribute this effect alters.
    attribute: AttributeName,
    /// List of conditions which trigger the effect, none for passive effects.
    conditions: Option<Vec<String>>,
    /// Specifies whether the value is multiplicative (ex. rune acquisition) or
    /// additive (ex. +5 strength).
    model: Option<EffectModel>,
    /// Interval in seconds on how often the effect gets applied.
    tick_interval: Option<f64>,
    /// The kind of the effect, considering the whole context (`value` *alone*
    /// can mean different things depending on `attribute` and `model`).
    #[serde(rename = "type")]
    effect_type: Option<EffectType>,
    /// Value modifying the attribute.
    value: f64,
    /// Optional modifying value when used in PvP scenario.
    value_pvp: Option<f64>,
}

/// Specific attribute this effect alters.
///
/// An enumeration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttributeName {
    Absorption,
    #[serde(rename = "Appear as Cooperator")]
    AppearAsCooperator,
    #[serde(rename = "Appear as Host")]
    AppearAsHost,
    Arcane,
    #[serde(rename = "Attack Power")]
    AttackPower,
    #[serde(rename = "Attract Enemy Aggression")]
    AttractEnemyAggression,
    #[serde(rename = "Bleed Resistance")]
    BleedResistance,
    #[serde(rename = "Bow Distance")]
    BowDistance,
    #[serde(rename = "Casting Speed")]
    CastingSpeed,
    #[serde(rename = "Death Blight Resistance")]
    DeathBlightResistance,
    #[serde(rename = "Destroy Item on Death")]
    DestroyItemOnDeath,
    Dexterity,
    #[serde(rename = "Elemental Absorption")]
    ElementalAbsorption,
    #[serde(rename = "Elemental Attack Power")]
    ElementalAttackPower,
    Endurance,
    #[serde(rename = "Enemy Hearing")]
    EnemyHearing,
    Faith,
    #[serde(rename = "Fall Damage")]
    FallDamage,
    #[serde(rename = "Fire Absorption")]
    FireAbsorption,
    #[serde(rename = "Fire Attack Power")]
    FireAttackPower,
    #[serde(rename = "Flask Focus Restoration")]
    FlaskFocusRestoration,
    #[serde(rename = "Flask Health Restoration")]
    FlaskHealthRestoration,
    Focus,
    #[serde(rename = "Focus Points")]
    FocusPoints,
    #[serde(rename = "Frostbite Resistance")]
    FrostbiteResistance,
    #[serde(rename = "Health Points")]
    HealthPoints,
    #[serde(rename = "Holy Absorption")]
    HolyAbsorption,
    #[serde(rename = "Holy Attack Power")]
    HolyAttackPower,
    Immunity,
    #[serde(rename = "Incantation Focus Consumption")]
    IncantationFocusConsumption,
    Intelligence,
    #[serde(rename = "Invisible at Distance")]
    InvisibleAtDistance,
    #[serde(rename = "Item Discovery")]
    ItemDiscovery,
    #[serde(rename = "Lightning Absorption")]
    LightningAbsorption,
    #[serde(rename = "Lightning Attack Power")]
    LightningAttackPower,
    #[serde(rename = "Madness Resistance")]
    MadnessResistance,
    #[serde(rename = "Magic Absorption")]
    MagicAbsorption,
    #[serde(rename = "Magic Attack Power")]
    MagicAttackPower,
    #[serde(rename = "Maximum Equip Load")]
    MaximumEquipLoad,
    #[serde(rename = "Maximum Focus")]
    MaximumFocus,
    #[serde(rename = "Maximum Health")]
    MaximumHealth,
    #[serde(rename = "Maximum Stamina")]
    MaximumStamina,
    #[serde(rename = "Memory Slots")]
    MemorySlots,
    Mind,
    #[serde(rename = "Physical Absorption")]
    PhysicalAbsorption,
    #[serde(rename = "Physical Attack Power")]
    PhysicalAttackPower,
    #[serde(rename = "Pierce Absorption")]
    PierceAbsorption,
    #[serde(rename = "Pierce Attack Power")]
    PierceAttackPower,
    Poise,
    #[serde(rename = "Poison Resistance")]
    PoisonResistance,
    #[serde(rename = "Preserve Runes on Death")]
    PreserveRunesOnDeath,
    #[serde(rename = "Pyromancy Focus Consumption")]
    PyromancyFocusConsumption,
    #[serde(rename = "Reduce Headshot Impact")]
    ReduceHeadshotImpact,
    Robustness,
    #[serde(rename = "Rune Acquisition")]
    RuneAcquisition,
    #[serde(rename = "Scarlet Rot Resistance")]
    ScarletRotResistance,
    #[serde(rename = "Skill Focus Consumption")]
    SkillFocusConsumption,
    #[serde(rename = "Slash Absorption")]
    SlashAbsorption,
    #[serde(rename = "Slash Attack Power")]
    SlashAttackPower,
    #[serde(rename = "Sleep Resistance")]
    SleepResistance,
    #[serde(rename = "Sorcery Focus Consumption")]
    SorceryFocusConsumption,
    #[serde(rename = "Spell Duration")]
    SpellDuration,
    #[serde(rename = "Spell Focus Consumption")]
    SpellFocusConsumption,
    Stability,
    #[serde(rename = "Stamina Attack Rate")]
    StaminaAttackRate,
    #[serde(rename = "Stamina Recovery Speed")]
    StaminaRecoverySpeed,
    #[serde(rename = "Standard Absorption")]
    StandardAbsorption,
    #[serde(rename = "Standard Attack Power")]
    StandardAttackPower,
    Strength,
    #[serde(rename = "Strike Absorption")]
    StrikeAbsorption,
    #[serde(rename = "Strike Attack Power")]
    StrikeAttackPower,
    #[serde(rename = "Switch Animation Gender")]
    SwitchAnimationGender,
    Vigor,
    Vitality,
}

/// The kind of the effect, considering the whole context (`value` *alone* can
/// mean different things depending on `attribute` and `model`).
///
/// An enumeration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectType {
    Negative,
    Neutral,
    Positive,
}

/// Specifies whether the value is multiplicative (ex. rune acquisition) or
/// additive (ex. +5 strength).
///
/// An enumeration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectModel {
    Additive,
    Multiplicative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationDetail {
    /// List of situations which cause the item to become unavailable in full
    /// sentences.
    blockers: Option<Vec<String>>,
    /// The type of currency this item is bought for, if applicable.
    currency: Option<Currency>,
    /// Exact description on where to find the Item if summary cannot be
    /// straightfoward enough.
    directions: Option<String>,
    /// The specific location in which the Item is found.
    location: Option<Location>,
    /// The amount of Currency the Item is bought for at this location, if
    /// applicable.
    price_bought: Option<i64>,
    /// Specifies the amount if an integer, otherwise `infinite` if the Item
    /// respawns or can be purchased infinitely.
    quantity: Option<Quantity>,
    /// The generic region in which the Item is found.
    region: Option<Region>,
    /// List of requirements which make the item available in full sentences.
    requirements: Option<Vec<String>>,
    /// Short, consice summary of the location. To help concatenating with other
    /// data, there are no capital letters or periods at the end.
    summary: Option<String>,
}

/// The type of currency this item is bought for, if applicable.
///
/// An enumeration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "Dragon Hearts")]
    DragonHearts,
    #[serde(rename = "Lost Ashes of War")]
    LostAshesOfWar,
    Runes,
    #[serde(rename = "Starlight Shards")]
    StarlightShards,
}

/// The specific location in which the Item is found.
///
/// An enumeration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Location {
    #[serde(rename = "Abandoned Cave")]
    AbandonedCave,
    #[serde(rename = "Academy Crystal Cave")]
    AcademyCrystalCave,
    #[serde(rename = "Academy Gate Town")]
    AcademyGateTown,
    #[serde(rename = "Academy of Raya Lucaria")]
    AcademyOfRayaLucaria,
    #[serde(rename = "Ailing Village")]
    AilingVillage,
    #[serde(rename = "Ainsel River Well")]
    AinselRiverWell,
    #[serde(rename = "Albinauric Rise")]
    AlbinauricRise,
    #[serde(rename = "Altus Tunnel")]
    AltusTunnel,
    #[serde(rename = "Apostate Derelict")]
    ApostateDerelict,
    #[serde(rename = "Artist's Shack (1)")]
    ArtistSShack1,
    #[serde(rename = "Artist's Shack (2)")]
    ArtistSShack2,
    #[serde(rename = "Auriza Hero's Grave")]
    AurizaHeroSGrave,
    #[serde(rename = "Auriza Side Tomb")]
    AurizaSideTomb,
    #[serde(rename = "Bellum Church")]
    BellumChurch,
    #[serde(rename = "Bestial Sanctum")]
    BestialSanctum,
    #[serde(rename = "Black Knife Catacombs")]
    BlackKnifeCatacombs,
    #[serde(rename = "Boilprawn Shack")]
    BoilprawnShack,
    #[serde(rename = "Bridge of Sacrifice")]
    BridgeOfSacrifice,
    #[serde(rename = "Caelem Ruins")]
    CaelemRuins,
    #[serde(rename = "Caelid Catacombs")]
    CaelidCatacombs,
    #[serde(rename = "Caelid Waypoint Ruins")]
    CaelidWaypointRuins,
    #[serde(rename = "Callu Baptismal Church")]
    CalluBaptismalChurch,
    #[serde(rename = "Caria Manor")]
    CariaManor,
    #[serde(rename = "Carian Study Hall")]
    CarianStudyHall,
    #[serde(rename = "Castle Morne")]
    CastleMorne,
    #[serde(rename = "Castle Sol")]
    CastleSol,
    #[serde(rename = "Cathedral of Dragon Communion")]
    CathedralOfDragonCommunion,
    #[serde(rename = "Cathedral of Manus Celes")]
    CathedralOfManusCeles,
    #[serde(rename = "Cave of the Forlorn")]
    CaveOfTheForlorn,
    #[serde(rename = "Chelona's Rise")]
    ChelonaSRise,
    #[serde(rename = "Church of Dragon Communion")]
    ChurchOfDragonCommunion,
    #[serde(rename = "Church of Elleh")]
    ChurchOfElleh,
    #[serde(rename = "Church of Inhibition")]
    ChurchOfInhibition,
    #[serde(rename = "Church of Irith")]
    ChurchOfIrith,
    #[serde(rename = "Church of Pilgrimage")]
    ChurchOfPilgrimage,
    #[serde(rename = "Church of Repose")]
    ChurchOfRepose,
    #[serde(rename = "Church of the Plague")]
    ChurchOfThePlague,
    #[serde(rename = "Church of Vows")]
    ChurchOfVows,
    #[serde(rename = "Cliffbottom Catacombs")]
    CliffbottomCatacombs,
    #[serde(rename = "Coastal Cave")]
    CoastalCave,
    #[serde(rename = "Consecrated Snowfield Catacombs")]
    ConsecratedSnowfieldCatacombs,
    #[serde(rename = "Converted Fringe Tower")]
    ConvertedFringeTower,
    #[serde(rename = "Converted Tower")]
    ConvertedTower,
    #[serde(rename = "Corpse-Stench Shack")]
    CorpseStenchShack,
    #[serde(rename = "Craftsman's Shack")]
    CraftsmanSShack,
    #[serde(rename = "Crumbling Farum Azula")]
    CrumblingFarumAzula,
    #[serde(rename = "Cuckoo's Evergaol")]
    CuckooSEvergaol,
    #[serde(rename = "Deathtouched Catacombs")]
    DeathtouchedCatacombs,
    #[serde(rename = "Deep Ainsel Well")]
    DeepAinselWell,
    #[serde(rename = "Deep Siofra Well")]
    DeepSiofraWell,
    #[serde(rename = "Demi-Human Forest Ruins")]
    DemiHumanForestRuins,
    #[serde(rename = "Divine Tower of Caelid")]
    DivineTowerOfCaelid,
    #[serde(rename = "Divine Tower of East Altus")]
    DivineTowerOfEastAltus,
    #[serde(rename = "Divine Tower of Limgrave")]
    DivineTowerOfLimgrave,
    #[serde(rename = "Divine Tower of Liurnia")]
    DivineTowerOfLiurnia,
    #[serde(rename = "Divine Tower of West Altus")]
    DivineTowerOfWestAltus,
    #[serde(rename = "Dominula, Windmill Village")]
    DominulaWindmillVillage,
    #[serde(rename = "Dragon-Burnt Ruins")]
    DragonBurntRuins,
    #[serde(rename = "Dragonbarrow Cave")]
    DragonbarrowCave,
    #[serde(rename = "Earthbore Cave")]
    EarthboreCave,
    #[serde(rename = "East Windmill Pasture")]
    EastWindmillPasture,
    #[serde(rename = "Elphael, Brace of the Haligtree")]
    ElphaelBraceOfTheHaligtree,
    #[serde(rename = "First Church of Marika")]
    FirstChurchOfMarika,
    #[serde(rename = "Forest Lookout Tower")]
    ForestLookoutTower,
    #[serde(rename = "Forge of the Giants")]
    ForgeOfTheGiants,
    #[serde(rename = "Forlorn Hound Evergaol")]
    ForlornHoundEvergaol,
    #[serde(rename = "Forsaken Ruins")]
    ForsakenRuins,
    #[serde(rename = "Fort Faroth")]
    FortFaroth,
    #[serde(rename = "Fort Gael")]
    FortGael,
    #[serde(rename = "Fort Haight")]
    FortHaight,
    #[serde(rename = "Fort Laiedd")]
    FortLaiedd,
    #[serde(rename = "Fourth Church of Marika")]
    FourthChurchOfMarika,
    #[serde(rename = "Frenzied Flame Village")]
    FrenziedFlameVillage,
    #[serde(rename = "Frenzy-Flaming Tower")]
    FrenzyFlamingTower,
    #[serde(rename = "Fringefolk Hero's Grave")]
    FringefolkHeroSGrave,
    #[serde(rename = "Gael Tunnel")]
    GaelTunnel,
    #[serde(rename = "Gaol Cave")]
    GaolCave,
    #[serde(rename = "Gatefront Ruins")]
    GatefrontRuins,
    #[serde(rename = "Gelmir Hero's Grave")]
    GelmirHeroSGrave,
    #[serde(rename = "Giant-Conquering Hero's Grave")]
    GiantConqueringHeroSGrave,
    #[serde(rename = "Giants' Mountaintop Catacombs")]
    GiantsMountaintopCatacombs,
    #[serde(rename = "Golden Lineage Evergaol")]
    GoldenLineageEvergaol,
    #[serde(rename = "Gowry's Shack")]
    GowrySShack,
    #[serde(rename = "Grand Cloister")]
    GrandCloister,
    #[serde(rename = "Grand Lift of Dectus")]
    GrandLiftOfDectus,
    #[serde(rename = "Grand Lift of Rold")]
    GrandLiftOfRold,
    #[serde(rename = "Groveside Cave")]
    GrovesideCave,
    #[serde(rename = "Guardians' Garrison")]
    GuardiansGarrison,
    #[serde(rename = "Hallowhorn Grounds (1)")]
    HallowhornGrounds1,
    #[serde(rename = "Hallowhorn Grounds (2)")]
    HallowhornGrounds2,
    #[serde(rename = "Heretical Rise")]
    HereticalRise,
    #[serde(rename = "Hermit Merchant's Shack")]
    HermitMerchantSShack,
    #[serde(rename = "Hermit's Shack")]
    HermitSShack,
    #[serde(rename = "Hermit Village")]
    HermitVillage,
    #[serde(rename = "Hidden Path to the Haligtree")]
    HiddenPathToTheHaligtree,
    #[serde(rename = "Highroad Cave")]
    HighroadCave,
    #[serde(rename = "Highway Lookout Tower (1)")]
    HighwayLookoutTower1,
    #[serde(rename = "Highway Lookout Tower (2)")]
    HighwayLookoutTower2,
    #[serde(rename = "Impaler's Catacombs")]
    ImpalerSCatacombs,
    #[serde(rename = "Isolated Divine Tower")]
    IsolatedDivineTower,
    #[serde(rename = "Isolated Merchant's Shack (1)")]
    IsolatedMerchantSShack1,
    #[serde(rename = "Isolated Merchant's Shack (2)")]
    IsolatedMerchantSShack2,
    Jarburg,
    #[serde(rename = "Kingsrealm Ruins")]
    KingsrealmRuins,
    #[serde(rename = "Lakeside Crystal Cave")]
    LakesideCrystalCave,
    #[serde(rename = "Laskyar Ruins")]
    LaskyarRuins,
    #[serde(rename = "Lenne's Rise")]
    LenneSRise,
    #[serde(rename = "Leyndell, Royal Capital")]
    LeyndellRoyalCapital,
    #[serde(rename = "Limgrave Tunnels")]
    LimgraveTunnels,
    #[serde(rename = "Lord Contender's Evergaol")]
    LordContenderSEvergaol,
    #[serde(rename = "Lunar Estate Ruins")]
    LunarEstateRuins,
    #[serde(rename = "Lux Ruins")]
    LuxRuins,
    #[serde(rename = "Malefactor's Evergaol")]
    MalefactorSEvergaol,
    #[serde(rename = "Minor Erdtree (1)")]
    MinorErdtree1,
    #[serde(rename = "Minor Erdtree (10)")]
    MinorErdtree10,
    #[serde(rename = "Minor Erdtree (11)")]
    MinorErdtree11,
    #[serde(rename = "Minor Erdtree (2)")]
    MinorErdtree2,
    #[serde(rename = "Minor Erdtree (3)")]
    MinorErdtree3,
    #[serde(rename = "Minor Erdtree (4)")]
    MinorErdtree4,
    #[serde(rename = "Minor Erdtree (5)")]
    MinorErdtree5,
    #[serde(rename = "Minor Erdtree (6)")]
    MinorErdtree6,
    #[serde(rename = "Minor Erdtree (7)")]
    MinorErdtree7,
    #[serde(rename = "Minor Erdtree (8)")]
    MinorErdtree8,
    #[serde(rename = "Minor Erdtree (9)")]
    MinorErdtree9,
    #[serde(rename = "Minor Erdtree Catacombs")]
    MinorErdtreeCatacombs,
    #[serde(rename = "Minor Erdtree Church")]
    MinorErdtreeChurch,
    #[serde(rename = "Miquella's Haligtree")]
    MiquellaSHaligtree,
    #[serde(rename = "Mirage Rise")]
    MirageRise,
    #[serde(rename = "Mistwood Ruins")]
    MistwoodRuins,
    #[serde(rename = "Mohgwyn Dynasty Mausoleum")]
    MohgwynDynastyMausoleum,
    #[serde(rename = "Moonfolk Ruins")]
    MoonfolkRuins,
    #[serde(rename = "Morne Tunnel")]
    MorneTunnel,
    #[serde(rename = "Murkwater Catacombs")]
    MurkwaterCatacombs,
    #[serde(rename = "Murkwater Cave")]
    MurkwaterCave,
    #[serde(rename = "Night's Sacred Ground")]
    NightSSacredGround,
    #[serde(rename = "Nokron, Eternal City")]
    NokronEternalCity,
    #[serde(rename = "Nokstella, Eternal City")]
    NokstellaEternalCity,
    #[serde(rename = "Old Altus Tunnel")]
    OldAltusTunnel,
    #[serde(rename = "Ordina, Liturgical Town")]
    OrdinaLiturgicalTown,
    #[serde(rename = "Oridys's Rise")]
    OridysSRise,
    #[serde(rename = "Perfumer's Grotto")]
    PerfumerSGrotto,
    #[serde(rename = "Perfumer's Ruins")]
    PerfumerSRuins,
    #[serde(rename = "Purified Ruins")]
    PurifiedRuins,
    #[serde(rename = "Ranni's Rise")]
    RanniSRise,
    #[serde(rename = "Raya Lucaria Crystal Tunnel")]
    RayaLucariaCrystalTunnel,
    #[serde(rename = "Redmane Castle")]
    RedmaneCastle,
    #[serde(rename = "Renna's Rise")]
    RennaSRise,
    #[serde(rename = "Revenger's Shack")]
    RevengerSShack,
    #[serde(rename = "Ringleader's Evergaol")]
    RingleaderSEvergaol,
    #[serde(rename = "Road's End Catacombs")]
    RoadSEndCatacombs,
    #[serde(rename = "Rose Church")]
    RoseChurch,
    #[serde(rename = "Royal Grave Evergaol")]
    RoyalGraveEvergaol,
    #[serde(rename = "Ruin-Strewn Precipice")]
    RuinStrewnPrecipice,
    #[serde(rename = "Sage's Cave")]
    SageSCave,
    #[serde(rename = "Sainted Hero's Grave")]
    SaintedHeroSGrave,
    #[serde(rename = "Sealed Tunnel")]
    SealedTunnel,
    #[serde(rename = "Second Church of Marika")]
    SecondChurchOfMarika,
    #[serde(rename = "Seethewater Cave")]
    SeethewaterCave,
    #[serde(rename = "Sellia Crystal Tunnel")]
    SelliaCrystalTunnel,
    #[serde(rename = "Sellia Evergaol")]
    SelliaEvergaol,
    #[serde(rename = "Sellia Gateway")]
    SelliaGateway,
    #[serde(rename = "Sellia Hideaway")]
    SelliaHideaway,
    #[serde(rename = "Sellia, Town of Sorcery")]
    SelliaTownOfSorcery,
    #[serde(rename = "Seluvis's Rise")]
    SeluvisSRise,
    #[serde(rename = "Shack of the Lofty")]
    ShackOfTheLofty,
    #[serde(rename = "Shack of the Rotting")]
    ShackOfTheRotting,
    #[serde(rename = "Siofra Aqueduct")]
    SiofraAqueduct,
    #[serde(rename = "Siofra River Well")]
    SiofraRiverWell,
    #[serde(rename = "Slumbering Wolf's Shack")]
    SlumberingWolfSShack,
    #[serde(rename = "Smoldering Church")]
    SmolderingChurch,
    #[serde(rename = "Spiritcaller's Cave")]
    SpiritcallerSCave,
    #[serde(rename = "St. Trina's Hideaway")]
    StTrinaSHideaway,
    #[serde(rename = "Stargazers' Ruins")]
    StargazersRuins,
    #[serde(rename = "Stillwater Cave")]
    StillwaterCave,
    #[serde(rename = "Stormcaller Church")]
    StormcallerChurch,
    #[serde(rename = "Stormfoot Catacombs")]
    StormfootCatacombs,
    Stormgate,
    #[serde(rename = "Stormhill Evergaol")]
    StormhillEvergaol,
    #[serde(rename = "Stormhill Shack")]
    StormhillShack,
    #[serde(rename = "Stormveil Castle")]
    StormveilCastle,
    #[serde(rename = "Stranded Graveyard")]
    StrandedGraveyard,
    #[serde(rename = "Street of Sages Ruins")]
    StreetOfSagesRuins,
    #[serde(rename = "Subterranean Shunning-Grounds")]
    SubterraneanShunningGrounds,
    #[serde(rename = "Summonwater Village")]
    SummonwaterVillage,
    #[serde(rename = "Swamp Lookout Tower")]
    SwampLookoutTower,
    #[serde(rename = "Temple Quarter")]
    TempleQuarter,
    #[serde(rename = "Testu's Rise")]
    TestuSRise,
    #[serde(rename = "The Four Belfries")]
    TheFourBelfries,
    #[serde(rename = "The Shaded Castle")]
    TheShadedCastle,
    #[serde(rename = "Third Church of Marika")]
    ThirdChurchOfMarika,
    #[serde(rename = "Three Sisters")]
    ThreeSisters,
    #[serde(rename = "Tombsward Catacombs")]
    TombswardCatacombs,
    #[serde(rename = "Tombsward Cave")]
    TombswardCave,
    #[serde(rename = "Tombsward Ruins")]
    TombswardRuins,
    #[serde(rename = "Tower of Return")]
    TowerOfReturn,
    #[serde(rename = "Uhl Palace Ruins (1)")]
    UhlPalaceRuins1,
    #[serde(rename = "Uhl Palace Ruins (2)")]
    UhlPalaceRuins2,
    #[serde(rename = "Uld Palace Ruins")]
    UldPalaceRuins,
    #[serde(rename = "Unsightly Catacombs")]
    UnsightlyCatacombs,
    #[serde(rename = "Village of the Albinaurics")]
    VillageOfTheAlbinaurics,
    #[serde(rename = "Village Windmill Pasture")]
    VillageWindmillPasture,
    #[serde(rename = "Volcano Cave")]
    VolcanoCave,
    #[serde(rename = "Volcano Manor")]
    VolcanoManor,
    #[serde(rename = "Wailing Dunes")]
    WailingDunes,
    #[serde(rename = "War-Dead Catacombs")]
    WarDeadCatacombs,
    #[serde(rename = "Warmaster's Shack")]
    WarmasterSShack,
    #[serde(rename = "Waypoint Ruins")]
    WaypointRuins,
    #[serde(rename = "Weeping Evergaol")]
    WeepingEvergaol,
    #[serde(rename = "West Windmill Pasture")]
    WestWindmillPasture,
    #[serde(rename = "Witchbane Ruins")]
    WitchbaneRuins,
    #[serde(rename = "Woodfolk Ruins")]
    WoodfolkRuins,
    #[serde(rename = "Writheblood Ruins")]
    WrithebloodRuins,
    #[serde(rename = "Wyndham Catacombs")]
    WyndhamCatacombs,
    #[serde(rename = "Wyndham Ruins")]
    WyndhamRuins,
    #[serde(rename = "Yelough Anix Ruins")]
    YeloughAnixRuins,
    #[serde(rename = "Yelough Anix Tunnel")]
    YeloughAnixTunnel,
    #[serde(rename = "Zamor Ruins")]
    ZamorRuins,
}

/// Specifies the amount if an integer, otherwise `infinite` if the Item
/// respawns or can be purchased infinitely.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Quantity {
    Enum(QuantityEnum),
    Integer(i64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuantityEnum {
    Infinite,
}

/// The generic region in which the Item is found.
///
/// An enumeration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Region {
    #[serde(rename = "Ainsel River")]
    AinselRiver,
    #[serde(rename = "Altus Plateau")]
    AltusPlateau,
    Caelid,
    #[serde(rename = "Consecrated Snowfield")]
    ConsecratedSnowfield,
    #[serde(rename = "Deeproot Depths")]
    DeeprootDepths,
    Dragonbarrow,
    #[serde(rename = "Lake of Rot")]
    LakeOfRot,
    Limgrave,
    #[serde(rename = "Liurnia of the Lakes")]
    LiurniaOfTheLakes,
    #[serde(rename = "Mountaintops of the Giants")]
    MountaintopsOfTheGiants,
    #[serde(rename = "Mt. Gelmir")]
    MtGelmir,
    #[serde(rename = "Roundtable Hold")]
    RoundtableHold,
    #[serde(rename = "Siofra River")]
    SiofraRiver,
    #[serde(rename = "Weeping Peninsula")]
    WeepingPeninsula,
}

/// Rarity of the Item.
///
/// An enumeration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoodsRarity {
    Common,
    Legendary,
    Rare,
}

/// Resistance values for the Armor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resistances {
    focus: i64,
    immunity: i64,
    poise: i64,
    robustness: i64,
    vitality: i64,
}
