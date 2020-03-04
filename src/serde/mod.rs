/// IPLD DagCBOR serialization/deserialization.
/// See the [link](https://github.com/ipld/specs/blob/master/block-layer/codecs/dag-cbor.md#link-format) for details.
///
/// As with all IPLD formats, DagCBOR must be able to encode [Links](https://github.com/ipld/specs/blob/master/data-model-layer/data-model.md#link-kind).
/// In DagCBOR, links are CIDs encoded using the raw-binary identity Multibase.
/// That Multibase prefix (0x00) must not be omitted.
/// They are stored as byte-string type (major type 2), with the tag 42.
pub mod ipld_dag_cbor;

/// IPLD DagJSON serialization/deserialization.
pub mod ipld_dag_json;

use serde::{de, ser};

use crate::cid::Cid;

impl ser::Serialize for Cid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self::ipld_dag_cbor::serialize(self, serializer)
    }
}

impl<'de> de::Deserialize<'de> for Cid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        self::ipld_dag_cbor::deserialize(deserializer)
    }
}
