// This crate deliberately does not use the same linting rules as the other
// crates because of all the generated code it contains that we don't have much
// control over.
#![deny(rustdoc::broken_intra_doc_links, rustdoc::bare_urls)]
#![allow(
    clippy::derive_partial_eq_without_eq,
    clippy::needless_borrow,
    clippy::needless_borrows_for_generic_args
)]
#![warn(unused_crate_dependencies)]

// Workaround for "unused crate" lint false positives.
use workspace_hack as _;

// Re-export prost for users of proto types.
pub use prost;

/// This module imports the generated protobuf code into a Rust module
/// hierarchy that matches the namespace hierarchy of the protobuf
/// definitions
pub mod influxdata {
    pub mod platform {
        pub mod storage {
            include!(concat!(
                env!("OUT_DIR"),
                "/influxdata.platform.storage.read.rs"
            ));
            include!(concat!(
                env!("OUT_DIR"),
                "/influxdata.platform.storage.read.serde.rs"
            ));

            include!(concat!(env!("OUT_DIR"), "/influxdata.platform.storage.rs"));
            include!(concat!(
                env!("OUT_DIR"),
                "/influxdata.platform.storage.serde.rs"
            ));

            // Can't implement `Default` because `prost::Message` implements `Default`
            impl TimestampRange {
                pub fn max() -> Self {
                    TimestampRange {
                        start: std::i64::MIN,
                        end: std::i64::MAX,
                    }
                }
            }
        }
        pub mod errors {
            include!(concat!(env!("OUT_DIR"), "/influxdata.platform.errors.rs"));
            include!(concat!(
                env!("OUT_DIR"),
                "/influxdata.platform.errors.serde.rs"
            ));
        }
    }

    pub mod iox {
        pub mod authz {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/influxdata.iox.authz.v1.rs"));
                include!(concat!(
                    env!("OUT_DIR"),
                    "/influxdata.iox.authz.v1.serde.rs"
                ));
            }
        }

        pub mod catalog {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/influxdata.iox.catalog.v1.rs"));
                include!(concat!(
                    env!("OUT_DIR"),
                    "/influxdata.iox.catalog.v1.serde.rs"
                ));
            }
        }

        pub mod compactor {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/influxdata.iox.compactor.v1.rs"));
                include!(concat!(
                    env!("OUT_DIR"),
                    "/influxdata.iox.compactor.v1.serde.rs"
                ));
            }
        }

        pub mod delete {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/influxdata.iox.delete.v1.rs"));
                include!(concat!(
                    env!("OUT_DIR"),
                    "/influxdata.iox.delete.v1.serde.rs"
                ));
            }
        }

        pub mod gossip {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/influxdata.iox.gossip.v1.rs"));
            }

            /// The set of topics used for IOx gossiping.
            ///
            /// NOTE: Don't renumber topics. Don't re-use numbers. Use the range
            /// 0 to 63 for numbers.
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum Topic {
                /// New namespace, table, and column additions observed and
                /// broadcast by the routers.
                SchemaChanges = 1,

                /// Parquet file creation notifications.
                NewParquetFiles = 2,

                /// Compaction round completion notifications.
                CompactionEvents = 3,

                /// Schema cache consistency check / sync / convergence
                /// messages.
                SchemaCacheConsistency = 4,
            }

            impl TryFrom<u64> for Topic {
                type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

                fn try_from(v: u64) -> Result<Self, Self::Error> {
                    Ok(match v {
                        v if v == Self::SchemaChanges as u64 => Self::SchemaChanges,
                        v if v == Self::NewParquetFiles as u64 => Self::NewParquetFiles,
                        v if v == Self::CompactionEvents as u64 => Self::CompactionEvents,
                        v if v == Self::SchemaCacheConsistency as u64 => {
                            Self::SchemaCacheConsistency
                        }
                        _ => return Err(format!("unknown topic id {}", v).into()),
                    })
                }
            }

            impl From<Topic> for u64 {
                fn from(v: Topic) -> u64 {
                    v as u64
                }
            }
        }

        pub mod ingester {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/influxdata.iox.ingester.v1.rs"));
                include!(concat!(
                    env!("OUT_DIR"),
                    "/influxdata.iox.ingester.v1.serde.rs"
                ));
            }
        }

        pub mod namespace {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/influxdata.iox.namespace.v1.rs"));
                include!(concat!(
                    env!("OUT_DIR"),
                    "/influxdata.iox.namespace.v1.serde.rs"
                ));
            }
        }

        pub mod object_store {
            pub mod v1 {
                include!(concat!(
                    env!("OUT_DIR"),
                    "/influxdata.iox.object_store.v1.rs"
                ));
                include!(concat!(
                    env!("OUT_DIR"),
                    "/influxdata.iox.object_store.v1.serde.rs"
                ));
            }
        }

        pub mod partition_template {
            pub mod v1 {
                include!(concat!(
                    env!("OUT_DIR"),
                    "/influxdata.iox.partition_template.v1.rs"
                ));
                include!(concat!(
                    env!("OUT_DIR"),
                    "/influxdata.iox.partition_template.v1.serde.rs"
                ));
            }
        }

        pub mod predicate {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/influxdata.iox.predicate.v1.rs"));
                include!(concat!(
                    env!("OUT_DIR"),
                    "/influxdata.iox.predicate.v1.serde.rs"
                ));
            }
        }

        pub mod querier {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/influxdata.iox.querier.v1.rs"));
                include!(concat!(
                    env!("OUT_DIR"),
                    "/influxdata.iox.querier.v1.serde.rs"
                ));
            }
        }

        pub mod schema {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/influxdata.iox.schema.v1.rs"));
                include!(concat!(
                    env!("OUT_DIR"),
                    "/influxdata.iox.schema.v1.serde.rs"
                ));
            }
        }

        pub mod table {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/influxdata.iox.table.v1.rs"));
                include!(concat!(
                    env!("OUT_DIR"),
                    "/influxdata.iox.table.v1.serde.rs"
                ));
            }
        }

        pub mod wal {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/influxdata.iox.wal.v1.rs"));
                include!(concat!(env!("OUT_DIR"), "/influxdata.iox.wal.v1.serde.rs"));
            }
        }
    }

    pub mod pbdata {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/influxdata.pbdata.v1.rs"));
            include!(concat!(env!("OUT_DIR"), "/influxdata.pbdata.v1.serde.rs"));
        }
    }
}

// Needed because of https://github.com/hyperium/tonic/issues/471
pub mod grpc {
    pub mod health {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/grpc.health.v1.rs"));
        }
    }
}

/// gRPC Storage Service
pub const STORAGE_SERVICE: &str = "influxdata.platform.storage.Storage";

/// gRPC Testing Service
pub const IOX_TESTING_SERVICE: &str = "influxdata.platform.storage.IOxTesting";

/// gRPC Arrow Flight Service
pub const ARROW_SERVICE: &str = "arrow.flight.protocol.FlightService";

/// The type prefix for any types
pub const ANY_TYPE_PREFIX: &str = "type.googleapis.com";

/// Returns the protobuf URL usable with a google.protobuf.Any message
/// This is the full Protobuf package and message name prefixed by
/// "type.googleapis.com/"
pub fn protobuf_type_url(protobuf_type: &str) -> String {
    format!("{ANY_TYPE_PREFIX}/{protobuf_type}")
}

/// Protobuf file descriptor containing all generated types.
/// Useful in gRPC reflection.
pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("proto_descriptor");

/// Compares the protobuf type URL found within a google.protobuf.Any
/// message to an expected Protobuf package and message name
///
/// i.e. strips off the "type.googleapis.com/" prefix from `url`
/// and compares the result with `protobuf_type`
///
/// ```
/// use generated_types::protobuf_type_url_eq;
/// assert!(protobuf_type_url_eq("type.googleapis.com/google.protobuf.Empty", "google.protobuf.Empty"));
/// assert!(!protobuf_type_url_eq("type.googleapis.com/google.protobuf.Empty", "something.else"));
/// ```
pub fn protobuf_type_url_eq(url: &str, protobuf_type: &str) -> bool {
    let mut split = url.splitn(2, '/');
    match (split.next(), split.next()) {
        (Some(ANY_TYPE_PREFIX), Some(t)) => t == protobuf_type,
        _ => false,
    }
}

// TODO: Remove these (#2419)
pub use influxdata::platform::storage::*;

pub mod google;

pub use prost::{DecodeError, EncodeError};

#[cfg(test)]
mod tests {
    use crate::influxdata::iox::gossip::Topic;

    use super::*;

    #[test]
    fn test_protobuf_type_url() {
        let t = protobuf_type_url(STORAGE_SERVICE);

        assert_eq!(
            &t,
            "type.googleapis.com/influxdata.platform.storage.Storage"
        );

        assert!(protobuf_type_url_eq(&t, STORAGE_SERVICE));
        assert!(!protobuf_type_url_eq(&t, "foo"));

        // The URL must start with the type.googleapis.com prefix
        assert!(!protobuf_type_url_eq(STORAGE_SERVICE, STORAGE_SERVICE,));
    }

    #[test]
    fn test_gossip_topics() {
        let topics = [
            Topic::SchemaChanges,
            Topic::NewParquetFiles,
            Topic::CompactionEvents,
            Topic::SchemaCacheConsistency,
        ];

        for topic in topics {
            let v = u64::from(topic);
            let got = Topic::try_from(v).expect("failed to round-trip topic");
            assert_eq!(got, topic);
        }

        // Adding a new topic? Add it to the test cases too and then add it to
        // this match (that forces a compile-time error and makes you read this
        // message).
        match topics[0] {
            Topic::SchemaChanges => {}
            Topic::NewParquetFiles => {}
            Topic::CompactionEvents => {}
            Topic::SchemaCacheConsistency => {}
        }
    }
}
