pub const SEARCH_SEASON: &str = r#"query ($search: String, $isAdult: Boolean) {
  anime: Page(perPage: 8) {
    pageInfo {
      total
    }
    results: media(type: ANIME, search: $search, isAdult: $isAdult) {
      id
      title {
        romaji(stylised:true)
        english(stylised:true)
        native(stylised:true)
      }
      synonyms
      coverImage {
        extraLarge,
        color
      }
      type
      format
      description
      bannerImage
      isLicensed
      season
      seasonYear
      duration
      episodes
      startDate {
        year
      }
    }
  }
}
  "#;

  pub const GET_SEASON: &str = r#"query ($id: Int) {
    anime: Page(perPage: 8) {
      pageInfo {
        total
      }
      results: media(type: ANIME, id: $id) {
        id
        title {
          romaji(stylised:true)
          english(stylised:true)
          native(stylised:true)
        }
        synonyms
        coverImage {
          extraLarge,
          color
        }
        type
        format
        description
        bannerImage
        isLicensed
        season
        seasonYear
        duration
        episodes
        startDate {
          year
        }
      }
    }
  }
  
  "#;