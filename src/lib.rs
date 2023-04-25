mod models {
    use serde::{Deserialize, Serialize};
    use std::fmt::{self, Display, Formatter};
    use std::str::FromStr;
    use std::string::ToString;
    use strum_macros::EnumString;

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, EnumString)]
    #[strum(ascii_case_insensitive)]
    pub enum SideChain {
        Nonpolar,
        Polar,
        Acidic,
        Basic,
        Positive,
    }

    impl Display for SideChain {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{}", self.to_owned())
        }
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
    pub struct AminoAcid {
        name: String,
        short_name: String,
        abbreviation: String,
        side_chain: SideChain,
        molecular_weight: f64,
        codon: Vec<String>,
    }

    impl AminoAcid {
        #[must_use]
        pub fn new(
            name: &str,
            short_name: &str,
            abbreviation: &str,
            side_chain: &str,
            molecular_weight: f64,
            codon: &[&str],
        ) -> Self {
            Self {
                name: name.to_string(),
                short_name: short_name.to_string(),
                abbreviation: abbreviation.to_string(),
                side_chain: SideChain::from_str(side_chain).unwrap(),
                molecular_weight,
                codon: codon.iter().map(ToString::to_string).collect(),
            }
        }
        #[must_use]
        pub fn get_name(&self) -> String {
            self.name.clone()
        }
        #[must_use]
        pub fn get_short_name(&self) -> String {
            self.short_name.clone()
        }
        #[must_use]
        pub fn get_abbreviation(&self) -> String {
            self.abbreviation.clone()
        }
        #[must_use]
        pub fn get_side_chain(&self) -> SideChain {
            self.side_chain.clone()
        }
        #[must_use]
        pub const fn get_molecular_weight(&self) -> f64 {
            self.molecular_weight
        }
        #[must_use]
        pub fn get_codon(&self) -> Vec<String> {
            self.codon.clone()
        }
        #[must_use]
        pub fn get_codon_string(&self) -> String {
            self.codon.join(", ")
        }
        #[must_use]
        pub fn get_codon_count(&self) -> usize {
            self.codon.len()
        }
    }

    impl Display for AminoAcid {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(
            f,
            "Name: {}\tShort Name: {}\tAbbreviation: {}\tSide Chain: {}\tMolecular Weight: {}\tCodon: {}",
            self.name,
            self.short_name,
            self.abbreviation,
            self.side_chain,
            self.molecular_weight,
            self.codon.join(", ")
        )
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_name() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCU", "GCC", "GCA", "GCG"],
            );
            assert_eq!(amino_acid.get_name(), "Alanine");
        }
        #[test]
        fn test_get_short_name() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCU", "GCC", "GCA", "GCG"],
            );
            assert_eq!(amino_acid.get_short_name(), "Ala");
        }
        #[test]
        fn test_get_abbreviation() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCU", "GCC", "GCA", "GCG"],
            );
            assert_eq!(amino_acid.get_abbreviation(), "A");
        }
        #[test]
        fn test_get_side_chain() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCU", "GCC", "GCA", "GCG"],
            );
            assert_eq!(amino_acid.get_side_chain(), SideChain::Nonpolar);
        }
        #[test]
        fn test_get_molecular_weight() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCU", "GCC", "GCA", "GCG"],
            );
            assert_eq!(amino_acid.get_molecular_weight(), 89.09);
        }
        #[test]
        fn test_get_codon() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCT", "GCC", "GCA", "GCG"],
            );
            assert_eq!(amino_acid.get_codon(), vec!["GCT", "GCC", "GCA", "GCG"]);
        }
        #[test]
        fn test_get_codon_string() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCT", "GCC", "GCA", "GCG"],
            );
            assert_eq!(amino_acid.get_codon_string(), "GCT, GCC, GCA, GCG");
        }
        #[test]
        fn test_get_codon_count() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCT", "GCC", "GCA", "GCG"],
            );
            assert_eq!(amino_acid.get_codon_count(), 4);
        }

        #[test]
        fn test_fmt() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCT", "GCC", "GCA", "GCG"],
            );
            assert_eq!(
            format!("{}", amino_acid),
            "Name: Alanine\tShort Name: Ala\tAbbreviation: A\tSide Chain: Nonpolar\tMolecular Weight: 89.09\tCodon: GCT, GCC, GCA, GCG"
        );
        }
    }
}

mod data {
    use super::models::AminoAcid;
    use std::fs;

    pub fn amino_acids() -> Vec<AminoAcid> {
        let mut amino_acids: Vec<AminoAcid> = Vec::new();
        let file = fs::read_to_string("src/amino_acid_data.json").expect("Unable to read file");
        let all_amino_acids: Vec<AminoAcid> = serde_json::from_str(&file).unwrap();
        for amino_acid in all_amino_acids {
            amino_acids.push(amino_acid);
        }
        amino_acids
    }
}

mod responses {
    use super::models::AminoAcid;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct AminoAcidResponse {
        pub amino_acid: AminoAcid,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AminoAcidNameResponse {
        pub name: String,
        pub short_name: String,
        pub abbreviation: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AminoAcidSideChainResponse {
        pub name: String,
        pub side_chain: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AminoAcidMolecularWeightResponse {
        pub name: String,
        pub molecular_weight: f64,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AminoAcidCodonResponse {
        pub name: String,
        pub codon: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AminoAcidCodonCountResponse {
        pub name: String,
        pub codon_count: usize,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AminoAcidShortNameResponse {
        pub name: String,
        pub short_name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AminoAcidAbbreviationResponse {
        pub name: String,
        pub abbreviation: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RootResponse {
        pub message: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ErrorResponse {
        pub error: String,
    }
}

mod routes {
    use axum::extract::Path;
    use axum::{http::StatusCode, Json};

    use crate::models::AminoAcid;
    use crate::responses::{
        AminoAcidAbbreviationResponse, AminoAcidCodonCountResponse, AminoAcidCodonResponse,
        AminoAcidMolecularWeightResponse, AminoAcidNameResponse, AminoAcidResponse,
        AminoAcidShortNameResponse, AminoAcidSideChainResponse, ErrorResponse, RootResponse,
    };

    fn match_amino_acid(amino_acid: String) -> Option<AminoAcid> {
        let amino_acids = crate::data::amino_acids();
        amino_acids
            .iter()
            .find(|&a| a.get_name().to_lowercase() == amino_acid.to_lowercase())
            .cloned()
    }

    pub async fn get_root() -> Result<(StatusCode, Json<RootResponse>), Json<ErrorResponse>> {
        let response = RootResponse {
            message: "Welcome to the Amino Acid API".to_string(),
        };
        Ok((StatusCode::OK, Json(response)))
    }

    pub async fn get_amino_acid(
        Path(amino_acid): Path<String>,
    ) -> Result<(StatusCode, Json<AminoAcidResponse>), (StatusCode, Json<ErrorResponse>)> {
        let matched: Option<AminoAcid> = match_amino_acid(amino_acid);
        match matched {
            None => {
                let response = ErrorResponse {
                    error: "Amino Acid not found".to_string(),
                };
                Err((StatusCode::NOT_FOUND, Json(response)))
            }
            Some(amino_acid) => {
                let response = AminoAcidResponse { amino_acid };
                Ok((StatusCode::OK, Json(response)))
            }
        }
    }

    pub async fn get_amino_acid_name(
        Path(amino_acid): Path<String>,
    ) -> Result<(StatusCode, Json<AminoAcidNameResponse>), (StatusCode, Json<ErrorResponse>)> {
        let matched = match_amino_acid(amino_acid);
        match matched {
            None => {
                let response = ErrorResponse {
                    error: "Amino Acid not found".to_string(),
                };
                Err((StatusCode::NOT_FOUND, Json(response)))
            }
            Some(amino_acid) => {
                let response = AminoAcidNameResponse {
                    name: amino_acid.get_name(),
                    short_name: amino_acid.get_short_name(),
                    abbreviation: amino_acid.get_abbreviation(),
                };
                Ok((StatusCode::OK, Json(response)))
            }
        }
    }

    pub async fn get_amino_acid_short_name(
        Path(amino_acid): Path<String>,
    ) -> Result<(StatusCode, Json<AminoAcidShortNameResponse>), (StatusCode, Json<ErrorResponse>)>
    {
        let matched = match_amino_acid(amino_acid);
        match matched {
            None => {
                let response = ErrorResponse {
                    error: "Amino Acid not found".to_string(),
                };
                Err((StatusCode::NOT_FOUND, Json(response)))
            }
            Some(amino_acid) => {
                let response = AminoAcidShortNameResponse {
                    name: amino_acid.get_name(),
                    short_name: amino_acid.get_short_name(),
                };
                Ok((StatusCode::OK, Json(response)))
            }
        }
    }

    pub async fn get_amino_acid_abbreviation(
        Path(amino_acid): Path<String>,
    ) -> Result<(StatusCode, Json<AminoAcidAbbreviationResponse>), (StatusCode, Json<ErrorResponse>)>
    {
        let matched = match_amino_acid(amino_acid);
        match matched {
            None => {
                let response = ErrorResponse {
                    error: "Amino Acid not found".to_string(),
                };
                Err((StatusCode::NOT_FOUND, Json(response)))
            }
            Some(amino_acid) => {
                let response = AminoAcidAbbreviationResponse {
                    name: amino_acid.get_name(),
                    abbreviation: amino_acid.get_abbreviation(),
                };
                Ok((StatusCode::OK, Json(response)))
            }
        }
    }
}

pub mod interface {
    use super::routes;
    use axum::routing::get;
    use axum::Router;

    pub fn create_router() -> Router {
        Router::new()
            .route("/", get(routes::get_root))
            .route("/:amino_acid", get(routes::get_amino_acid))
            .route("/:amino_acid/name", get(routes::get_amino_acid_name))
            .route(
                "/:amino_acid/short_name",
                get(routes::get_amino_acid_short_name),
            )
            .route(
                "/:amino_acid/abbreviation",
                get(routes::get_amino_acid_abbreviation),
            )
        // .route(
        //     "/:amino_acid/side_chain",
        //     get(routes::get_amino_acid_side_chain),
        // )
        // .route(
        //     "/:amino_acid/molecular_weight",
        //     get(routes::get_amino_acid_molecular_weight),
        // )
    }
}
