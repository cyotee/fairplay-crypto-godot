//! GDExtension entry for ManaMesh FairPlay.
//! Thin bindings — crypto math lives in `manamesh_fairplay_core`.

use godot::prelude::*;
use manamesh_fairplay_core as core;

struct ManaMeshFairPlay;

#[gdextension]
unsafe impl ExtensionLibrary for ManaMeshFairPlay {}

/// Autoload-friendly API for fair-play crypto (GDScript-callable).
#[derive(GodotClass)]
#[class(base=RefCounted)]
struct FairPlayApi {
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for FairPlayApi {
    fn init(base: Base<RefCounted>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl FairPlayApi {
    /// SHA-256 hex of UTF-8 string.
    #[func]
    fn sha256_hex(text: GString) -> GString {
        GString::from(core::sha256_hex(text.to_string().as_bytes()).as_str())
    }

    /// Generate keypair: returns Dictionary { public_key, private_key } — private stays local.
    #[func]
    fn generate_keypair() -> Dictionary {
        let kp = core::generate_keypair();
        let mut d = Dictionary::new();
        d.set("public_key", GString::from(kp.public_key_hex.as_str()));
        d.set("private_key", GString::from(kp.private_key_hex.as_str()));
        d
    }

    /// Validate compressed public key hex.
    #[func]
    fn validate_public_key(public_key_hex: GString) -> bool {
        core::validate_public_key(&public_key_hex.to_string())
    }

    /// Commit to bytes given as PackedByteArray. Returns { commitment_hex, nonce_hex, message_hex }.
    #[func]
    fn commit_bytes(message: PackedByteArray) -> Dictionary {
        let bytes: Vec<u8> = message.to_vec();
        let c = core::commit(&bytes);
        commitment_to_dict(&c)
    }

    /// Commit to a die face (1..=255). Dice-shaped Liar's Dice helper.
    #[func]
    fn commit_die_face(face: i32) -> Dictionary {
        let c = core::commitment::commit_die_face(face as u8);
        commitment_to_dict(&c)
    }

    /// Commit to multiple die faces (PackedByteArray of faces).
    #[func]
    fn commit_dice_hand(faces: PackedByteArray) -> Dictionary {
        let bytes: Vec<u8> = faces.to_vec();
        let c = core::commit(&bytes);
        commitment_to_dict(&c)
    }

    /// Verify commitment open. Returns true if valid.
    #[func]
    fn verify_commitment(
        commitment_hex: GString,
        message: PackedByteArray,
        nonce_hex: GString,
    ) -> bool {
        let msg: Vec<u8> = message.to_vec();
        let Ok(nonce) = core::hexutil::from_hex(&nonce_hex.to_string()) else {
            return false;
        };
        core::verify_commitment(&commitment_hex.to_string(), &msg, &nonce)
    }

    /// Verify die face open.
    #[func]
    fn verify_die_face(commitment_hex: GString, face: i32, nonce_hex: GString) -> bool {
        core::commitment::verify_die_face(
            &commitment_hex.to_string(),
            face as u8,
            &nonce_hex.to_string(),
        )
        .unwrap_or(false)
    }

    /// Encrypt payload id under private key → { ciphertext, layers } or empty on error.
    #[func]
    fn encrypt_payload(payload_id: GString, private_key_hex: GString) -> Dictionary {
        match core::encrypt_payload(&payload_id.to_string(), &private_key_hex.to_string()) {
            Ok(c) => layered_to_dict(&c),
            Err(_) => Dictionary::new(),
        }
    }

    /// Add encryption layer.
    #[func]
    fn encrypt_layer(ciphertext: GString, layers: i32, private_key_hex: GString) -> Dictionary {
        let card = core::LayeredCiphertext {
            ciphertext: ciphertext.to_string(),
            layers: layers as u32,
        };
        match core::encrypt_layer(&card, &private_key_hex.to_string()) {
            Ok(c) => layered_to_dict(&c),
            Err(_) => Dictionary::new(),
        }
    }

    /// Peel one layer.
    #[func]
    fn peel_layer(ciphertext: GString, layers: i32, private_key_hex: GString) -> Dictionary {
        let card = core::LayeredCiphertext {
            ciphertext: ciphertext.to_string(),
            layers: layers as u32,
        };
        match core::peel_layer(&card, &private_key_hex.to_string()) {
            Ok(c) => layered_to_dict(&c),
            Err(_) => Dictionary::new(),
        }
    }

    /// Merkle root hex of UTF-8 leaves (PackedStringArray).
    #[func]
    fn merkle_root_utf8(leaves: PackedStringArray) -> GString {
        let leaf_bytes: Vec<Vec<u8>> = leaves
            .to_vec()
            .into_iter()
            .map(|s| core::merkle::leaf_from_utf8(&s.to_string()))
            .collect();
        GString::from(core::merkle::merkle_root_hex(&leaf_bytes).as_str())
    }

    /// Library version string.
    #[func]
    fn version() -> GString {
        GString::from("0.1.0")
    }
}

fn commitment_to_dict(c: &core::Commitment) -> Dictionary {
    let mut d = Dictionary::new();
    d.set("commitment_hex", GString::from(c.commitment_hex.as_str()));
    d.set("nonce_hex", GString::from(c.nonce_hex.as_str()));
    d.set("message_hex", GString::from(c.message_hex.as_str()));
    d
}

fn layered_to_dict(c: &core::LayeredCiphertext) -> Dictionary {
    let mut d = Dictionary::new();
    d.set("ciphertext", GString::from(c.ciphertext.as_str()));
    d.set("layers", c.layers as i64);
    d
}
