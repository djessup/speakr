// ============================================================================
//! Supported Whisper Models (ggml format)
//!
//! A **hand-curated** catalogue of the Whisper models that *Speakr* ships with
//! out-of-the-box. The information is sourced from the official
//! `whisper.cpp` HuggingFace repository and is kept in-sync at runtime by the
//! [`ModelListUpdater`].
//!
//! For quick reference the table below lists every variant together with its
//! on-disk **size** and **SHA-1** checksum:
// ============================================================================
//!
//! [Available models](https://huggingface.co/ggerganov/whisper.cpp/tree/main)
//!
//! | Model               | Disk    | SHA                                        |
//! | ------------------- | ------- | ------------------------------------------ |
//! | tiny                | 75 MiB  | `bd577a113a864445d4c299885e0cb97d4ba92b5f` |
//! | tiny-q5_1           | 31 MiB  | `2827a03e495b1ed3048ef28a6a4620537db4ee51` |
//! | tiny-q8_0           | 42 MiB  | `19e8118f6652a650569f5a949d962154e01571d9` |
//! | tiny.en             | 75 MiB  | `c78c86eb1a8faa21b369bcd33207cc90d64ae9df` |
//! | tiny.en-q5_1        | 31 MiB  | `3fb92ec865cbbc769f08137f22470d6b66e071b6` |
//! | tiny.en-q8_0        | 42 MiB  | `802d6668e7d411123e672abe4cb6c18f12306abb` |
//! | base                | 142 MiB | `465707469ff3a37a2b9b8d8f89f2f99de7299dac` |
//! | base-q5_1           | 57 MiB  | `a3733eda680ef76256db5fc5dd9de8629e62c5e7` |
//! | base-q8_0           | 78 MiB  | `7bb89bb49ed6955013b166f1b6a6c04584a20fbe` |
//! | base.en             | 142 MiB | `137c40403d78fd54d454da0f9bd998f78703390c` |
//! | base.en-q5_1        | 57 MiB  | `d26d7ce5a1b6e57bea5d0431b9c20ae49423c94a` |
//! | base.en-q8_0        | 78 MiB  | `bb1574182e9b924452bf0cd1510ac034d323e948` |
//! | small               | 466 MiB | `55356645c2b361a969dfd0ef2c5a50d530afd8d5` |
//! | small-q5_1          | 181 MiB | `6fe57ddcfdd1c6b07cdcc73aaf620810ce5fc771` |
//! | small-q8_0          | 252 MiB | `bcad8a2083f4e53d648d586b7dbc0cd673d8afad` |
//! | small.en            | 466 MiB | `db8a495a91d927739e50b3fc1cc4c6b8f6c2d022` |
//! | small.en-q5_1       | 181 MiB | `20f54878d608f94e4a8ee3ae56016571d47cba34` |
//! | small.en-q8_0       | 252 MiB | `9d75ff4ccfa0a8217870d7405cf8cef0a5579852` |
//! | small.en-tdrz       | 465 MiB | `b6c6e7e89af1a35c08e6de56b66ca6a02a2fdfa1` |
//! | medium              | 1.5 GiB | `fd9727b6e1217c2f614f9b698455c4ffd82463b4` |
//! | medium-q5_0         | 514 MiB | `7718d4c1ec62ca96998f058114db98236937490e` |
//! | medium-q8_0         | 785 MiB | `e66645948aff4bebbec71b3485c576f3d63af5d6` |
//! | medium.en           | 1.5 GiB | `8c30f0e44ce9560643ebd10bbe50cd20eafd3723` |
//! | medium.en-q5_0      | 514 MiB | `bb3b5281bddd61605d6fc76bc5b92d8f20284c3b` |
//! | medium.en-q8_0      | 785 MiB | `b1cf48c12c807e14881f634fb7b6c6ca867f6b38` |
//! | large-v1            | 2.9 GiB | `b1caaf735c4cc1429223d5a74f0f4d0b9b59a299` |
//! | large-v2            | 2.9 GiB | `0f4c8e34f21cf1a914c59d8b3ce882345ad349d6` |
//! | large-v2-q5_0       | 1.1 GiB | `00e39f2196344e901b3a2bd5814807a769bd1630` |
//! | large-v2-q8_0       | 1.5 GiB | `da97d6ca8f8ffbeeb5fd147f79010eeea194ba38` |
//! | large-v3            | 2.9 GiB | `ad82bf6a9043ceed055076d0fd39f5f186ff8062` |
//! | large-v3-q5_0       | 1.1 GiB | `e6e2ed78495d403bef4b7cff42ef4aaadcfea8de` |
//! | large-v3-turbo      | 1.5 GiB | `4af2b29d7ec73d781377bfd1758ca957a807e941` |
//! | large-v3-turbo-q5_0 | 547 MiB | `e050f7970618a659205450ad97eb95a18d69c9ee` |
//! | large-v3-turbo-q8_0 | 834 MiB | `01bf15bedffe9f39d65c1b6ff9b687ea91f59e0e` |
//! Example direct download link:
//! <https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.en-q5_0.bin>
use size::Size;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Model {
    Tiny,
    TinyQuantizedQ5_1,
    TinyQuantizedQ8_0,
    TinyEn,
    TinyEnQuantizedQ5_1,
    TinyEnQuantizedQ8_0,
    Base,
    BaseQuantizedQ5_1,
    BaseQuantizedQ8_0,
    BaseEn,
    BaseEnQuantizedQ5_1,
    BaseEnQuantizedQ8_0,
    Small,
    SmallQuantizedQ5_1,
    SmallQuantizedQ8_0,
    SmallEn,
    SmallEnQuantizedQ5_1,
    SmallEnQuantizedQ8_0,
    Medium,
    MediumQuantizedQ5_0,
    MediumQuantizedQ8_0,
    MediumEn,
    MediumEnQuantizedQ5_0,
    MediumEnQuantizedQ8_0,
    LargeV1,
    LargeV2,
    LargeV2QuantizedQ5_0,
    LargeV2QuantizedQ8_0,
    LargeV3,
    LargeV3QuantizedQ5_0,
    LargeV3Turbo,
    LargeV3TurboQuantizedQ5_0,
    LargeV3TurboQuantizedQ8_0,
}

impl Model {
    pub fn filename(&self) -> &'static str {
        match self {
            Model::Tiny => "tiny",
            Model::TinyQuantizedQ5_1 => "tiny-q5_1",
            Model::TinyQuantizedQ8_0 => "tiny-q8_0",
            Model::TinyEn => "tiny.en",
            Model::TinyEnQuantizedQ5_1 => "tiny.en-q5_1",
            Model::TinyEnQuantizedQ8_0 => "tiny.en-q8_0",
            Model::Base => "base",
            Model::BaseQuantizedQ5_1 => "base-q5_1",
            Model::BaseQuantizedQ8_0 => "base-q8_0",
            Model::BaseEn => "base.en",
            Model::BaseEnQuantizedQ5_1 => "base.en-q5_1",
            Model::BaseEnQuantizedQ8_0 => "base.en-q8_0",
            Model::Small => "small",
            Model::SmallQuantizedQ5_1 => "small-q5_1",
            Model::SmallQuantizedQ8_0 => "small-q8_0",
            Model::SmallEn => "small.en",
            Model::SmallEnQuantizedQ5_1 => "small.en-q5_1",
            Model::SmallEnQuantizedQ8_0 => "small.en-q8_0",
            Model::Medium => "medium",
            Model::MediumQuantizedQ5_0 => "medium-q5_0",
            Model::MediumQuantizedQ8_0 => "medium-q8_0",
            Model::MediumEn => "medium.en",
            Model::MediumEnQuantizedQ5_0 => "medium.en-q5_0",
            Model::MediumEnQuantizedQ8_0 => "medium.en-q8_0",
            Model::LargeV1 => "large-v1",
            Model::LargeV2 => "large-v2",
            Model::LargeV2QuantizedQ5_0 => "large-v2-q5_0",
            Model::LargeV2QuantizedQ8_0 => "large-v2-q8_0",
            Model::LargeV3 => "large-v3",
            Model::LargeV3QuantizedQ5_0 => "large-v3-q5_0",
            Model::LargeV3Turbo => "large-v3-turbo",
            Model::LargeV3TurboQuantizedQ5_0 => "large-v3-turbo-q5_0",
            Model::LargeV3TurboQuantizedQ8_0 => "large-v3-turbo-q8_0",
        }
    }

    pub fn filesize(&self) -> Size {
        match self {
            Model::Tiny => Size::from_mib(75),
            Model::TinyQuantizedQ5_1 => Size::from_mib(31),
            Model::TinyQuantizedQ8_0 => Size::from_mib(42),
            Model::TinyEn => Size::from_mib(75),
            Model::TinyEnQuantizedQ5_1 => Size::from_mib(31),
            Model::TinyEnQuantizedQ8_0 => Size::from_mib(42),
            Model::Base => Size::from_mib(142),
            Model::BaseQuantizedQ5_1 => Size::from_mib(57),
            Model::BaseQuantizedQ8_0 => Size::from_mib(78),
            Model::BaseEn => Size::from_mib(142),
            Model::BaseEnQuantizedQ5_1 => Size::from_mib(57),
            Model::BaseEnQuantizedQ8_0 => Size::from_mib(78),
            Model::Small => Size::from_mib(466),
            Model::SmallQuantizedQ5_1 => Size::from_mib(181),
            Model::SmallQuantizedQ8_0 => Size::from_mib(252),
            Model::SmallEn => Size::from_mib(466),
            Model::SmallEnQuantizedQ5_1 => Size::from_mib(181),
            Model::SmallEnQuantizedQ8_0 => Size::from_mib(252),
            Model::Medium => Size::from_gib(1.5),
            Model::MediumQuantizedQ5_0 => Size::from_mib(514),
            Model::MediumQuantizedQ8_0 => Size::from_mib(785),
            Model::MediumEn => Size::from_gib(1.5),
            Model::MediumEnQuantizedQ5_0 => Size::from_mib(514),
            Model::MediumEnQuantizedQ8_0 => Size::from_mib(785),
            Model::LargeV1 => Size::from_gib(2.9),
            Model::LargeV2 => Size::from_gib(2.9),
            Model::LargeV2QuantizedQ5_0 => Size::from_gib(1.1),
            Model::LargeV2QuantizedQ8_0 => Size::from_gib(1.5),
            Model::LargeV3 => Size::from_gib(2.9),
            Model::LargeV3QuantizedQ5_0 => Size::from_gib(1.1),
            Model::LargeV3Turbo => Size::from_gib(1.5),
            Model::LargeV3TurboQuantizedQ5_0 => Size::from_mib(547),
            Model::LargeV3TurboQuantizedQ8_0 => Size::from_mib(834),
        }
    }

    /// Approximate peak RAM usage (in megabytes) required to load the model into memory.
    ///
    /// These numbers are based on official whisper.cpp benchmarks and empirical measurements.
    /// Memory usage includes model weights, activations, and inference buffers. Quantized
    /// models typically use 30-50% less memory than their full-precision counterparts.
    ///
    /// **Note**: These are peak memory estimates during inference. Actual usage may vary
    /// depending on hardware, batch size, and audio length. For production use, ensure
    /// at least 50% additional headroom for system processes and UI.
    pub fn memory_usage_mb(&self) -> u32 {
        match self {
            // Tiny models: ~273 MB (official whisper.cpp measurement)
            // Quantized versions use ~40-50% less memory
            Model::Tiny | Model::TinyEn => 273,
            Model::TinyQuantizedQ5_1 | Model::TinyEnQuantizedQ5_1 => 164, // ~40% reduction
            Model::TinyQuantizedQ8_0 | Model::TinyEnQuantizedQ8_0 => 191, // ~30% reduction

            // Base models: ~388 MB (official whisper.cpp measurement)
            // Quantized versions use ~40-50% less memory
            Model::Base | Model::BaseEn => 388,
            Model::BaseQuantizedQ5_1 | Model::BaseEnQuantizedQ5_1 => 233, // ~40% reduction
            Model::BaseQuantizedQ8_0 | Model::BaseEnQuantizedQ8_0 => 272, // ~30% reduction

            // Small models: ~852 MB (official whisper.cpp measurement)
            // Quantized versions use ~40-50% less memory
            Model::Small | Model::SmallEn => 852,
            Model::SmallQuantizedQ5_1 | Model::SmallEnQuantizedQ5_1 => 511, // ~40% reduction
            Model::SmallQuantizedQ8_0 | Model::SmallEnQuantizedQ8_0 => 596, // ~30% reduction

            // Medium models: ~2.1 GB (official whisper.cpp measurement)
            // Quantized versions use ~40-50% less memory
            Model::Medium | Model::MediumEn => 2150,
            Model::MediumQuantizedQ5_0 | Model::MediumEnQuantizedQ5_0 => 1290, // ~40% reduction
            Model::MediumQuantizedQ8_0 | Model::MediumEnQuantizedQ8_0 => 1505, // ~30% reduction

            // Large V3 Turbo: ~1.5 GB (based on model size and architecture)
            // Quantized versions use ~40-50% less memory
            Model::LargeV3Turbo => 1500,
            Model::LargeV3TurboQuantizedQ5_0 => 900, // ~40% reduction
            Model::LargeV3TurboQuantizedQ8_0 => 1050, // ~30% reduction

            // Large models: ~3.9 GB (official whisper.cpp measurement)
            // Quantized versions use ~40-50% less memory
            Model::LargeV1 | Model::LargeV2 | Model::LargeV3 => 3900,
            Model::LargeV2QuantizedQ5_0 | Model::LargeV3QuantizedQ5_0 => 2340, // ~40% reduction
            Model::LargeV2QuantizedQ8_0 => 2730,                               // ~30% reduction
        }
    }

    /// Returns a list of ISO-639-1 language codes that the model is *restricted* to.
    ///
    /// Models with the `.en` suffix are English-only.  All other models support the
    /// full multilingual vocabulary, hence `None` is returned.
    pub fn supported_languages(&self) -> Option<&'static [&'static str]> {
        if self.filename().ends_with(".en") {
            Some(&["en"]) // English-only models
        } else {
            None // Multilingual
        }
    }

    pub fn sha(&self) -> &'static str {
        match self {
            Model::Tiny => "bd577a113a864445d4c299885e0cb97d4ba92b5f",
            Model::TinyQuantizedQ5_1 => "2827a03e495b1ed3048ef28a6a4620537db4ee51",
            Model::TinyQuantizedQ8_0 => "19e8118f6652a650569f5a949d962154e01571d9",
            Model::TinyEn => "c78c86eb1a8faa21b369bcd33207cc90d64ae9df",
            Model::TinyEnQuantizedQ5_1 => "3fb92ec865cbbc769f08137f22470d6b66e071b6",
            Model::TinyEnQuantizedQ8_0 => "802d6668e7d411123e672abe4cb6c18f12306abb",
            Model::Base => "465707469ff3a37a2b9b8d8f89f2f99de7299dac",
            Model::BaseQuantizedQ5_1 => "a3733eda680ef76256db5fc5dd9de8629e62c5e7",
            Model::BaseQuantizedQ8_0 => "7bb89bb49ed6955013b166f1b6a6c04584a20fbe",
            Model::BaseEn => "137c40403d78fd54d454da0f9bd998f78703390c",
            Model::BaseEnQuantizedQ5_1 => "d26d7ce5a1b6e57bea5d0431b9c20ae49423c94a",
            Model::BaseEnQuantizedQ8_0 => "bb1574182e9b924452bf0cd1510ac034d323e948",
            Model::Small => "55356645c2b361a969dfd0ef2c5a50d530afd8d5",
            Model::SmallQuantizedQ5_1 => "6fe57ddcfdd1c6b07cdcc73aaf620810ce5fc771",
            Model::SmallQuantizedQ8_0 => "bcad8a2083f4e53d648d586b7dbc0cd673d8afad",
            Model::SmallEn => "db8a495a91d927739e50b3fc1cc4c6b8f6c2d022",
            Model::SmallEnQuantizedQ5_1 => "20f54878d608f94e4a8ee3ae56016571d47cba34",
            Model::SmallEnQuantizedQ8_0 => "9d75ff4ccfa0a8217870d7405cf8cef0a5579852",
            Model::Medium => "fd9727b6e1217c2f614f9b698455c4ffd82463b4",
            Model::MediumQuantizedQ5_0 => "7718d4c1ec62ca96998f058114db98236937490e",
            Model::MediumQuantizedQ8_0 => "e66645948aff4bebbec71b3485c576f3d63af5d6",
            Model::MediumEn => "8c30f0e44ce9560643ebd10bbe50cd20eafd3723",
            Model::MediumEnQuantizedQ5_0 => "bb3b5281bddd61605d6fc76bc5b92d8f20284c3b",
            Model::MediumEnQuantizedQ8_0 => "b1cf48c12c807e14881f634fb7b6c6ca867f6b38",
            Model::LargeV1 => "b1caaf735c4cc1429223d5a74f0f4d0b9b59a299",
            Model::LargeV2 => "0f4c8e34f21cf1a914c59d8b3ce882345ad349d6",
            Model::LargeV2QuantizedQ5_0 => "00e39f2196344e901b3a2bd5814807a769bd1630",
            Model::LargeV2QuantizedQ8_0 => "da97d6ca8f8ffbeeb5fd147f79010eeea194ba38",
            Model::LargeV3 => "ad82bf6a9043ceed055076d0fd39f5f186ff8062",
            Model::LargeV3QuantizedQ5_0 => "e6e2ed78495d403bef4b7cff42ef4aaadcfea8de",
            Model::LargeV3Turbo => "4af2b29d7ec73d781377bfd1758ca957a807e941",
            Model::LargeV3TurboQuantizedQ5_0 => "e050f7970618a659205450ad97eb95a18d69c9ee",
            Model::LargeV3TurboQuantizedQ8_0 => "01bf15bedffe9f39d65c1b6ff9b687ea91f59e0e",
        }
    }

    pub fn url(&self) -> String {
        format!(
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/f281eb45af861ab5e5297d23694b7d46e090c02c/ggml-{}.bin",
            self.filename()
        )
    }

    /// Returns an iterator over **all** supported models.
    pub fn iter() -> impl Iterator<Item = Self> {
        use Model::*;
        [
            Tiny,
            TinyQuantizedQ5_1,
            TinyQuantizedQ8_0,
            TinyEn,
            TinyEnQuantizedQ5_1,
            TinyEnQuantizedQ8_0,
            Base,
            BaseQuantizedQ5_1,
            BaseQuantizedQ8_0,
            BaseEn,
            BaseEnQuantizedQ5_1,
            BaseEnQuantizedQ8_0,
            Small,
            SmallQuantizedQ5_1,
            SmallQuantizedQ8_0,
            SmallEn,
            SmallEnQuantizedQ5_1,
            SmallEnQuantizedQ8_0,
            Medium,
            MediumQuantizedQ5_0,
            MediumQuantizedQ8_0,
            MediumEn,
            MediumEnQuantizedQ5_0,
            MediumEnQuantizedQ8_0,
            LargeV1,
            LargeV2,
            LargeV2QuantizedQ5_0,
            LargeV2QuantizedQ8_0,
            LargeV3,
            LargeV3QuantizedQ5_0,
            LargeV3Turbo,
            LargeV3TurboQuantizedQ5_0,
            LargeV3TurboQuantizedQ8_0,
        ]
        .into_iter()
    }
}
