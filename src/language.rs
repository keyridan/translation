use std::fmt::{Display, Formatter, Result as SResult};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Language {
    AUTO,
    AF,
    SQ,
    AM,
    AR,
    HY,
    AZ,
    EU,
    BE,
    BN,
    BS,
    BG,
    CA,
    CEB,
    NY,
    ZHCN,
    ZHTW,
    CO,
    HR,
    CS,
    DA,
    NL,
    EN,
    EO,
    ET,
    TL,
    FI,
    FR,
    FY,
    GL,
    KA,
    DE,
    EL,
    GU,
    HT,
    HA,
    HAW,
    IW,
    HI,
    HMN,
    HU,
    IS,
    IG,
    ID,
    GA,
    IT,
    JA,
    JW,
    KN,
    KK,
    KM,
    KO,
    KU,
    KY,
    LO,
    LA,
    LV,
    LT,
    LB,
    MK,
    MG,
    MS,
    ML,
    MT,
    MI,
    MR,
    MN,
    MY,
    NE,
    NO,
    PS,
    FA,
    PL,
    PT,
    MA,
    RO,
    RU,
    SM,
    GD,
    SR,
    ST,
    SN,
    SD,
    SI,
    SK,
    SL,
    SO,
    ES,
    SU,
    SW,
    SV,
    TG,
    TA,
    TE,
    TH,
    TR,
    UK,
    UR,
    UZ,
    VI,
    CY,
    XH,
    YI,
    YO,
    ZU,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter) -> SResult {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Language {
    fn from_str(s: &str) -> Result<Language, ()> {
        match s {
            "auto" => Ok(Language::AUTO),
            "af" => Ok(Language::AF),
            "sq" => Ok(Language::SQ),
            "am" => Ok(Language::AM),
            "ar" => Ok(Language::AR),
            "hy" => Ok(Language::HY),
            "az" => Ok(Language::AZ),
            "eu" => Ok(Language::EU),
            "be" => Ok(Language::BE),
            "bn" => Ok(Language::BN),
            "bs" => Ok(Language::BS),
            "bg" => Ok(Language::BG),
            "ca" => Ok(Language::CA),
            "ceb" => Ok(Language::CEB),
            "ny" => Ok(Language::NY),
            "zh-cn" => Ok(Language::ZHCN),
            "zh-tw" => Ok(Language::ZHTW),
            "co" => Ok(Language::CO),
            "hr" => Ok(Language::HR),
            "cs" => Ok(Language::CS),
            "da" => Ok(Language::DA),
            "nl" => Ok(Language::NL),
            "en" => Ok(Language::EN),
            "eo" => Ok(Language::EO),
            "et" => Ok(Language::ET),
            "tl" => Ok(Language::TL),
            "fi" => Ok(Language::FI),
            "fr" => Ok(Language::FR),
            "fy" => Ok(Language::FY),
            "gl" => Ok(Language::GL),
            "ka" => Ok(Language::KA),
            "de" => Ok(Language::DE),
            "el" => Ok(Language::EL),
            "gu" => Ok(Language::GU),
            "ht" => Ok(Language::HT),
            "ha" => Ok(Language::HA),
            "haw" => Ok(Language::HAW),
            "iw" => Ok(Language::IW),
            "hi" => Ok(Language::HI),
            "hmn" => Ok(Language::HMN),
            "hu" => Ok(Language::HU),
            "is" => Ok(Language::IS),
            "ig" => Ok(Language::IG),
            "id" => Ok(Language::ID),
            "ga" => Ok(Language::GA),
            "it" => Ok(Language::IT),
            "ja" => Ok(Language::JA),
            "jw" => Ok(Language::JW),
            "kn" => Ok(Language::KN),
            "kk" => Ok(Language::KK),
            "km" => Ok(Language::KM),
            "ko" => Ok(Language::KO),
            "ku" => Ok(Language::KU),
            "ky" => Ok(Language::KY),
            "lo" => Ok(Language::LO),
            "la" => Ok(Language::LA),
            "lv" => Ok(Language::LV),
            "lt" => Ok(Language::LT),
            "lb" => Ok(Language::LB),
            "mk" => Ok(Language::MK),
            "mg" => Ok(Language::MG),
            "ms" => Ok(Language::MS),
            "ml" => Ok(Language::ML),
            "mt" => Ok(Language::MT),
            "mi" => Ok(Language::MI),
            "mr" => Ok(Language::MR),
            "mn" => Ok(Language::MN),
            "my" => Ok(Language::MY),
            "ne" => Ok(Language::NE),
            "no" => Ok(Language::NO),
            "ps" => Ok(Language::PS),
            "fa" => Ok(Language::FA),
            "pl" => Ok(Language::PL),
            "pt" => Ok(Language::PT),
            "ma" => Ok(Language::MA),
            "ro" => Ok(Language::RO),
            "ru" => Ok(Language::RU),
            "sm" => Ok(Language::SM),
            "gd" => Ok(Language::GD),
            "sr" => Ok(Language::SR),
            "st" => Ok(Language::ST),
            "sn" => Ok(Language::SN),
            "sd" => Ok(Language::SD),
            "si" => Ok(Language::SI),
            "sk" => Ok(Language::SK),
            "sl" => Ok(Language::SL),
            "so" => Ok(Language::SO),
            "es" => Ok(Language::ES),
            "su" => Ok(Language::SU),
            "sw" => Ok(Language::SW),
            "sv" => Ok(Language::SV),
            "tg" => Ok(Language::TG),
            "ta" => Ok(Language::TA),
            "te" => Ok(Language::TE),
            "th" => Ok(Language::TH),
            "tr" => Ok(Language::TR),
            "uk" => Ok(Language::UK),
            "ur" => Ok(Language::UR),
            "uz" => Ok(Language::UZ),
            "vi" => Ok(Language::VI),
            "cy" => Ok(Language::CY),
            "xh" => Ok(Language::XH),
            "yi" => Ok(Language::YI),
            "yo" => Ok(Language::YO),
            "zu" => Ok(Language::ZU),
            _ => Err(()),
        }
    }
    type Err = ();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_to_string() {
        // given
        let yo = Language::YO;

        // when
        let language = yo.to_string();

        // then
        assert_eq!(language, "YO");
    }

    #[test]
    fn from_str_when_str_language() {
        let value = Language::from_str("yo").unwrap();

        assert_eq!(value, Language::YO);
    }

    #[test]
    fn from_str_when_str_not_language() {
        let value = Language::from_str("not_language");

        assert_eq!(value, Err(()));
    }
}