use std::str::FromStr;
use blob::Blob;
use serde::{Deserialize, Serialize};

/// Hitomi and pixiv image proxying
pub fn proxy(shuffled_image_url: &str, token: String) -> Blob {
    let res = reqeust(format!("proxy/{}", shuffled_image_url), token);
    let blob: Blob = Blob::from_str(&res).unwrap();
    return blob;
}

/// Get Hitomi galleryinfo
pub fn galleryinfo(num: usize, token: String) -> Option<GalleryInfo> {
    let res = reqeust(format!("galleryinfo/{}", num), token);
    let json: Result<GalleryInfo, serde_json::Error> = serde_json::from_str(&res);

    if json.is_ok() {
        Some(json.unwrap())
    } else {
        None
    }
}

/// Get Hitomi image info
pub fn images(num: usize, token: String) -> Option<Images> {
    let res = reqeust(format!("images/{}", num), token);
    let json: Result<Images, serde_json::Error> = serde_json::from_str(&res);

    if json.is_ok() {
        Some(json.unwrap())
    } else {
        None
    }
}

/// Get latest Hitomi index list of Korean
pub fn index(token: String) -> Option<Vec<usize>> {
    let res = reqeust("index".to_string(), token);
    let json: Result<Vec<usize>, serde_json::Error> = serde_json::from_str(&res);

    if json.is_ok() {
        Some(json.unwrap())
    } else {
        None
    }
}

/// Get Hitomi info
pub fn info(num: usize, token: String) -> Option<Info> {
    let res = reqeust(format!("info/{}", num), token);
    let json: Result<Info, serde_json::Error> = serde_json::from_str(&res);

    if json.is_ok() {
        Some(json.unwrap())
    } else {
        None
    }
}

/// Get Hitomi info and galleryinfo
pub fn integrated(num: usize, token: String) -> Option<Integrated> {
    let res = reqeust(format!("integrated/{}", num), token);
    let json: Result<Integrated, serde_json::Error> = serde_json::from_str(&res);

    if json.is_ok() {
        Some(json.unwrap())
    } else {
        None
    }
}

/// Get latest Hitomi info list of Korean
pub fn list(num: usize, token: String) -> Option<List> {
    let res = reqeust(format!("integrated/{}", num), token);
    let json: Result<List, serde_json::Error> = serde_json::from_str(&res);

    if json.is_ok() {
        Some(json.unwrap())
    } else {
        None
    }
}


fn reqeust(url: String, token: String) -> String {

    let url: String = format!("https://doujinshiman.ga/v3/api/hitomi/{}", url);
    let client = reqwest::blocking::Client::new();
    let res = client.get(url)
        .header(reqwest::header::USER_AGENT, format!("AkiaCode/heliotrope v0.1 (https://github.com/AkiaCode)"))
        .header(reqwest::header::AUTHORIZATION, format!("{}", token))
        .send().unwrap().text().unwrap();

    return res;
}

#[derive(Serialize, Deserialize)]
pub struct List {
    status: usize,
    list: Vec<InfoListOrIntegrated>
}
impl List {
    pub fn status(&self) -> usize {
        self.status
    }
    pub fn list(&self) -> Vec<InfoListOrIntegrated> {
        self.list.clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Images {
    status: usize,
    images: Vec<Urls>
}
impl Images {
    pub fn status(&self) -> usize {
        self.status
    }
}

#[derive(Serialize, Deserialize)]
pub struct Urls {
    url: String
}
impl Urls {
    pub fn url(&self) -> String {
        self.url.clone()
    }
}
#[derive(Serialize, Deserialize)]
pub struct Integrated {
    data: GalleryInfoAndTags
}
impl Integrated {
    pub fn data(&self) -> GalleryInfoAndTags {
        self.data.clone()
    } 
}
#[derive(Clone, Serialize, Deserialize)]
pub struct GalleryInfoAndTags {
    galleryinfo: Vec<GalleryInfoIntegrated>,
    tags: Vec<InfoListOrIntegrated>
}

impl GalleryInfoAndTags {
    pub fn galleryinfo(&self) -> Vec<GalleryInfoIntegrated> {
        self.galleryinfo.clone()
    }
    pub fn tags(&self) -> Vec<InfoListOrIntegrated> {
        self.tags.clone()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InfoListOrIntegrated {
    title: Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters>,
    galleryid: String,
    thumbnail: String,
    artist: Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters>,
    group: Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters>,
    r#type: TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters,
    language: TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters,
    series: Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters>,
    characters: Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters>,
    tags: Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters>
}
impl InfoListOrIntegrated {
    pub fn title(&self) -> Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters> {
        self.title.clone()
    }
    pub fn galleryid(&self) -> String {
        self.galleryid.clone()
    }
    pub fn thumbnail(&self) -> String {
        self.thumbnail.clone()
    }
    pub fn artist(&self) -> Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters> {
        self.artist.clone()
    }
    pub fn group(&self) -> Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters> {
        self.group.clone()
    }
    pub fn r#type(&self) -> TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters {
        self.r#type.clone()
    }
    pub fn language(&self) -> TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters {
        self.language.clone()
    }
    pub fn series(&self) -> Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters> {
        self.series.clone()
    }
    pub fn characters(&self) -> Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters> {
        self.characters.clone()
    }
    pub fn tags(&self) -> Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters> {
        self.tags.clone()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GalleryInfoIntegrated {
    language_localname: String,
    language: String,
    date: String,
    files: Vec<Files>,
    tags: Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters>,
    japanese_title: Option<String>,
    title: String,
    id: String,
    r#type: String
}
impl GalleryInfoIntegrated {
    pub fn language_localname(&self) -> String {
        self.language_localname.clone()
    }
    pub fn language(&self) -> String {
        self.language.clone()
    }
    pub fn date(&self) -> String {
        self.date.clone()
    }
    pub fn files(&self) -> Vec<Files> {
        self.files.clone()
    }
    pub fn tags(&self) -> Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters> {
        self.tags.clone()
    }
    pub fn japanese_title(&self) -> String {
        self.japanese_title.as_ref().unwrap().clone()
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn id(&self) -> String {
        self.id.clone()
    }
    pub fn r#type(&self) -> String {
        self.r#type.clone()
    }

}
#[derive(Serialize, Deserialize)]
pub struct Info {
    status: usize,
    title: TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters,
    galleryid: String,
    thumbnail: String,
    artist: Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters>,
    group: Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters>,
    r#type: TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters,
    language: TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters,
    series: Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters>,
    characters: Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters>,
    tags: Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters>
}
impl Info {
    pub fn status(&self) -> usize {
        self.status
    }
    pub fn title(&self) -> TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters {
        self.title.clone()
    }
    pub fn galleryid(&self) -> String {
        self.galleryid.clone()
    }
    pub fn thumbnail(&self) -> String {
        self.thumbnail.clone()
    }
    pub fn artist(&self) -> Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters> {
        self.artist.clone()
    }
    pub fn group(&self) -> Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters> {
        self.group.clone()
    }
    pub fn r#type(&self) -> TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters {
        self.r#type.clone()
    }
    pub fn language(&self) -> TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters {
        self.language.clone()
    }
    pub fn series(&self) -> Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters> {
        self.series.clone()
    }
    pub fn characters(&self) -> Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters> {
        self.characters.clone()
    }
    pub fn tags(&self) -> Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters> {
        self.tags.clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct GalleryInfo {
    status: usize,
    language_localname: String,
    language: String,
    date: String,
    files: Vec<Files>,
    tags: Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters>,
    japanese_title: Option<String>,
    title: String,
    id: String,
    r#type: String
}

impl GalleryInfo {
    pub fn status(&self) -> usize {
        self.status
    }
    pub fn language_localname(&self) -> String {
        self.language_localname.clone()
    }
    pub fn language(&self) -> String {
        self.language.clone()
    }
    pub fn date(&self) -> String {
        self.date.clone()
    }
    pub fn files(&self) -> Vec<Files> {
        self.files.clone()
    }
    pub fn tags(&self) -> Vec<TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters> {
        self.tags.clone()
    }
    pub fn japanese_title(&self) -> String {
        self.japanese_title.as_ref().unwrap().clone()
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn id(&self) -> String {
        self.id.clone()
    }
    pub fn r#type(&self) -> String {
        self.r#type.clone()
    }

}

#[derive(Clone, Serialize, Deserialize)]
pub struct TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters {
    value: String,
    url: String
}

impl TagsOrTitleOrArtistOrGroupOrTypeOrLanguageOrSeriesOrCharacters {
    pub fn value(&self) -> String {
        self.value.clone()
    }
    pub fn url(&self) -> String {
        self.url.clone()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Files {
    width: usize,
    hash: String,
    haswebp: usize,
    hasavifsmalltn: usize,
    name: String,
    height: usize,
    hasavif: usize
}

impl Files {

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn hash(&self) -> String {
        self.hash.clone()
    }
    pub fn haswebp(&self) -> bool {
        if self.haswebp == 0 {
            false
        } else {
            true
        }
    }
    pub fn hasavifsmalltn(&self) -> bool {
        if self.hasavifsmalltn == 0 {
            false
        } else {
            true
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn hasavif(&self) -> bool {
        if self.hasavif == 0 {
            false
        } else {
            true
        }
    }
}