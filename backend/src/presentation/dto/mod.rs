pub mod auth_dto;
pub mod berita_dto;
pub mod galeri_dto;
pub mod lowongan_dto;
pub mod penduduk_dto;

use serde::{Deserialize, Deserializer};

/// Distinguishes "JSON key absent" from "JSON key present with value null"
/// when deserializing into `Option<Option<T>>`. Required for PATCH semantics
/// on nullable columns: absent = leave alone, null = clear, value = set.
///
/// Use with `#[serde(default, deserialize_with = "double_option")]`.
pub fn double_option<'de, T, D>(de: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(de).map(Some)
}
