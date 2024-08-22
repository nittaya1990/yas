use std::hash::{Hash, Hasher};

use log::error;
use regex::Regex;
use strum_macros::Display;

use crate::character::CHARACTER_NAMES;
use crate::scanner::GenshinArtifactScanResult;

#[derive(Debug, Hash, Clone, PartialEq, Eq, Display)]
pub enum ArtifactStatName {
    HealingBonus,
    CriticalDamage,
    Critical,
    Atk,
    AtkPercentage,
    ElementalMastery,
    Recharge,
    HpPercentage,
    Hp,
    DefPercentage,
    Def,
    ElectroBonus,
    PyroBonus,
    HydroBonus,
    CryoBonus,
    AnemoBonus,
    GeoBonus,
    PhysicalBonus,
    DendroBonus,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Display)]
pub enum ArtifactSlot {
    Flower,
    Feather,
    Sand,
    Goblet,
    Head,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Display)]
pub enum ArtifactSetName {
    ArchaicPetra,
    HeartOfDepth,
    BlizzardStrayer,
    RetracingBolide,
    NoblesseOblige,
    GladiatorFinale,
    MaidenBeloved,
    ViridescentVenerer,
    LavaWalker,
    CrimsonWitch,
    ThunderSmoother,
    ThunderingFury,
    BloodstainedChivalry,
    WandererTroupe,
    Scholar,
    Gambler,
    TinyMiracle,
    MartialArtist,
    BraveHeart,
    ResolutionOfSojourner,
    DefenderWill,
    Berserker,
    Instructor,
    Exile,
    Adventurer,
    LuckyDog,
    TravelingDoctor,
    PrayersForWisdom,
    PrayersToSpringtime,
    PrayersForIllumination,
    PrayersForDestiny,
    PaleFlame,
    TenacityOfTheMillelith,
    EmblemOfSeveredFate,
    ShimenawaReminiscence,
    HuskOfOpulentDreams,
    OceanHuedClam,
    VermillionHereafter,
    EchoesOfAnOffering,
    DeepwoodMemories,
    GildedDreams,
    FlowerOfParadiseLost,
    DesertPavilionChronicle,
    NymphsDream,
    VourukashasGlow,
    MarechausseeHunter,
    GoldenTroupe,
    SongOfDaysPast,
    NighttimeWhispersInTheEchoingWoods,
    FragmentOfHarmonicWhimsy,
    UnfinishedReverie,
}

#[derive(Debug, Clone)]
pub struct ArtifactStat {
    pub name: ArtifactStatName,
    pub value: f64,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct GenshinArtifact {
    pub set_name: ArtifactSetName,
    pub slot: ArtifactSlot,
    pub star: i32,
    pub lock: bool,
    pub level: i32,
    pub main_stat: ArtifactStat,
    pub sub_stat_1: Option<ArtifactStat>,
    pub sub_stat_2: Option<ArtifactStat>,
    pub sub_stat_3: Option<ArtifactStat>,
    pub sub_stat_4: Option<ArtifactStat>,
    pub equip: Option<String>,
}

impl Hash for ArtifactStat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        let v = (self.value * 1000.0) as i32;
        v.hash(state);
    }
}

impl PartialEq for ArtifactStat {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }

        let v1 = (self.value * 1000.0) as i32;
        let v2 = (other.value * 1000.0) as i32;

        v1 == v2
    }
}

impl Eq for ArtifactStat {}

impl ArtifactStatName {
    #[rustfmt::skip]
    pub fn from_zh_cn(name: &str, is_percentage: bool) -> Option<ArtifactStatName> {
        match name {
            "治疗加成" => Some(ArtifactStatName::HealingBonus),
            "暴击伤害" => Some(ArtifactStatName::CriticalDamage),
            "暴击率" => Some(ArtifactStatName::Critical),
            "攻击力" => if is_percentage { Some(ArtifactStatName::AtkPercentage) } else { Some(ArtifactStatName::Atk) },
            "元素精通" => Some(ArtifactStatName::ElementalMastery),
            "元素充能效率" => Some(ArtifactStatName::Recharge),
            "生命值" => if is_percentage { Some(ArtifactStatName::HpPercentage) } else { Some(ArtifactStatName::Hp) },
            "防御力" => if is_percentage { Some(ArtifactStatName::DefPercentage) } else { Some(ArtifactStatName::Def) },
            "雷元素伤害加成" => Some(ArtifactStatName::ElectroBonus),
            "火元素伤害加成" => Some(ArtifactStatName::PyroBonus),
            "水元素伤害加成" => Some(ArtifactStatName::HydroBonus),
            "冰元素伤害加成" => Some(ArtifactStatName::CryoBonus),
            "风元素伤害加成" => Some(ArtifactStatName::AnemoBonus),
            "岩元素伤害加成" => Some(ArtifactStatName::GeoBonus),
            "草元素伤害加成" => Some(ArtifactStatName::DendroBonus),
            "物理伤害加成" => Some(ArtifactStatName::PhysicalBonus),
            _ => None,
        }
    }
}

impl ArtifactStat {
    // e.g "生命值+4,123", "暴击率+10%"
    pub fn from_zh_cn_raw(s: &str) -> Option<ArtifactStat> {
        let temp: Vec<&str> = s.split('+').collect();
        if temp.len() != 2 {
            return None;
        }

        let is_percentage = temp[1].contains('%');
        let stat_name = match ArtifactStatName::from_zh_cn(temp[0], is_percentage) {
            Some(v) => v,
            None => return None,
        };

        let re = Regex::new("[%,]").unwrap();
        let mut value = match re.replace_all(temp[1], "").parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                error!("stat `{}` parse error", s);
                return None;
            },
        };
        if is_percentage {
            value /= 100.0;
        }

        Some(ArtifactStat {
            name: stat_name,
            value,
        })
    }
}

impl TryFrom<&GenshinArtifactScanResult> for GenshinArtifact {
    type Error = ();

    fn try_from(value: &GenshinArtifactScanResult) -> Result<Self, Self::Error> {
        let set_name = ArtifactSetName::from_zh_cn(&value.name).ok_or(())?;
        let slot = ArtifactSlot::from_zh_cn(&value.name).ok_or(())?;
        let star = value.star;
        let lock = value.lock;

        let main_stat = ArtifactStat::from_zh_cn_raw(
            (value.main_stat_name.clone() + "+" + value.main_stat_value.as_str()).as_str(),
        )
        .ok_or(())?;

        let sub1 = ArtifactStat::from_zh_cn_raw(&value.sub_stat[0]);
        let sub2 = ArtifactStat::from_zh_cn_raw(&value.sub_stat[1]);
        let sub3 = ArtifactStat::from_zh_cn_raw(&value.sub_stat[2]);
        let sub4 = ArtifactStat::from_zh_cn_raw(&value.sub_stat[3]);

        let equip = if value.equip.ends_with("已装备") {
            let chars = value.equip.chars().collect::<Vec<_>>();
            let equip_name = chars[..chars.len() - 3].iter().collect::<String>();

            if CHARACTER_NAMES.contains(equip_name.as_str()) {
                Some(equip_name)
            } else {
                None
            }
        } else {
            None
        };

        Ok(GenshinArtifact {
            set_name,
            slot,
            star,
            lock,
            level: value.level,
            main_stat,
            sub_stat_1: sub1,
            sub_stat_2: sub2,
            sub_stat_3: sub3,
            sub_stat_4: sub4,
            equip,
        })
    }
}

impl ArtifactSetName {
    #[rustfmt::skip]
    pub fn from_zh_cn(s: &str) -> Option<ArtifactSetName> {
        match s {
            "磐陀裂生之花" => Some(ArtifactSetName::ArchaicPetra),
            "嵯峨群峰之翼" => Some(ArtifactSetName::ArchaicPetra),
            "星罗圭壁之晷" => Some(ArtifactSetName::ArchaicPetra),
            // "壁" is different
            "星罗圭璧之晷" => Some(ArtifactSetName::ArchaicPetra),
            "巉岩琢塑之樽" => Some(ArtifactSetName::ArchaicPetra),
            "不动玄石之相" => Some(ArtifactSetName::ArchaicPetra),
            "历经风雪的思念" => Some(ArtifactSetName::BlizzardStrayer),
            "摧冰而行的执望" => Some(ArtifactSetName::BlizzardStrayer),
            "冰雪故园的终期" => Some(ArtifactSetName::BlizzardStrayer),
            "遍结寒霜的傲骨" => Some(ArtifactSetName::BlizzardStrayer),
            "破冰踏雪的回音" => Some(ArtifactSetName::BlizzardStrayer),
            "染血的铁之心" => Some(ArtifactSetName::BloodstainedChivalry),
            "染血的黑之羽" => Some(ArtifactSetName::BloodstainedChivalry),
            "骑士染血之时" => Some(ArtifactSetName::BloodstainedChivalry),
            "染血骑士之杯" => Some(ArtifactSetName::BloodstainedChivalry),
            "染血的铁假面" => Some(ArtifactSetName::BloodstainedChivalry),
            "魔女的炎之花" => Some(ArtifactSetName::CrimsonWitch),
            "魔女常燃之羽" => Some(ArtifactSetName::CrimsonWitch),
            "魔女破灭之时" => Some(ArtifactSetName::CrimsonWitch),
            "魔女的心之火" => Some(ArtifactSetName::CrimsonWitch),
            "焦灼的魔女帽" => Some(ArtifactSetName::CrimsonWitch),
            "角斗士的留恋" => Some(ArtifactSetName::GladiatorFinale),
            "角斗士的归宿" => Some(ArtifactSetName::GladiatorFinale),
            "角斗士的希冀" => Some(ArtifactSetName::GladiatorFinale),
            "角斗士的酣醉" => Some(ArtifactSetName::GladiatorFinale),
            "角斗士的凯旋" => Some(ArtifactSetName::GladiatorFinale),
            "饰金胸花" => Some(ArtifactSetName::HeartOfDepth),
            "追忆之风" => Some(ArtifactSetName::HeartOfDepth),
            "坚铜罗盘" => Some(ArtifactSetName::HeartOfDepth),
            "沉波之盏" => Some(ArtifactSetName::HeartOfDepth),
            "酒渍船帽" => Some(ArtifactSetName::HeartOfDepth),
            "渡火者的决绝" => Some(ArtifactSetName::LavaWalker),
            "渡火者的解脱" => Some(ArtifactSetName::LavaWalker),
            "渡火者的煎熬" => Some(ArtifactSetName::LavaWalker),
            "渡火者的醒悟" => Some(ArtifactSetName::LavaWalker),
            "渡火者的智慧" => Some(ArtifactSetName::LavaWalker),
            "远方的少女之心" => Some(ArtifactSetName::MaidenBeloved),
            "少女飘摇的思念" => Some(ArtifactSetName::MaidenBeloved),
            "少女苦短的良辰" => Some(ArtifactSetName::MaidenBeloved),
            "少女片刻的闲暇" => Some(ArtifactSetName::MaidenBeloved),
            "少女易逝的芳颜" => Some(ArtifactSetName::MaidenBeloved),
            "宗室之花" => Some(ArtifactSetName::NoblesseOblige),
            "宗室之翎" => Some(ArtifactSetName::NoblesseOblige),
            "宗室时计" => Some(ArtifactSetName::NoblesseOblige),
            "宗室银瓮" => Some(ArtifactSetName::NoblesseOblige),
            "宗室面具" => Some(ArtifactSetName::NoblesseOblige),
            "夏祭之花" => Some(ArtifactSetName::RetracingBolide),
            "夏祭终末" => Some(ArtifactSetName::RetracingBolide),
            "夏祭之刻" => Some(ArtifactSetName::RetracingBolide),
            "夏祭水玉" => Some(ArtifactSetName::RetracingBolide),
            "夏祭之面" => Some(ArtifactSetName::RetracingBolide),
            "平雷之心" => Some(ArtifactSetName::ThunderSmoother),
            "平雷之羽" => Some(ArtifactSetName::ThunderSmoother),
            "平雷之刻" => Some(ArtifactSetName::ThunderSmoother),
            "平雷之器" => Some(ArtifactSetName::ThunderSmoother),
            "平雷之冠" => Some(ArtifactSetName::ThunderSmoother),
            "雷鸟的怜悯" => Some(ArtifactSetName::ThunderingFury),
            "雷灾的孑遗" => Some(ArtifactSetName::ThunderingFury),
            "雷霆的时计" => Some(ArtifactSetName::ThunderingFury),
            "降雷的凶兆" => Some(ArtifactSetName::ThunderingFury),
            "唤雷的头冠" => Some(ArtifactSetName::ThunderingFury),
            "野花记忆的绿野" => Some(ArtifactSetName::ViridescentVenerer),
            "猎人青翠的箭羽" => Some(ArtifactSetName::ViridescentVenerer),
            "翠绿猎人的笃定" => Some(ArtifactSetName::ViridescentVenerer),
            "翠绿猎人的容器" => Some(ArtifactSetName::ViridescentVenerer),
            "翠绿的猎人之冠" => Some(ArtifactSetName::ViridescentVenerer),
            "乐团的晨光" => Some(ArtifactSetName::WandererTroupe),
            "琴师的箭羽" => Some(ArtifactSetName::WandererTroupe),
            "终幕的时计" => Some(ArtifactSetName::WandererTroupe),
            "终末的时计" => Some(ArtifactSetName::WandererTroupe),
            "吟游者之壶" => Some(ArtifactSetName::WandererTroupe),
            "指挥的礼帽" => Some(ArtifactSetName::WandererTroupe),
            "战狂的蔷薇" => Some(ArtifactSetName::Berserker),
            "战狂的翎羽" => Some(ArtifactSetName::Berserker),
            "战狂的时计" => Some(ArtifactSetName::Berserker),
            "战狂的骨杯" => Some(ArtifactSetName::Berserker),
            "战狂的鬼面" => Some(ArtifactSetName::Berserker),
            "勇士的勋章" => Some(ArtifactSetName::BraveHeart),
            "勇士的期许" => Some(ArtifactSetName::BraveHeart),
            "勇士的坚毅" => Some(ArtifactSetName::BraveHeart),
            "勇士的壮行" => Some(ArtifactSetName::BraveHeart),
            "勇士的冠冕" => Some(ArtifactSetName::BraveHeart),
            "守护之花" => Some(ArtifactSetName::DefenderWill),
            "守护徽印" => Some(ArtifactSetName::DefenderWill),
            "守护座钟" => Some(ArtifactSetName::DefenderWill),
            "守护之皿" => Some(ArtifactSetName::DefenderWill),
            "守护束带" => Some(ArtifactSetName::DefenderWill),
            "流放者之花" => Some(ArtifactSetName::Exile),
            "流放者之羽" => Some(ArtifactSetName::Exile),
            "流放者怀表" => Some(ArtifactSetName::Exile),
            "流放者之杯" => Some(ArtifactSetName::Exile),
            "流放者头冠" => Some(ArtifactSetName::Exile),
            "赌徒的胸花" => Some(ArtifactSetName::Gambler),
            "赌徒的羽饰" => Some(ArtifactSetName::Gambler),
            "赌徒的怀表" => Some(ArtifactSetName::Gambler),
            "赌徒的骰盅" => Some(ArtifactSetName::Gambler),
            "赌徒的耳环" => Some(ArtifactSetName::Gambler),
            "教官的胸花" => Some(ArtifactSetName::Instructor),
            "教官的羽饰" => Some(ArtifactSetName::Instructor),
            "教官的怀表" => Some(ArtifactSetName::Instructor),
            "教官的茶杯" => Some(ArtifactSetName::Instructor),
            "教官的帽子" => Some(ArtifactSetName::Instructor),
            "武人的红花" => Some(ArtifactSetName::MartialArtist),
            "武人的羽饰" => Some(ArtifactSetName::MartialArtist),
            "武人的水漏" => Some(ArtifactSetName::MartialArtist),
            "武人的酒杯" => Some(ArtifactSetName::MartialArtist),
            "武人的头巾" => Some(ArtifactSetName::MartialArtist),
            "祭水礼冠" => Some(ArtifactSetName::PrayersForDestiny),
            "祭火礼冠" => Some(ArtifactSetName::PrayersForIllumination),
            "祭雷礼冠" => Some(ArtifactSetName::PrayersForWisdom),
            "祭冰礼冠" => Some(ArtifactSetName::PrayersToSpringtime),
            "故人之心" => Some(ArtifactSetName::ResolutionOfSojourner),
            "归乡之羽" => Some(ArtifactSetName::ResolutionOfSojourner),
            "逐光之石" => Some(ArtifactSetName::ResolutionOfSojourner),
            "异国之盏" => Some(ArtifactSetName::ResolutionOfSojourner),
            "感别之冠" => Some(ArtifactSetName::ResolutionOfSojourner),
            "学士的书签" => Some(ArtifactSetName::Scholar),
            "学士的羽笔" => Some(ArtifactSetName::Scholar),
            "学士的时钟" => Some(ArtifactSetName::Scholar),
            "学士的墨杯" => Some(ArtifactSetName::Scholar),
            "学士的镜片" => Some(ArtifactSetName::Scholar),
            "奇迹之花" => Some(ArtifactSetName::TinyMiracle),
            "奇迹之羽" => Some(ArtifactSetName::TinyMiracle),
            "奇迹之沙" => Some(ArtifactSetName::TinyMiracle),
            "奇迹之杯" => Some(ArtifactSetName::TinyMiracle),
            "奇迹耳坠" => Some(ArtifactSetName::TinyMiracle),
            "冒险家之花" => Some(ArtifactSetName::Adventurer),
            "冒险家尾羽" => Some(ArtifactSetName::Adventurer),
            "冒险家怀表" => Some(ArtifactSetName::Adventurer),
            "冒险家金杯" => Some(ArtifactSetName::Adventurer),
            "冒险家头带" => Some(ArtifactSetName::Adventurer),
            "幸运儿绿花" => Some(ArtifactSetName::LuckyDog),
            "幸运儿鹰羽" => Some(ArtifactSetName::LuckyDog),
            "幸运儿沙漏" => Some(ArtifactSetName::LuckyDog),
            "幸运儿之杯" => Some(ArtifactSetName::LuckyDog),
            "幸运儿银冠" => Some(ArtifactSetName::LuckyDog),
            "游医的银莲" => Some(ArtifactSetName::TravelingDoctor),
            "游医的枭羽" => Some(ArtifactSetName::TravelingDoctor),
            "游医的怀钟" => Some(ArtifactSetName::TravelingDoctor),
            "游医的药壶" => Some(ArtifactSetName::TravelingDoctor),
            "游医的方巾" => Some(ArtifactSetName::TravelingDoctor),
            "勋绩之花" => Some(ArtifactSetName::TenacityOfTheMillelith),
            "昭武翎羽" => Some(ArtifactSetName::TenacityOfTheMillelith),
            "金铜时晷" => Some(ArtifactSetName::TenacityOfTheMillelith),
            "盟誓金爵" => Some(ArtifactSetName::TenacityOfTheMillelith),
            "将帅兜鍪" => Some(ArtifactSetName::TenacityOfTheMillelith),
            "无垢之花" => Some(ArtifactSetName::PaleFlame),
            "贤医之羽" => Some(ArtifactSetName::PaleFlame),
            "停摆之刻" => Some(ArtifactSetName::PaleFlame),
            "超越之盏" => Some(ArtifactSetName::PaleFlame),
            "嗤笑之面" => Some(ArtifactSetName::PaleFlame),
            "明威之镡" => Some(ArtifactSetName::EmblemOfSeveredFate),
            "切落之羽" => Some(ArtifactSetName::EmblemOfSeveredFate),
            "雷云之笼" => Some(ArtifactSetName::EmblemOfSeveredFate),
            "绯花之壶" => Some(ArtifactSetName::EmblemOfSeveredFate),
            "华饰之兜" => Some(ArtifactSetName::EmblemOfSeveredFate),
            "羁缠之花" => Some(ArtifactSetName::ShimenawaReminiscence),
            "思忆之矢" => Some(ArtifactSetName::ShimenawaReminiscence),
            "朝露之时" => Some(ArtifactSetName::ShimenawaReminiscence),
            "祈望之心" => Some(ArtifactSetName::ShimenawaReminiscence),
            "无常之面" => Some(ArtifactSetName::ShimenawaReminiscence),
            "荣花之期" => Some(ArtifactSetName::HuskOfOpulentDreams),
            "华馆之羽" => Some(ArtifactSetName::HuskOfOpulentDreams),
            "众生之谣" => Some(ArtifactSetName::HuskOfOpulentDreams),
            "梦醒之瓢" => Some(ArtifactSetName::HuskOfOpulentDreams),
            "形骸之笠" => Some(ArtifactSetName::HuskOfOpulentDreams),
            "海染之花" => Some(ArtifactSetName::OceanHuedClam),
            "渊宫之羽" => Some(ArtifactSetName::OceanHuedClam),
            "离别之贝" => Some(ArtifactSetName::OceanHuedClam),
            "真珠之笼" => Some(ArtifactSetName::OceanHuedClam),
            "海祇之冠" => Some(ArtifactSetName::OceanHuedClam),
            "生灵之华" | "阳辔之遗" | "潜光片羽" | "结契之刻" | "虺雷之姿" => Some(ArtifactSetName::VermillionHereafter),
            "魂香之花" | "祝祀之凭" | "垂玉之叶" | "涌泉之盏" | "浮溯之珏" => Some(ArtifactSetName::EchoesOfAnOffering),
            "迷宫的游人" | "翠蔓的智者" | "贤智的定期" | "迷误者之灯" | "月桂的宝冠" => Some(ArtifactSetName::DeepwoodMemories),
            "梦中的铁花" | "裁断的翎羽" | "沉金的岁月" | "如蜜的终宴" | "沙王的投影" => Some(ArtifactSetName::GildedDreams),
            "月女的华彩" | "谢落的筵席" | "凝结的时刻" | "守秘的魔瓶" | "紫晶的花冠" => Some(ArtifactSetName::FlowerOfParadiseLost),
            "众王之都的开端" | "黄金邦国的结末" | "失落迷途的机芯" | "迷醉长梦的守护" | "流沙贵嗣的遗宝" => Some(ArtifactSetName::DesertPavilionChronicle),
            "恶龙的单片镜" | "坏巫师的羽杖" | "旅途中的鲜花" | "水仙的时时刻刻" | "勇者们的茶会" => Some(ArtifactSetName::NymphsDream),
            "灵光明烁之心" | "琦色灵彩之羽" | "灵光源起之蕊" | "久远花落之时" | "无边酣乐之筵" => Some(ArtifactSetName::VourukashasGlow),
            "猎人的胸花" | "杰作的序曲" | "裁判的时刻" | "遗忘的容器" | "老兵的容颜" => Some(ArtifactSetName::MarechausseeHunter),
            "黄金乐曲的变奏" | "黄金飞鸟的落羽" | "黄金时代的先声" | "黄金之夜的喧嚣" | "黄金剧团的奖赏" => Some(ArtifactSetName::GoldenTroupe),
            "昔时传奏之诗" | "昔时浮想之思" | "昔时遗落之誓" | "昔时回映之音" | "昔时应许之梦" => Some(ArtifactSetName::SongOfDaysPast),
            "慈爱的淑女帽" | "诚恳的蘸水笔" | "无私的妆饰花" | "忠实的砂时计" | "慷慨的墨水瓶" => Some(ArtifactSetName::NighttimeWhispersInTheEchoingWoods),
            "异想零落的圆舞" | "古海玄幽的夜想" | "谐律交响的前奏" | "命途轮转的谐谑" | "灵露倾洒的狂诗" => Some(ArtifactSetName::FragmentOfHarmonicWhimsy),
            "失冕的宝冠" | "褪光的翠尾" | "暗结的明花" | "举业的识刻" | "筹谋的共樽" => Some(ArtifactSetName::UnfinishedReverie),
            _ => None,
        }
    }
}

impl ArtifactSlot {
    pub fn from_zh_cn(s: &str) -> Option<ArtifactSlot> {
        match s {
            "磐陀裂生之花" => Some(ArtifactSlot::Flower),
            "嵯峨群峰之翼" => Some(ArtifactSlot::Feather),
            "星罗圭壁之晷" => Some(ArtifactSlot::Sand),
            "星罗圭璧之晷" => Some(ArtifactSlot::Sand),
            "巉岩琢塑之樽" => Some(ArtifactSlot::Goblet),
            "不动玄石之相" => Some(ArtifactSlot::Head),
            "历经风雪的思念" => Some(ArtifactSlot::Flower),
            "摧冰而行的执望" => Some(ArtifactSlot::Feather),
            "冰雪故园的终期" => Some(ArtifactSlot::Sand),
            "遍结寒霜的傲骨" => Some(ArtifactSlot::Goblet),
            "破冰踏雪的回音" => Some(ArtifactSlot::Head),
            "染血的铁之心" => Some(ArtifactSlot::Flower),
            "染血的黑之羽" => Some(ArtifactSlot::Feather),
            "骑士染血之时" => Some(ArtifactSlot::Sand),
            "染血骑士之杯" => Some(ArtifactSlot::Goblet),
            "染血的铁假面" => Some(ArtifactSlot::Head),
            "魔女的炎之花" => Some(ArtifactSlot::Flower),
            "魔女常燃之羽" => Some(ArtifactSlot::Feather),
            "魔女破灭之时" => Some(ArtifactSlot::Sand),
            "魔女的心之火" => Some(ArtifactSlot::Goblet),
            "焦灼的魔女帽" => Some(ArtifactSlot::Head),
            "角斗士的留恋" => Some(ArtifactSlot::Flower),
            "角斗士的归宿" => Some(ArtifactSlot::Feather),
            "角斗士的希冀" => Some(ArtifactSlot::Sand),
            "角斗士的酣醉" => Some(ArtifactSlot::Goblet),
            "角斗士的凯旋" => Some(ArtifactSlot::Head),
            "饰金胸花" => Some(ArtifactSlot::Flower),
            "追忆之风" => Some(ArtifactSlot::Feather),
            "坚铜罗盘" => Some(ArtifactSlot::Sand),
            "沉波之盏" => Some(ArtifactSlot::Goblet),
            "酒渍船帽" => Some(ArtifactSlot::Head),
            "渡火者的决绝" => Some(ArtifactSlot::Flower),
            "渡火者的解脱" => Some(ArtifactSlot::Feather),
            "渡火者的煎熬" => Some(ArtifactSlot::Sand),
            "渡火者的醒悟" => Some(ArtifactSlot::Goblet),
            "渡火者的智慧" => Some(ArtifactSlot::Head),
            "远方的少女之心" => Some(ArtifactSlot::Flower),
            "少女飘摇的思念" => Some(ArtifactSlot::Feather),
            "少女苦短的良辰" => Some(ArtifactSlot::Sand),
            "少女片刻的闲暇" => Some(ArtifactSlot::Goblet),
            "少女易逝的芳颜" => Some(ArtifactSlot::Head),
            "宗室之花" => Some(ArtifactSlot::Flower),
            "宗室之翎" => Some(ArtifactSlot::Feather),
            "宗室时计" => Some(ArtifactSlot::Sand),
            "宗室银瓮" => Some(ArtifactSlot::Goblet),
            "宗室面具" => Some(ArtifactSlot::Head),
            "夏祭之花" => Some(ArtifactSlot::Flower),
            "夏祭终末" => Some(ArtifactSlot::Feather),
            "夏祭之刻" => Some(ArtifactSlot::Sand),
            "夏祭水玉" => Some(ArtifactSlot::Goblet),
            "夏祭之面" => Some(ArtifactSlot::Head),
            "平雷之心" => Some(ArtifactSlot::Flower),
            "平雷之羽" => Some(ArtifactSlot::Feather),
            "平雷之刻" => Some(ArtifactSlot::Sand),
            "平雷之器" => Some(ArtifactSlot::Goblet),
            "平雷之冠" => Some(ArtifactSlot::Head),
            "雷鸟的怜悯" => Some(ArtifactSlot::Flower),
            "雷灾的孑遗" => Some(ArtifactSlot::Feather),
            "雷霆的时计" => Some(ArtifactSlot::Sand),
            "降雷的凶兆" => Some(ArtifactSlot::Goblet),
            "唤雷的头冠" => Some(ArtifactSlot::Head),
            "野花记忆的绿野" => Some(ArtifactSlot::Flower),
            "猎人青翠的箭羽" => Some(ArtifactSlot::Feather),
            "翠绿猎人的笃定" => Some(ArtifactSlot::Sand),
            "翠绿猎人的容器" => Some(ArtifactSlot::Goblet),
            "翠绿的猎人之冠" => Some(ArtifactSlot::Head),
            "乐团的晨光" => Some(ArtifactSlot::Flower),
            "琴师的箭羽" => Some(ArtifactSlot::Feather),
            "终幕的时计" => Some(ArtifactSlot::Sand),
            "终末的时计" => Some(ArtifactSlot::Sand),
            "吟游者之壶" => Some(ArtifactSlot::Goblet),
            "指挥的礼帽" => Some(ArtifactSlot::Head),
            "战狂的蔷薇" => Some(ArtifactSlot::Flower),
            "战狂的翎羽" => Some(ArtifactSlot::Feather),
            "战狂的时计" => Some(ArtifactSlot::Sand),
            "战狂的骨杯" => Some(ArtifactSlot::Goblet),
            "战狂的鬼面" => Some(ArtifactSlot::Head),
            "勇士的勋章" => Some(ArtifactSlot::Flower),
            "勇士的期许" => Some(ArtifactSlot::Feather),
            "勇士的坚毅" => Some(ArtifactSlot::Sand),
            "勇士的壮行" => Some(ArtifactSlot::Goblet),
            "勇士的冠冕" => Some(ArtifactSlot::Head),
            "守护之花" => Some(ArtifactSlot::Flower),
            "守护徽印" => Some(ArtifactSlot::Feather),
            "守护座钟" => Some(ArtifactSlot::Sand),
            "守护之皿" => Some(ArtifactSlot::Goblet),
            "守护束带" => Some(ArtifactSlot::Head),
            "流放者之花" => Some(ArtifactSlot::Flower),
            "流放者之羽" => Some(ArtifactSlot::Feather),
            "流放者怀表" => Some(ArtifactSlot::Sand),
            "流放者之杯" => Some(ArtifactSlot::Goblet),
            "流放者头冠" => Some(ArtifactSlot::Head),
            "赌徒的胸花" => Some(ArtifactSlot::Flower),
            "赌徒的羽饰" => Some(ArtifactSlot::Feather),
            "赌徒的怀表" => Some(ArtifactSlot::Sand),
            "赌徒的骰盅" => Some(ArtifactSlot::Goblet),
            "赌徒的耳环" => Some(ArtifactSlot::Head),
            "教官的胸花" => Some(ArtifactSlot::Flower),
            "教官的羽饰" => Some(ArtifactSlot::Feather),
            "教官的怀表" => Some(ArtifactSlot::Sand),
            "教官的茶杯" => Some(ArtifactSlot::Goblet),
            "教官的帽子" => Some(ArtifactSlot::Head),
            "武人的红花" => Some(ArtifactSlot::Flower),
            "武人的羽饰" => Some(ArtifactSlot::Feather),
            "武人的水漏" => Some(ArtifactSlot::Sand),
            "武人的酒杯" => Some(ArtifactSlot::Goblet),
            "武人的头巾" => Some(ArtifactSlot::Head),
            "祭水礼冠" => Some(ArtifactSlot::Head),
            "祭火礼冠" => Some(ArtifactSlot::Head),
            "祭雷礼冠" => Some(ArtifactSlot::Head),
            "祭冰礼冠" => Some(ArtifactSlot::Head),
            "故人之心" => Some(ArtifactSlot::Flower),
            "归乡之羽" => Some(ArtifactSlot::Feather),
            "逐光之石" => Some(ArtifactSlot::Sand),
            "异国之盏" => Some(ArtifactSlot::Goblet),
            "感别之冠" => Some(ArtifactSlot::Head),
            "学士的书签" => Some(ArtifactSlot::Flower),
            "学士的羽笔" => Some(ArtifactSlot::Feather),
            "学士的时钟" => Some(ArtifactSlot::Sand),
            "学士的墨杯" => Some(ArtifactSlot::Goblet),
            "学士的镜片" => Some(ArtifactSlot::Head),
            "奇迹之花" => Some(ArtifactSlot::Flower),
            "奇迹之羽" => Some(ArtifactSlot::Feather),
            "奇迹之沙" => Some(ArtifactSlot::Sand),
            "奇迹之杯" => Some(ArtifactSlot::Goblet),
            "奇迹耳坠" => Some(ArtifactSlot::Head),
            "冒险家之花" => Some(ArtifactSlot::Flower),
            "冒险家尾羽" => Some(ArtifactSlot::Feather),
            "冒险家怀表" => Some(ArtifactSlot::Sand),
            "冒险家金杯" => Some(ArtifactSlot::Goblet),
            "冒险家头带" => Some(ArtifactSlot::Head),
            "幸运儿绿花" => Some(ArtifactSlot::Flower),
            "幸运儿鹰羽" => Some(ArtifactSlot::Feather),
            "幸运儿沙漏" => Some(ArtifactSlot::Sand),
            "幸运儿之杯" => Some(ArtifactSlot::Goblet),
            "幸运儿银冠" => Some(ArtifactSlot::Head),
            "游医的银莲" => Some(ArtifactSlot::Flower),
            "游医的枭羽" => Some(ArtifactSlot::Feather),
            "游医的怀钟" => Some(ArtifactSlot::Sand),
            "游医的药壶" => Some(ArtifactSlot::Goblet),
            "游医的方巾" => Some(ArtifactSlot::Head),
            "勋绩之花" => Some(ArtifactSlot::Flower),
            "昭武翎羽" => Some(ArtifactSlot::Feather),
            "金铜时晷" => Some(ArtifactSlot::Sand),
            "盟誓金爵" => Some(ArtifactSlot::Goblet),
            "将帅兜鍪" => Some(ArtifactSlot::Head),
            "无垢之花" => Some(ArtifactSlot::Flower),
            "贤医之羽" => Some(ArtifactSlot::Feather),
            "停摆之刻" => Some(ArtifactSlot::Sand),
            "超越之盏" => Some(ArtifactSlot::Goblet),
            "嗤笑之面" => Some(ArtifactSlot::Head),
            "明威之镡" => Some(ArtifactSlot::Flower),
            "切落之羽" => Some(ArtifactSlot::Feather),
            "雷云之笼" => Some(ArtifactSlot::Sand),
            "绯花之壶" => Some(ArtifactSlot::Goblet),
            "华饰之兜" => Some(ArtifactSlot::Head),
            "羁缠之花" => Some(ArtifactSlot::Flower),
            "思忆之矢" => Some(ArtifactSlot::Feather),
            "朝露之时" => Some(ArtifactSlot::Sand),
            "祈望之心" => Some(ArtifactSlot::Goblet),
            "无常之面" => Some(ArtifactSlot::Head),
            "荣花之期" => Some(ArtifactSlot::Flower),
            "华馆之羽" => Some(ArtifactSlot::Feather),
            "众生之谣" => Some(ArtifactSlot::Sand),
            "梦醒之瓢" => Some(ArtifactSlot::Goblet),
            "形骸之笠" => Some(ArtifactSlot::Head),
            "海染之花" => Some(ArtifactSlot::Flower),
            "渊宫之羽" => Some(ArtifactSlot::Feather),
            "离别之贝" => Some(ArtifactSlot::Sand),
            "真珠之笼" => Some(ArtifactSlot::Goblet),
            "海祇之冠" => Some(ArtifactSlot::Head),
            "生灵之华" => Some(ArtifactSlot::Flower),
            "阳辔之遗" => Some(ArtifactSlot::Sand),
            "潜光片羽" => Some(ArtifactSlot::Feather),
            "结契之刻" => Some(ArtifactSlot::Goblet),
            "虺雷之姿" => Some(ArtifactSlot::Head),
            "魂香之花" => Some(ArtifactSlot::Flower),
            "祝祀之凭" => Some(ArtifactSlot::Sand),
            "垂玉之叶" => Some(ArtifactSlot::Feather),
            "涌泉之盏" => Some(ArtifactSlot::Goblet),
            "浮溯之珏" => Some(ArtifactSlot::Head),
            "迷宫的游人" => Some(ArtifactSlot::Flower),
            "翠蔓的智者" => Some(ArtifactSlot::Feather),
            "贤智的定期" => Some(ArtifactSlot::Sand),
            "迷误者之灯" => Some(ArtifactSlot::Goblet),
            "月桂的宝冠" => Some(ArtifactSlot::Head),
            "梦中的铁花" => Some(ArtifactSlot::Flower),
            "裁断的翎羽" => Some(ArtifactSlot::Feather),
            "沉金的岁月" => Some(ArtifactSlot::Sand),
            "如蜜的终宴" => Some(ArtifactSlot::Goblet),
            "沙王的投影" => Some(ArtifactSlot::Head),
            "月女的华彩" => Some(ArtifactSlot::Flower),
            "谢落的筵席" => Some(ArtifactSlot::Feather),
            "凝结的时刻" => Some(ArtifactSlot::Sand),
            "守秘的魔瓶" => Some(ArtifactSlot::Goblet),
            "紫晶的花冠" => Some(ArtifactSlot::Head),
            "众王之都的开端" => Some(ArtifactSlot::Flower),
            "黄金邦国的结末" => Some(ArtifactSlot::Feather),
            "失落迷途的机芯" => Some(ArtifactSlot::Sand),
            "迷醉长梦的守护" => Some(ArtifactSlot::Goblet),
            "流沙贵嗣的遗宝" => Some(ArtifactSlot::Head),
            "旅途中的鲜花" => Some(ArtifactSlot::Flower),
            "坏巫师的羽杖" => Some(ArtifactSlot::Feather),
            "水仙的时时刻刻" => Some(ArtifactSlot::Sand),
            "勇者们的茶会" => Some(ArtifactSlot::Goblet),
            "恶龙的单片镜" => Some(ArtifactSlot::Head),
            "灵光源起之蕊" => Some(ArtifactSlot::Flower),
            "琦色灵彩之羽" => Some(ArtifactSlot::Feather),
            "久远花落之时" => Some(ArtifactSlot::Sand),
            "无边酣乐之筵" => Some(ArtifactSlot::Goblet),
            "灵光明烁之心" => Some(ArtifactSlot::Head),
            "猎人的胸花" => Some(ArtifactSlot::Flower),
            "杰作的序曲" => Some(ArtifactSlot::Feather),
            "裁判的时刻" => Some(ArtifactSlot::Sand),
            "遗忘的容器" => Some(ArtifactSlot::Goblet),
            "老兵的容颜" => Some(ArtifactSlot::Head),
            "黄金乐曲的变奏" => Some(ArtifactSlot::Flower),
            "黄金飞鸟的落羽" => Some(ArtifactSlot::Feather),
            "黄金时代的先声" => Some(ArtifactSlot::Sand),
            "黄金之夜的喧嚣" => Some(ArtifactSlot::Goblet),
            "黄金剧团的奖赏" => Some(ArtifactSlot::Head),
            "昔时遗落之誓" => Some(ArtifactSlot::Flower),
            "昔时浮想之思" => Some(ArtifactSlot::Feather),
            "昔时回映之音" => Some(ArtifactSlot::Sand),
            "昔时应许之梦" => Some(ArtifactSlot::Goblet),
            "昔时传奏之诗" => Some(ArtifactSlot::Head),
            "无私的妆饰花" => Some(ArtifactSlot::Flower),
            "诚恳的蘸水笔" => Some(ArtifactSlot::Feather),
            "忠实的砂时计" => Some(ArtifactSlot::Sand),
            "慷慨的墨水瓶" => Some(ArtifactSlot::Goblet),
            "慈爱的淑女帽" => Some(ArtifactSlot::Head),
            "异想零落的圆舞" => Some(ArtifactSlot::Head),
            "古海玄幽的夜想" => Some(ArtifactSlot::Feather),
            "谐律交响的前奏" => Some(ArtifactSlot::Flower),
            "命途轮转的谐谑" => Some(ArtifactSlot::Sand),
            "灵露倾洒的狂诗" => Some(ArtifactSlot::Goblet),
            "失冕的宝冠" => Some(ArtifactSlot::Head),
            "褪光的翠尾" => Some(ArtifactSlot::Feather),
            "暗结的明花" => Some(ArtifactSlot::Flower),
            "举业的识刻" => Some(ArtifactSlot::Sand),
            "筹谋的共樽" => Some(ArtifactSlot::Goblet),
            _ => None,
        }
    }
}