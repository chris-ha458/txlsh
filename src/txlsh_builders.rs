use crate::TxLshBuilder;
use crate::{BucketKind, ChecksumKind, Version};

pub fn default_builder() -> TxLshBuilder {
    TxLshBuilder::new(
        BucketKind::Bucket128,
        ChecksumKind::OneByte,
        Version::Version4,
    )
}

pub fn full_builder() -> TxLshBuilder {
    TxLshBuilder::new(
        BucketKind::Bucket256,
        ChecksumKind::ThreeByte,
        Version::Version4,
    )
}

pub fn tx_lsh_builder() -> TxLshBuilder {
    TxLshBuilder::new(
        BucketKind::Bucket256,
        ChecksumKind::ThreeByte,
        Version::TxLshV1,
    )
}
