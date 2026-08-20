//! OWP v0.1 protocol types: deterministic CBOR actions, identifiers, and the
//! `asset.transfer` reference profile with consumable state references.
//!
//! Normative source: docs/spec/owp-protocol-v0.1.md

pub mod cbor;

use cbor::{CborError, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

// ---------- Domain separation (spec §3.2) ----------

pub const DOMAIN_ACTION_SIGN: &[u8] = b"OWP-ACTION-v1\0";
pub const DOMAIN_ACTION_ID: &[u8] = b"OWP-ACTION-ID-v1\0";
pub const DOMAIN_OBJECT_ID: &[u8] = b"OWP-OBJECT-ID-v1\0";
pub const DOMAIN_VOTE: &[u8] = b"OWP-VOTE-v1\0";
pub const DOMAIN_CHECKPOINT: &[u8] = b"OWP-CHECKPOINT-v1\0";

pub const ACTION_VERSION: u64 = 1;

// ---------- Consensus levels (spec §5.2 key 6) ----------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsensusLevel {
    None = 0,
    Local = 1,
    Community = 2,
    Global = 3,
}

impl ConsensusLevel {
    pub fn from_u64(v: u64) -> Option<Self> {
        match v {
            0 => Some(Self::None),
            1 => Some(Self::Local),
            2 => Some(Self::Community),
            3 => Some(Self::Global),
            _ => None,
        }
    }
}

// ---------- Errors ----------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolError {
    Cbor(CborError),
    Schema(&'static str),
    NonCanonical(&'static str),
    InvalidType(&'static str),
}

impl From<CborError> for ProtocolError {
    fn from(e: CborError) -> Self {
        ProtocolError::Cbor(e)
    }
}

// ---------- UnsignedAction (spec §5.2) ----------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsignedAction {
    pub version: u64,
    pub actor: [u8; 32],
    pub action_type: String,
    pub nonce: u64,
    pub created_at: u64,
    /// MUST be unique and sorted ascending bytewise (spec §5.6). Enforced by `validate_shape`.
    pub refs: Vec<[u8; 32]>,
    pub consensus: ConsensusLevel,
    pub scope: Option<[u8; 32]>,
    pub payload: BTreeMap<String, Value>,
}

impl UnsignedAction {
    /// Structural validation independent of world state (spec §5.5, §5.6).
    pub fn validate_shape(&self) -> Result<(), ProtocolError> {
        if self.version != ACTION_VERSION {
            return Err(ProtocolError::Schema("unsupported action version"));
        }
        validate_action_type(&self.action_type)?;
        for w in self.refs.windows(2) {
            if w[0] >= w[1] {
                return Err(ProtocolError::NonCanonical("refs not strictly ascending"));
            }
        }
        match (self.consensus, &self.scope) {
            (ConsensusLevel::None | ConsensusLevel::Local, Some(_)) => {
                return Err(ProtocolError::Schema("scope must be null for NONE/LOCAL"))
            }
            (ConsensusLevel::Community | ConsensusLevel::Global, None) => {
                return Err(ProtocolError::Schema("scope required for COMMUNITY/GLOBAL"))
            }
            _ => {}
        }
        Ok(())
    }

    /// Deterministic CBOR map with integer keys 0..=8 (spec §5.2).
    pub fn to_cbor_value(&self) -> Value {
        // Integer-keyed map: encoded directly as fixed-order entries (keys 0..8 ascending),
        // which is exactly the canonical order for uint keys.
        // We build it manually because cbor::Value maps are text-keyed by design (§5.4 payloads).
        // Encoding handled by `canonical_bytes`.
        unreachable!("use canonical_bytes()");
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(256);
        // map(9) with uint keys 0..=8 in ascending order = canonical.
        out.push(0xA9);
        push_uint_key(&mut out, 0);
        cbor::encode(&Value::Uint(self.version), &mut out);
        push_uint_key(&mut out, 1);
        cbor::encode(&Value::Bytes(self.actor.to_vec()), &mut out);
        push_uint_key(&mut out, 2);
        cbor::encode(&Value::Text(self.action_type.clone()), &mut out);
        push_uint_key(&mut out, 3);
        cbor::encode(&Value::Uint(self.nonce), &mut out);
        push_uint_key(&mut out, 4);
        cbor::encode(&Value::Uint(self.created_at), &mut out);
        push_uint_key(&mut out, 5);
        let refs = Value::Array(self.refs.iter().map(|r| Value::Bytes(r.to_vec())).collect());
        cbor::encode(&refs, &mut out);
        push_uint_key(&mut out, 6);
        cbor::encode(&Value::Uint(self.consensus as u64), &mut out);
        push_uint_key(&mut out, 7);
        match &self.scope {
            Some(s) => cbor::encode(&Value::Bytes(s.to_vec()), &mut out),
            None => cbor::encode(&Value::Null, &mut out),
        }
        push_uint_key(&mut out, 8);
        cbor::encode(&Value::Map(self.payload.clone()), &mut out);
        out
    }

    /// signing_message = "OWP-ACTION-v1\0" || DeterministicCBOR(UnsignedAction)   (spec §7.1)
    pub fn signing_message(&self) -> Vec<u8> {
        let mut m = Vec::with_capacity(DOMAIN_ACTION_SIGN.len() + 256);
        m.extend_from_slice(DOMAIN_ACTION_SIGN);
        m.extend_from_slice(&self.canonical_bytes());
        m
    }

    /// action_id = SHA-256("OWP-ACTION-ID-v1\0" || DeterministicCBOR(UnsignedAction))   (spec §7.2)
    pub fn action_id(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(DOMAIN_ACTION_ID);
        h.update(self.canonical_bytes());
        h.finalize().into()
    }
}

fn push_uint_key(out: &mut Vec<u8>, k: u64) {
    cbor::encode(&Value::Uint(k), out);
}

pub fn validate_action_type(t: &str) -> Result<(), ProtocolError> {
    let bytes = t.as_bytes();
    if bytes.is_empty() || bytes.len() > 96 {
        return Err(ProtocolError::InvalidType("type length out of range"));
    }
    if !bytes[0].is_ascii_lowercase() {
        return Err(ProtocolError::InvalidType("type must start with lowercase letter"));
    }
    if !bytes.iter().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'-' | b'_' | b'.')) {
        return Err(ProtocolError::InvalidType("type contains forbidden character"));
    }
    Ok(())
}

// ---------- SignedAction (spec §5.3) ----------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedAction {
    pub unsigned: UnsignedAction,
    pub signature: [u8; 64],
}

impl SignedAction {
    /// { 0: UnsignedAction, 1: signature } as deterministic CBOR.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(384);
        out.push(0xA2); // map(2)
        push_uint_key(&mut out, 0);
        out.extend_from_slice(&self.unsigned.canonical_bytes());
        push_uint_key(&mut out, 1);
        cbor::encode(&Value::Bytes(self.signature.to_vec()), &mut out);
        out
    }

    /// Strict decode from wire bytes. Enforces spec §7.3: the received CBOR MUST be
    /// the deterministic encoding of the decoded action (guaranteed by re-encoding check).
    pub fn from_wire(bytes: &[u8]) -> Result<Self, ProtocolError> {
        let action = decode_signed(bytes)?;
        // Canonical-form guarantee: re-encode and compare byte-for-byte.
        if action.canonical_bytes() != bytes {
            return Err(ProtocolError::NonCanonical("wire bytes are not deterministic encoding"));
        }
        action.unsigned.validate_shape()?;
        Ok(action)
    }
}

fn decode_signed(bytes: &[u8]) -> Result<SignedAction, ProtocolError> {
    // Parse the outer map(2) {0: map(9), 1: bstr(64)} using a permissive raw parse,
    // then rely on the re-encoding check for canonicality.
    // We reuse the strict cbor decoder on integer-keyed maps by hand-parsing heads.
    let mut pos = 0usize;
    let head = *bytes.get(pos).ok_or(ProtocolError::Schema("empty input"))?;
    pos += 1;
    if head != 0xA2 {
        return Err(ProtocolError::Schema("expected map(2) SignedAction"));
    }
    expect_uint_key(bytes, &mut pos, 0)?;
    let unsigned = decode_unsigned(bytes, &mut pos)?;
    expect_uint_key(bytes, &mut pos, 1)?;
    let sig_v = decode_one(bytes, &mut pos)?;
    let Value::Bytes(sig) = sig_v else {
        return Err(ProtocolError::Schema("signature must be bstr"));
    };
    let signature: [u8; 64] = sig.try_into().map_err(|_| ProtocolError::Schema("signature must be 64 bytes"))?;
    if pos != bytes.len() {
        return Err(ProtocolError::Schema("trailing bytes"));
    }
    Ok(SignedAction { unsigned, signature })
}

fn decode_unsigned(bytes: &[u8], pos: &mut usize) -> Result<UnsignedAction, ProtocolError> {
    let head = *bytes.get(*pos).ok_or(ProtocolError::Schema("eof"))?;
    *pos += 1;
    if head != 0xA9 {
        return Err(ProtocolError::Schema("expected map(9) UnsignedAction; unknown keys are rejected"));
    }
    expect_uint_key(bytes, pos, 0)?;
    let version = expect_uint(decode_one(bytes, pos)?)?;
    expect_uint_key(bytes, pos, 1)?;
    let actor = expect_bstr32(decode_one(bytes, pos)?, "actor")?;
    expect_uint_key(bytes, pos, 2)?;
    let Value::Text(action_type) = decode_one(bytes, pos)? else {
        return Err(ProtocolError::Schema("type must be tstr"));
    };
    expect_uint_key(bytes, pos, 3)?;
    let nonce = expect_uint(decode_one(bytes, pos)?)?;
    expect_uint_key(bytes, pos, 4)?;
    let created_at = expect_uint(decode_one(bytes, pos)?)?;
    expect_uint_key(bytes, pos, 5)?;
    let Value::Array(refs_v) = decode_one(bytes, pos)? else {
        return Err(ProtocolError::Schema("refs must be array"));
    };
    let mut refs = Vec::with_capacity(refs_v.len());
    for r in refs_v {
        refs.push(expect_bstr32(r, "ref")?);
    }
    expect_uint_key(bytes, pos, 6)?;
    let consensus = ConsensusLevel::from_u64(expect_uint(decode_one(bytes, pos)?)?)
        .ok_or(ProtocolError::Schema("invalid consensus level"))?;
    expect_uint_key(bytes, pos, 7)?;
    let scope = match decode_one(bytes, pos)? {
        Value::Null => None,
        v => Some(expect_bstr32(v, "scope")?),
    };
    expect_uint_key(bytes, pos, 8)?;
    let Value::Map(payload) = decode_one(bytes, pos)? else {
        return Err(ProtocolError::Schema("payload must be map"));
    };
    Ok(UnsignedAction { version, actor, action_type, nonce, created_at, refs, consensus, scope, payload })
}

fn decode_one(bytes: &[u8], pos: &mut usize) -> Result<Value, ProtocolError> {
    // Decode a single item by finding its length via strict sub-decode.
    let rest = &bytes[*pos..];
    // strict decoder needs exact-length input; walk incrementally.
    let len = item_len(rest).ok_or(ProtocolError::Schema("truncated item"))?;
    let v = cbor::from_bytes(&rest[..len])?;
    *pos += len;
    Ok(v)
}

fn expect_uint_key(bytes: &[u8], pos: &mut usize, k: u64) -> Result<(), ProtocolError> {
    match decode_one(bytes, pos)? {
        Value::Uint(got) if got == k => Ok(()),
        _ => Err(ProtocolError::Schema("unexpected map key")),
    }
}

fn expect_uint(v: Value) -> Result<u64, ProtocolError> {
    match v {
        Value::Uint(n) => Ok(n),
        _ => Err(ProtocolError::Schema("expected uint")),
    }
}

fn expect_bstr32(v: Value, what: &'static str) -> Result<[u8; 32], ProtocolError> {
    match v {
        Value::Bytes(b) => b.try_into().map_err(|_| ProtocolError::Schema(what)),
        _ => Err(ProtocolError::Schema(what)),
    }
}

/// Length in bytes of the first complete CBOR item in `data`, or None if truncated/invalid head.
fn item_len(data: &[u8]) -> Option<usize> {
    let ib = *data.first()?;
    let major = ib >> 5;
    let info = ib & 0x1F;
    let (arg, head): (u64, usize) = match info {
        0..=23 => (info as u64, 1),
        24 => (*data.get(1)? as u64, 2),
        25 => (u16::from_be_bytes(data.get(1..3)?.try_into().ok()?) as u64, 3),
        26 => (u32::from_be_bytes(data.get(1..5)?.try_into().ok()?) as u64, 5),
        27 => (u64::from_be_bytes(data.get(1..9)?.try_into().ok()?), 9),
        _ => return None,
    };
    match major {
        0 | 1 | 7 => Some(head),
        2 | 3 => Some(head + arg as usize),
        4 => {
            let mut len = head;
            for _ in 0..arg {
                len += item_len(&data[len..])?;
            }
            Some(len)
        }
        5 => {
            let mut len = head;
            for _ in 0..arg * 2 {
                len += item_len(&data[len..])?;
            }
            Some(len)
        }
        _ => None,
    }
}

// ---------- OwnedObject & state_ref (spec §8.2 / §11.1) ----------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedObject {
    pub object_id: [u8; 32],
    pub owner: [u8; 32],
    pub version: u64,
    pub data_hash: [u8; 32],
}

impl OwnedObject {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(128);
        out.push(0xA4); // map(4): keys 0..=3
        push_uint_key(&mut out, 0);
        cbor::encode(&Value::Bytes(self.object_id.to_vec()), &mut out);
        push_uint_key(&mut out, 1);
        cbor::encode(&Value::Bytes(self.owner.to_vec()), &mut out);
        push_uint_key(&mut out, 2);
        cbor::encode(&Value::Uint(self.version), &mut out);
        push_uint_key(&mut out, 3);
        cbor::encode(&Value::Bytes(self.data_hash.to_vec()), &mut out);
        out
    }

    /// state_ref = SHA-256("OWP-OBJECT-ID-v1\0" || DeterministicCBOR(OwnedObject))
    pub fn state_ref(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(DOMAIN_OBJECT_ID);
        h.update(self.canonical_bytes());
        h.finalize().into()
    }
}

// ---------- asset.transfer reference profile ----------

pub const ACTION_ASSET_TRANSFER: &str = "asset.transfer";

/// Builds the canonical payload for asset.transfer:
/// { "input": {"object_id": bstr, "state_ref": bstr, "version": uint}, "to": bstr(32) }
pub fn asset_transfer_payload(input: &OwnedObject, to: [u8; 32]) -> BTreeMap<String, Value> {
    let mut inp = BTreeMap::new();
    inp.insert("object_id".to_owned(), Value::Bytes(input.object_id.to_vec()));
    inp.insert("state_ref".to_owned(), Value::Bytes(input.state_ref().to_vec()));
    inp.insert("version".to_owned(), Value::Uint(input.version));
    let mut payload = BTreeMap::new();
    payload.insert("input".to_owned(), Value::Map(inp));
    payload.insert("to".to_owned(), Value::Bytes(to.to_vec()));
    payload
}

pub struct AssetTransfer {
    pub object_id: [u8; 32],
    pub state_ref: [u8; 32],
    pub version: u64,
    pub to: [u8; 32],
}

pub fn parse_asset_transfer(payload: &BTreeMap<String, Value>) -> Result<AssetTransfer, ProtocolError> {
    if payload.len() != 2 {
        return Err(ProtocolError::Schema("asset.transfer payload must have exactly input,to"));
    }
    let Some(Value::Map(inp)) = payload.get("input") else {
        return Err(ProtocolError::Schema("missing input map"));
    };
    if inp.len() != 3 {
        return Err(ProtocolError::Schema("input must have exactly object_id,state_ref,version"));
    }
    let object_id = match inp.get("object_id") {
        Some(Value::Bytes(b)) => b.clone().try_into().map_err(|_| ProtocolError::Schema("object_id"))?,
        _ => return Err(ProtocolError::Schema("object_id")),
    };
    let state_ref = match inp.get("state_ref") {
        Some(Value::Bytes(b)) => b.clone().try_into().map_err(|_| ProtocolError::Schema("state_ref"))?,
        _ => return Err(ProtocolError::Schema("state_ref")),
    };
    let version = match inp.get("version") {
        Some(Value::Uint(n)) => *n,
        _ => return Err(ProtocolError::Schema("version")),
    };
    let to = match payload.get("to") {
        Some(Value::Bytes(b)) => b.clone().try_into().map_err(|_| ProtocolError::Schema("to"))?,
        _ => return Err(ProtocolError::Schema("to")),
    };
    Ok(AssetTransfer { object_id, state_ref, version, to })
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::cbor::{from_bytes, to_bytes, CborError, Value};
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn cbor_uses_shortest_integer_forms() {
        assert_eq!(to_bytes(&Value::Uint(0)), vec![0x00]);
        assert_eq!(to_bytes(&Value::Uint(23)), vec![0x17]);
        assert_eq!(to_bytes(&Value::Uint(24)), vec![0x18, 0x18]);
        assert_eq!(to_bytes(&Value::Uint(255)), vec![0x18, 0xFF]);
        assert_eq!(to_bytes(&Value::Uint(256)), vec![0x19, 0x01, 0x00]);
        assert_eq!(to_bytes(&Value::Uint(u64::MAX))[0], 0x1B);
    }

    #[test]
    fn cbor_decoder_rejects_non_shortest_forms() {
        // 0 encoded as 0x18 0x00 (one-byte arg for a value < 24) is non-canonical.
        assert_eq!(from_bytes(&[0x18, 0x00]), Err(CborError::NonCanonical("argument not shortest form")));
        // 255 encoded with two-byte arg is non-canonical.
        assert_eq!(from_bytes(&[0x19, 0x00, 0xFF]), Err(CborError::NonCanonical("argument not shortest form")));
    }

    #[test]
    fn cbor_decoder_rejects_indefinite_and_floats_and_tags() {
        assert!(matches!(from_bytes(&[0x9F, 0xFF]), Err(CborError::ForbiddenItem(_)))); // indefinite array
        assert!(matches!(from_bytes(&[0xF9, 0x00, 0x00]), Err(CborError::ForbiddenItem(_)))); // float16
        assert!(matches!(from_bytes(&[0xC0, 0x60]), Err(CborError::ForbiddenItem(_)))); // tag 0
    }

    #[test]
    fn cbor_map_keys_are_canonically_ordered() {
        let mut m = BTreeMap::new();
        m.insert("zz".to_owned(), Value::Uint(1));
        m.insert("a".to_owned(), Value::Uint(2));
        m.insert("b".to_owned(), Value::Uint(3));
        let bytes = to_bytes(&Value::Map(m.clone()));
        // shorter key "a" first, then "b", then "zz" (length-first ordering via encoded bytes)
        let decoded = from_bytes(&bytes).unwrap();
        assert_eq!(decoded, Value::Map(m));
        // Hand-build the same map with swapped order on the wire -> must be rejected.
        let mut bad = vec![0xA3];
        for (k, v) in [("b", 3u64), ("a", 2), ("zz", 1)] {
            bad.extend(to_bytes(&Value::Text(k.into())));
            bad.extend(to_bytes(&Value::Uint(v)));
        }
        assert_eq!(from_bytes(&bad), Err(CborError::NonCanonical("map keys not in canonical order")));
    }

    #[test]
    fn cbor_roundtrip_is_byte_identical() {
        let mut payload = BTreeMap::new();
        payload.insert("text".to_owned(), Value::Text("perennia".into()));
        payload.insert("n".to_owned(), Value::Uint(42));
        payload.insert("neg".to_owned(), Value::Nint(9)); // -10
        payload.insert("flag".to_owned(), Value::Bool(true));
        payload.insert("none".to_owned(), Value::Null);
        payload.insert("arr".to_owned(), Value::Array(vec![Value::Uint(1), Value::Bytes(vec![0xAB; 3])]));
        let v = Value::Map(payload);
        let bytes = to_bytes(&v);
        let decoded = from_bytes(&bytes).unwrap();
        assert_eq!(decoded, v);
        assert_eq!(to_bytes(&decoded), bytes);
    }

    #[test]
    fn action_type_syntax_is_enforced() {
        assert!(validate_action_type("asset.transfer").is_ok());
        assert!(validate_action_type("agent.delegate").is_ok());
        assert!(validate_action_type("Asset.transfer").is_err());
        assert!(validate_action_type("1bad").is_err());
        assert!(validate_action_type("").is_err());
        assert!(validate_action_type(&"a".repeat(97)).is_err());
    }

    #[test]
    fn refs_must_be_strictly_ascending() {
        let mut u = UnsignedAction {
            version: ACTION_VERSION,
            actor: [7; 32],
            action_type: "asset.transfer".into(),
            nonce: 1,
            created_at: 0,
            refs: vec![[2; 32], [1; 32]],
            consensus: ConsensusLevel::None,
            scope: None,
            payload: BTreeMap::new(),
        };
        assert!(u.validate_shape().is_err());
        u.refs = vec![[1; 32], [1; 32]];
        assert!(u.validate_shape().is_err());
        u.refs = vec![[1; 32], [2; 32]];
        assert!(u.validate_shape().is_ok());
    }

    #[test]
    fn scope_consensus_pairing_is_enforced() {
        let base = UnsignedAction {
            version: ACTION_VERSION,
            actor: [7; 32],
            action_type: "asset.transfer".into(),
            nonce: 1,
            created_at: 0,
            refs: vec![],
            consensus: ConsensusLevel::None,
            scope: Some([1; 32]),
            payload: BTreeMap::new(),
        };
        assert!(base.validate_shape().is_err()); // NONE + scope
        let mut community = base.clone();
        community.consensus = ConsensusLevel::Community;
        community.scope = None;
        assert!(community.validate_shape().is_err()); // COMMUNITY sin scope
        community.scope = Some([1; 32]);
        assert!(community.validate_shape().is_ok());
    }
}
