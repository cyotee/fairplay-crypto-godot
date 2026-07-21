//! In-process multi-seat simulator for offline multi-party tests/samples.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use crate::wire::FairplayEnvelope;

/// Shared bus for N local seats (no network).
#[derive(Clone, Default)]
pub struct MultiSeatSimulator {
    inner: Arc<Mutex<SimInner>>,
}

#[derive(Default)]
struct SimInner {
    /// Global log of all messages
    log: Vec<FairplayEnvelope>,
    /// Per-seat inbox
    inboxes: std::collections::BTreeMap<String, VecDeque<FairplayEnvelope>>,
}

impl MultiSeatSimulator {
    pub fn new(seats: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let mut inboxes = std::collections::BTreeMap::new();
        for s in seats {
            inboxes.insert(s.into(), VecDeque::new());
        }
        Self {
            inner: Arc::new(Mutex::new(SimInner {
                log: Vec::new(),
                inboxes,
            })),
        }
    }

    pub fn broadcast(&self, envelope: FairplayEnvelope) {
        let mut g = self.inner.lock().unwrap();
        g.log.push(envelope.clone());
        let from = envelope.from_seat.clone();
        for (seat, q) in g.inboxes.iter_mut() {
            if *seat != from {
                q.push_back(envelope.clone());
            }
        }
    }

    pub fn send_to(&self, to_seat: &str, envelope: FairplayEnvelope) {
        let mut g = self.inner.lock().unwrap();
        g.log.push(envelope.clone());
        if let Some(q) = g.inboxes.get_mut(to_seat) {
            q.push_back(envelope);
        }
    }

    pub fn recv(&self, seat: &str) -> Option<FairplayEnvelope> {
        let mut g = self.inner.lock().unwrap();
        g.inboxes.get_mut(seat).and_then(|q| q.pop_front())
    }

    pub fn log(&self) -> Vec<FairplayEnvelope> {
        self.inner.lock().unwrap().log.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commitment::commit_die_face;
    use crate::keychain::Keychain;
    use crate::secp::generate_keypair;
    use crate::sra::{encrypt_layer, encrypt_payload, peel_layer, peel_to_payload, PayloadLookup};
    use crate::wire::WireMessage;

    #[test]
    fn multi_seat_mental_poker_scenario() {
        let sim = MultiSeatSimulator::new(["0", "1"]);
        let mut kc = Keychain::empty_strict();
        let a = generate_keypair();
        let b = generate_keypair();
        kc.admit("0", &a.public_key_hex).unwrap();
        kc.admit("1", &b.public_key_hex).unwrap();

        // Seat 0 publishes pk
        sim.broadcast(FairplayEnvelope {
            from_seat: "0".into(),
            message: WireMessage::PublicKey {
                owner_id: "0".into(),
                public_key: a.public_key_hex.clone(),
            },
        });
        let msg = sim.recv("1").unwrap();
        match msg.message {
            WireMessage::PublicKey { public_key, .. } => {
                assert_eq!(public_key, a.public_key_hex);
            }
            _ => panic!("expected pk"),
        }

        let lookup = PayloadLookup::build(["d1", "d2", "d3"]).unwrap();
        let mut card = encrypt_payload("d2", &a.private_key_hex).unwrap();
        card = encrypt_layer(&card, &b.private_key_hex).unwrap();
        sim.broadcast(FairplayEnvelope {
            from_seat: "0".into(),
            message: WireMessage::Ciphertext { card: card.clone() },
        });
        let env = sim.recv("1").unwrap();
        let WireMessage::Ciphertext { card: c2 } = env.message else {
            panic!();
        };
        let after_b = peel_layer(&c2, &b.private_key_hex).unwrap();
        let id = peel_to_payload(&after_b, &a.private_key_hex, &lookup).unwrap();
        assert_eq!(id, "d2");

        // Wire log has no private keys
        for e in sim.log() {
            e.to_json().unwrap();
        }
    }

    #[test]
    fn multi_seat_dice_commit_reveal() {
        let sim = MultiSeatSimulator::new(["alice", "bob"]);
        let c = commit_die_face(5);
        sim.broadcast(FairplayEnvelope {
            from_seat: "alice".into(),
            message: WireMessage::Commitment {
                commitment_hex: c.commitment_hex.clone(),
                label: "hand".into(),
            },
        });
        let got = sim.recv("bob").unwrap();
        match got.message {
            WireMessage::Commitment { commitment_hex, .. } => {
                assert_eq!(commitment_hex, c.commitment_hex);
            }
            _ => panic!(),
        }
        sim.broadcast(FairplayEnvelope {
            from_seat: "alice".into(),
            message: WireMessage::CommitmentOpen {
                commitment_hex: c.commitment_hex.clone(),
                message_hex: c.message_hex.clone(),
                nonce_hex: c.nonce_hex.clone(),
                label: "hand".into(),
            },
        });
        let open = sim.recv("bob").unwrap();
        open.to_json().unwrap();
    }
}
