// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkOS library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::hash::Hash;
use std::{
    collections::{BTreeMap, HashMap},
    net::{IpAddr, SocketAddr},
};

use parking_lot::RwLock;
use snarkvm::{console::types::Field, ledger::narwhal::TransmissionID, prelude::Network};
use time::{Duration, OffsetDateTime};

#[derive(Debug)]
pub struct Cache<N: Network> {
    /// The ordered timestamp map of peer connections and cache hits.
    seen_inbound_connections: RwLock<BTreeMap<OffsetDateTime, HashMap<IpAddr, u32>>>,
    /// The ordered timestamp map of peer IPs and cache hits.
    seen_inbound_events: RwLock<BTreeMap<OffsetDateTime, HashMap<SocketAddr, u32>>>,
    /// The ordered timestamp map of certificate IDs and cache hits.
    seen_inbound_certificates: RwLock<BTreeMap<OffsetDateTime, HashMap<Field<N>, u32>>>,
    /// The ordered timestamp map of transmission IDs and cache hits.
    seen_inbound_transmissions: RwLock<BTreeMap<OffsetDateTime, HashMap<TransmissionID<N>, u32>>>,
    /// The ordered timestamp map of peer IPs and their cache hits on outbound events.
    seen_outbound_events: RwLock<BTreeMap<OffsetDateTime, HashMap<SocketAddr, u32>>>,
    /// The ordered timestamp map of peer IPs and their cache hits on certificate requests.
    seen_outbound_certificates: RwLock<BTreeMap<OffsetDateTime, HashMap<SocketAddr, u32>>>,
    /// The ordered timestamp map of peer IPs and their cache hits on transmission requests.
    seen_outbound_transmissions: RwLock<BTreeMap<OffsetDateTime, HashMap<SocketAddr, u32>>>,
}

impl<N: Network> Default for Cache<N> {
    /// Initializes a new instance of the cache.
    fn default() -> Self {
        Self::new()
    }
}

impl<N: Network> Cache<N> {
    /// Initializes a new instance of the cache.
    pub fn new() -> Self {
        Self {
            seen_inbound_connections: Default::default(),
            seen_inbound_events: Default::default(),
            seen_inbound_certificates: Default::default(),
            seen_inbound_transmissions: Default::default(),
            seen_outbound_events: Default::default(),
            seen_outbound_certificates: Default::default(),
            seen_outbound_transmissions: Default::default(),
        }
    }
}

impl<N: Network> Cache<N> {
    /// Inserts a new timestamp for the given peer connection, returning the number of recent connection requests.
    pub fn insert_inbound_connection(&self, peer_ip: IpAddr, interval_in_secs: i64) -> usize {
        Self::retain_and_insert(&self.seen_inbound_connections, peer_ip, interval_in_secs)
    }

    /// Inserts a new timestamp for the given peer, returning the number of recent events.
    pub fn insert_inbound_event(&self, peer_ip: SocketAddr, interval_in_secs: i64) -> usize {
        Self::retain_and_insert(&self.seen_inbound_events, peer_ip, interval_in_secs)
    }

    /// Inserts a certificate ID into the cache, returning the number of recent events.
    pub fn insert_inbound_certificate(&self, key: Field<N>, interval_in_secs: i64) -> usize {
        Self::retain_and_insert(&self.seen_inbound_certificates, key, interval_in_secs)
    }

    /// Inserts a transmission ID into the cache, returning the number of recent events.
    pub fn insert_inbound_transmission(&self, key: TransmissionID<N>, interval_in_secs: i64) -> usize {
        Self::retain_and_insert(&self.seen_inbound_transmissions, key, interval_in_secs)
    }
}

impl<N: Network> Cache<N> {
    /// Inserts a new timestamp for the given peer, returning the number of recent events.
    pub fn insert_outbound_event(&self, peer_ip: SocketAddr, interval_in_secs: i64) -> usize {
        Self::retain_and_insert(&self.seen_outbound_events, peer_ip, interval_in_secs)
    }

    /// Inserts a new timestamp for the given peer, returning the number of recent events.
    pub fn insert_outbound_certificate(&self, peer_ip: SocketAddr, interval_in_secs: i64) -> usize {
        Self::retain_and_insert(&self.seen_outbound_certificates, peer_ip, interval_in_secs)
    }

    /// Inserts a new timestamp for the given peer, returning the number of recent events.
    pub fn insert_outbound_transmission(&self, peer_ip: SocketAddr, interval_in_secs: i64) -> usize {
        Self::retain_and_insert(&self.seen_outbound_transmissions, peer_ip, interval_in_secs)
    }
}

impl<N: Network> Cache<N> {
    /// Insert a new timestamp for the given key, returning the number of recent entries.
    fn retain_and_insert<K: Copy + Clone + PartialEq + Eq + Hash>(
        map: &RwLock<BTreeMap<OffsetDateTime, HashMap<K, u32>>>,
        key: K,
        interval_in_secs: i64,
    ) -> usize {
        // Fetch the current timestamp.
        let now = OffsetDateTime::now_utc();

        // Get the write lock.
        let mut map_write = map.write();
        // Insert the new timestamp and increment the frequency for the key.
        *map_write.entry(now).or_default().entry(key).or_default() += 1;
        // Extract the subtree after interval (i.e. non-expired entries)
        let retained = map_write.split_off(&now.saturating_sub(Duration::seconds(interval_in_secs)));
        // Clear all the expired entries.
        map_write.clear();
        // Reinsert the entries into map and sum the frequency of recent requests for `key` while looping.
        let mut cache_hits = 0;
        for (time, cache_keys) in retained {
            cache_hits += *cache_keys.get(&key).unwrap_or(&0);
            map_write.insert(time, cache_keys);
        }
        // Return the frequency.
        cache_hits as usize
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use snarkvm::prelude::Testnet3;

    use super::*;

    type CurrentNetwork = Testnet3;

    trait Input {
        fn input() -> Self;
    }

    impl Input for IpAddr {
        fn input() -> Self {
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
        }
    }

    impl Input for SocketAddr {
        fn input() -> Self {
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234)
        }
    }

    impl Input for Field<CurrentNetwork> {
        fn input() -> Self {
            Field::from_u8(1)
        }
    }

    impl Input for TransmissionID<CurrentNetwork> {
        fn input() -> Self {
            TransmissionID::Transaction(Default::default())
        }
    }

    const INTERVAL_IN_SECS: i64 = 1;

    macro_rules! test_cache_fields {
        ($($name:ident),*) => {
            $(
                paste::paste! {
                    #[test]
                    fn [<test_seen_ $name s>]() {
                        let cache = Cache::<CurrentNetwork>::default();
                        let input = Input::input();

                        // Check that the cache is empty.
                        assert!(cache.[<seen_ $name s>].read().is_empty());

                        // Insert an input, recent events should be 1.
                        assert_eq!(cache.[<insert_ $name>](input, INTERVAL_IN_SECS), 1);
                        // Insert an input, recent events should be 2.
                        assert_eq!(cache.[<insert_ $name>](input, INTERVAL_IN_SECS), 2);
                        // Insert an input, recent events should be 3.
                        assert_eq!(cache.[<insert_ $name>](input, INTERVAL_IN_SECS), 3);

                        // Check that the cache contains the input for 3 entries.
                        assert_eq!(cache.[<seen_ $name s>].read().len(), 3);

                        // Wait for the input to expire.
                        std::thread::sleep(std::time::Duration::from_secs(INTERVAL_IN_SECS as u64 + 1));

                        // Insert an input again, recent events should be 1.
                        assert_eq!(cache.[<insert_ $name>](input, INTERVAL_IN_SECS), 1);

                        // Check that the cache still contains the input.
                        let counts: u32 = cache.[<seen_ $name s>].read().values().map(|hash_map| hash_map.get(&input).unwrap_or(&0)).cloned().sum();
                        assert_eq!(counts, 1);

                        // Check that the cache contains the input and 1 timestamp entry.
                        assert_eq!(cache.[<seen_ $name s>].read().len(), 1);
                    }
                }
            )*
        }
    }

    test_cache_fields! {
       inbound_connection,
       inbound_event,
       inbound_certificate,
       inbound_transmission,
       outbound_event,
       outbound_certificate,
       outbound_transmission
    }
}
