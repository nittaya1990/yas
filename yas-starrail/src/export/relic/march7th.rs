use serde::ser::{Serialize, SerializeMap, Serializer};
use std::convert::From;

use crate::relic::{
    RelicSetName, RelicSlot, RelicStat, RelicStatName, StarRailRelic,
};

type March7thRelic = StarRailRelic;

impl RelicStatName {
    pub fn to_march7th(&self) -> String {
        let temp = match self {
            RelicStatName::HP => "hp",
            RelicStatName::HPPercentage => "hp_",
            RelicStatName::ATK => "atk",
            RelicStatName::ATKPercentage => "atk_",
            RelicStatName::DEFPercentage => "def_",
            RelicStatName::SPD => "spd",
            RelicStatName::CRITRate => "critRate",
            RelicStatName::CRITDMG => "critDMG",
            RelicStatName::BreakEffect => "break",
            RelicStatName::OutgoingHealingBoost => "heal",
            RelicStatName::EnergyRegenerationRate => "enerRegen",
            RelicStatName::EffectHitRate => "eff",
            RelicStatName::PhysicalDMGBoost => "physicalDmg",
            RelicStatName::FireDMGBoost => "fireDmg",
            RelicStatName::IceDMGBoost => "iceDmg",
            RelicStatName::LightningDMGBoost => "lightningDmg",
            RelicStatName::WindDMGBoost => "windDmg",
            RelicStatName::QuantumDMGBoost => "quantumDmg",
            RelicStatName::ImaginaryDMGBoost => "imaginaryDmg",
            RelicStatName::DEF => "def",
            RelicStatName::EffectRES => "effRes",
        };
        String::from(temp)
    }
}

impl RelicSetName {
    pub fn to_march7th(&self) -> String {
        let temp = match self {
            RelicSetName::PasserbyofWanderingCloud => "PasserbyofWanderingCloud",
            RelicSetName::MusketeerofWildWheat => "MusketeerofWildWheat",
            RelicSetName::KnightofPurityPalace => "KnightofPurityPalace",
            RelicSetName::HunterofGlacialForest => "HunterofGlacialForest",
            RelicSetName::ChampionofStreetwiseBoxing => "ChampionofStreetwiseBoxing",
            RelicSetName::GuardofWutheringSnow => "GuardofWutheringSnow",
            RelicSetName::FiresmithofLavaForging => "FiresmithofLavaForging",
            RelicSetName::GeniusofBrilliantStars => "GeniusofBrilliantStars",
            RelicSetName::BandofSizzlingThunder => "BandofSizzlingThunder",
            RelicSetName::EagleofTwilightLine => "EagleofTwilightLine",
            RelicSetName::ThiefofShootingMeteor => "ThiefofShootingMeteor",
            RelicSetName::WastelanderofBanditryDesert => "WastelanderofBanditryDesert",
            RelicSetName::LongevousDisciple => "LongevousDisciple",
            RelicSetName::MessengerTraversingHackerspace => "MessengerTraversingHackerspace",
            RelicSetName::TheAshblazingGrandDuke => "TheAshblazingGrandDuke",
            RelicSetName::PrisonerinDeepConfinement => "PrisonerinDeepConfinement",
            RelicSetName::PioneerDiverofDeadWaters => "PioneerDiverofDeadWaters",
            RelicSetName::WatchmakerMasterofDreamMachinations => "WatchmakerMasterofDreamMachinations",

            RelicSetName::SpaceSealingStation => "SpaceSealingStation",
            RelicSetName::FleetoftheAgeless => "FleetoftheAgeless",
            RelicSetName::PanCosmicCommercialEnterprise => "PanCosmicCommercialEnterprise",
            RelicSetName::BelobogoftheArchitects => "BelobogoftheArchitects",
            RelicSetName::CelestialDifferentiator => "CelestialDifferentiator",
            RelicSetName::InertSalsotto => "InertSalsotto",
            RelicSetName::TaliaKingdomofBanditry => "TaliaKingdomofBanditry",
            RelicSetName::SprightlyVonwacq => "SprightlyVonwacq",
            RelicSetName::RutilantArena => "RutilantArena",
            RelicSetName::BrokenKeel => "BrokenKeel",
            RelicSetName::FirmamentFrontlineGlamoth => "FirmamentFrontlineGlamoth",
            RelicSetName::PenaconyLandoftheDreams => "PenaconyLandoftheDreams",
        };
        String::from(temp)
    }
}

impl RelicSlot {
    pub fn to_march7th(&self) -> String {
        let temp = match self {
            RelicSlot::Head => "head",
            RelicSlot::Hands => "hands",
            RelicSlot::Body => "body",
            RelicSlot::Feet => "feet",
            RelicSlot::PlanarSphere => "planarSphere",
            RelicSlot::LinkRope => "linkRope",
        };
        String::from(temp)
    }
}

impl Serialize for RelicStat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(2))?;
        root.serialize_entry("name", &self.name.to_march7th())?;
        root.serialize_entry("value", &self.value)?;
        root.end()
    }
}

impl Serialize for March7thRelic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(7))?;

        root.serialize_entry("setName", &self.set_name.to_march7th())?;
        root.serialize_entry("position", &self.slot.to_march7th())?;
        root.serialize_entry("mainTag", &self.main_stat)?;

        let mut sub_stats: Vec<&RelicStat> = vec![];
        if let Some(ref s) = self.sub_stat_1 {
            sub_stats.push(s);
        }
        if let Some(ref s) = self.sub_stat_2 {
            sub_stats.push(s);
        }
        if let Some(ref s) = self.sub_stat_3 {
            sub_stats.push(s);
        }
        if let Some(ref s) = self.sub_stat_4 {
            sub_stats.push(s);
        }

        root.serialize_entry("normalTags", &sub_stats)?;
        root.serialize_entry("level", &self.level)?;
        root.serialize_entry("star", &self.star)?;
        root.serialize_entry("equip", &self.equip)?;
        root.serialize_entry("lock", &self.lock)?;
        root.serialize_entry("discard", &self.discard)?;

        root.end()
    }
}

pub struct March7thFormat<'a> {
    version: String,
    head: Vec<&'a March7thRelic>,
    hands: Vec<&'a March7thRelic>,
    body: Vec<&'a March7thRelic>,
    feet: Vec<&'a March7thRelic>,
    sphere: Vec<&'a March7thRelic>,
    rope: Vec<&'a March7thRelic>,
}

impl<'a> Serialize for March7thFormat<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut root = serializer.serialize_map(Some(6))?;
        root.serialize_entry("version", &self.version)?;
        root.serialize_entry("head", &self.head)?;
        root.serialize_entry("hands", &self.hands)?;
        root.serialize_entry("body", &self.body)?;
        root.serialize_entry("feet", &self.feet)?;
        root.serialize_entry("planarSphere", &self.sphere)?;
        root.serialize_entry("linkRope", &self.rope)?;
        root.end()
    }
}

impl<'a> March7thFormat<'a> {
    pub fn new(results: &'a [StarRailRelic]) -> March7thFormat {
        let mut head: Vec<&March7thRelic> = Vec::new();
        let mut hands: Vec<&March7thRelic> = Vec::new();
        let mut body: Vec<&March7thRelic> = Vec::new();
        let mut feet: Vec<&March7thRelic> = Vec::new();
        let mut sphere: Vec<&March7thRelic> = Vec::new();
        let mut rope: Vec<&March7thRelic> = Vec::new();

        for relic in results.iter() {
            match relic.slot {
                RelicSlot::Head => head.push(relic),
                RelicSlot::Hands => hands.push(relic),
                RelicSlot::Body => body.push(relic),
                RelicSlot::Feet => feet.push(relic),
                RelicSlot::PlanarSphere => sphere.push(relic),
                RelicSlot::LinkRope => rope.push(relic),
            }
        }

        March7thFormat {
            head,
            hands,
            body,
            feet,
            sphere,
            rope,
            version: String::from("1"),
        }
    }
}