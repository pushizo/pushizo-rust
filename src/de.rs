//! Üretilen modellerin serde yardımcıları.

use serde::{Deserialize, Deserializer};

/// Zorunlu alan `null` gelirse varsayılan değere düşer (Go SDK'sındaki sıfır değer davranışı).
pub(crate) fn null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}
