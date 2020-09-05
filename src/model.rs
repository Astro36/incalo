use chrono::prelude::*;
use chrono::Duration;
use serde::{Deserialize, Serialize};

/// An university department
#[derive(Debug, Deserialize, Serialize)]
pub enum Department {
    /// 공과대학 기계공학과
    MEG,

    /// 공과대학 항공우주공학과
    ASE,

    /// 공과대학 조선해양공학과
    NOE,

    /// 공과대학 산업경영공학과
    IEN,

    /// 공과대학 화학공학과
    CHE,

    /// 공과대학 생명공학과
    BTE,

    /// 공과대학 고분자공학과
    PSE,

    /// 공과대학 신소재공학과
    MSE,

    /// 공과대학 사회인프라공학과
    CIV,

    /// 공과대학 환경공학과
    ENV,

    /// 공과대학 공간정보학과
    GEO,

    /// 공과대학 건축공학과
    ARE,

    /// 공과대학 건축학과
    ARC,

    /// 공과대학 에너지자원공학과
    ENR,

    /// 공과대학 융합기술경영학부
    CEM,

    /// 공과대학 전기공학과
    EEE,

    /// 공과대학 전자공학과
    ECE,

    /// 공과대학 컴퓨터공학과
    CSE,

    /// 공과대학 정보통신공학과
    ICE,

    /// 자연과학대학 수학과
    MTH,

    /// 자연과학대학 통계학과
    STS,

    /// 자연과학대학 물리학과
    PHY,

    /// 자연과학대학 화학과
    CHM,

    /// 자연과학대학 생명과학과
    BIO,

    /// 자연과학대학 해양과학과
    OCN,

    /// 자연과학대학 식품영양학과
    FAN,

    /// 경영대학 경영학과
    BUS,

    /// 경영대학 글로벌금융학과
    GFB,

    /// 경영대학 아태물류학부
    APL,

    /// 경영대학 국제통상학과
    INT,

    /// 사범대학 국어교육학과
    EKR,

    /// 사범대학 영어교육과
    EEG,

    /// 사범대학 사회교육과
    SSE,

    /// 사범대학 교육학과
    EDC,

    /// 사범대학 체육교육과
    PHE,

    /// 사범대학 수학교육과
    EMT,

    /// 사회과학대학 행정학과
    PAD,

    /// 사회과학대학 정치외교학과
    POL,

    /// 사회과학대학 미디어커뮤니케이션학과(언론정보학과)
    COM,

    /// 사회과학대학 경제학과
    ECO,

    /// 사회과학대학 소비자학과
    CON,

    /// 사회과학대학 아동심리학과
    CHS,

    /// 사회과학대학 사회복지학과
    SWE,

    /// 문과대학 한국어문학과
    HKO,

    /// 문과대학 사학과
    HIS,

    /// 문과대학 철학과
    PHI,

    /// 문과대학 중국학과
    CHN,

    /// 문과대학 일본언어문화학과
    JPN,

    /// 문과대학 영어영문학과
    ENG,

    /// 문과대학 프랑스언어문화학과
    FLL,

    /// 문과대학 문화콘텐츠문화경영학과
    HUM,

    /// 의과대학 의예과
    PMD,

    /// 의과대학 간호학과
    NUR,

    /// 미래융합대학 메카트로닉스공학과
    MCT,

    /// 미래융합대학 소프트웨어융합공학과
    ITC,

    /// 미래융합대학 산업경영학과
    SIM,

    /// 미래융합대학 금융세무재테크학과(금융투자학과)
    BNF,

    /// 예술체육학부 조형예술학과
    FAT,

    /// 예술체육학부 디자인융합학과
    CDN,

    /// 예술체육학부 스포츠과학과
    KIN,

    /// 예술체육학부 연극영화학과
    IPS,

    /// 예술체육학부 의류디자인학과
    FDT,

    /// 국제학부
    IGS,

    /// Unknown; An user does not reveal the department.
    UNK,
}

/// An user gender
#[derive(Debug, Deserialize, Serialize)]
pub enum Gender {
    /// Male
    MALE,

    /// Female
    FEMALE,

    /// Other gender
    OTHER,

    /// Hidden; An user does not reveal the gender.
    HIDDEN,
}

/// An user profile
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    /// An user ID
    pub id: String,

    /// An user email
    pub email: String,

    /// A korean user name
    pub name: String,

    /// An user gender
    pub gender: Gender,

    /// A Department to which the user belongs
    pub department: Department,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IdTokenPayload {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub name: String,
    pub gender: Gender,
    pub department: Department,
}

impl IdTokenPayload {
    pub fn new(issuer: String, client_id: String, expires_in: usize, user: User) -> Self {
        let issued_at = Utc::now();
        let expiration_time = issued_at + Duration::seconds(expires_in as i64);

        Self {
            iss: issuer,
            sub: user.id,
            aud: client_id,
            exp: expiration_time.timestamp() as usize,
            iat: issued_at.timestamp() as usize,
            email: user.email,
            name: user.name,
            gender: user.gender,
            department: user.department,
        }
    }
}
