//! Deterministic CBOR (RFC 8949 §4.2.1 core deterministic encoding) for OWP v0.1.
//!
//! Scope is intentionally restricted to the value set allowed by the OWP spec §5.4:
//! unsigned/negative integers, text, bytes, booleans, null, arrays, and text-keyed maps.
//! Floats, tags, and indefinite-length items are rejected by construction.

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Uint(u64),
    /// Negative integer n encoded as CBOR major type 1 with argument (-1 - n), i.e. value = -1 - arg.
    /// `Nint(x)` represents the integer -1 - x. Example: Nint(0) == -1, Nint(9) == -10.
    Nint(u64),
    Bytes(Vec<u8>),
    Text(String),
    Array(Vec<Value>),
    /// Map with UTF-8 text keys only (spec §5.4). Determinism: keys sorted by their encoded bytes.
    Map(BTreeMap<String, Value>),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CborError {
    UnexpectedEof,
    TrailingBytes,
    ForbiddenItem(&'static str),
    NonCanonical(&'static str),
    InvalidUtf8,
    DepthExceeded,
}

const MAX_DEPTH: usize = 32;

// ---------- Encoding ----------

fn write_type_arg(out: &mut Vec<u8>, major: u8, arg: u64) {
    let mt = major << 5;
    if arg < 24 {
        out.push(mt | arg as u8);
    } else if arg <= 0xFF {
        out.push(mt | 24);
        out.push(arg as u8);
    } else if arg <= 0xFFFF {
        out.push(mt | 25);
        out.extend_from_slice(&(arg as u16).to_be_bytes());
    } else if arg <= 0xFFFF_FFFF {
        out.push(mt | 26);
        out.extend_from_slice(&(arg as u32).to_be_bytes());
    } else {
        out.push(mt | 27);
        out.extend_from_slice(&arg.to_be_bytes());
    }
}

pub fn encode(value: &Value, out: &mut Vec<u8>) {
    match value {
        Value::Uint(n) => write_type_arg(out, 0, *n),
        Value::Nint(n) => write_type_arg(out, 1, *n),
        Value::Bytes(b) => {
            write_type_arg(out, 2, b.len() as u64);
            out.extend_from_slice(b);
        }
        Value::Text(t) => {
            write_type_arg(out, 3, t.len() as u64);
            out.extend_from_slice(t.as_bytes());
        }
        Value::Array(items) => {
            write_type_arg(out, 4, items.len() as u64);
            for item in items {
                encode(item, out);
            }
        }
        Value::Map(map) => {
            // Deterministic key order: bytewise lexicographic order of the *encoded* key.
            // For text keys this equals (length, then bytes) because the CBOR head embeds length.
            let mut entries: Vec<(Vec<u8>, &Value)> = map
                .iter()
                .map(|(k, v)| {
                    let mut kb = Vec::with_capacity(k.len() + 2);
                    encode(&Value::Text(k.clone()), &mut kb);
                    (kb, v)
                })
                .collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            write_type_arg(out, 5, entries.len() as u64);
            for (kb, v) in entries {
                out.extend_from_slice(&kb);
                encode(v, out);
            }
        }
        Value::Bool(b) => out.push(if *b { 0xF5 } else { 0xF4 }),
        Value::Null => out.push(0xF6),
    }
}

pub fn to_bytes(value: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    encode(value, &mut out);
    out
}

// ---------- Decoding (strict: rejects non-canonical input) ----------

struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn byte(&mut self) -> Result<u8, CborError> {
        let b = *self.data.get(self.pos).ok_or(CborError::UnexpectedEof)?;
        self.pos += 1;
        Ok(b)
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], CborError> {
        if self.pos + n > self.data.len() {
            return Err(CborError::UnexpectedEof);
        }
        let s = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    fn type_arg(&mut self) -> Result<(u8, u64), CborError> {
        let ib = self.byte()?;
        let major = ib >> 5;
        let info = ib & 0x1F;
        // Major type 7: info 24 = extended simple value, 25/26/27 = float16/32/64.
        // None of these are integer arguments; all are forbidden in OWP v0.1.
        if major == 7 && (24..=27).contains(&info) {
            return Err(CborError::ForbiddenItem("floating point or extended simple value"));
        }
        let arg = match info {
            0..=23 => info as u64,
            24 => {
                let v = self.byte()? as u64;
                if v < 24 {
                    return Err(CborError::NonCanonical("argument not shortest form"));
                }
                v
            }
            25 => {
                let v = u16::from_be_bytes(self.take(2)?.try_into().unwrap()) as u64;
                if v <= 0xFF {
                    return Err(CborError::NonCanonical("argument not shortest form"));
                }
                v
            }
            26 => {
                let v = u32::from_be_bytes(self.take(4)?.try_into().unwrap()) as u64;
                if v <= 0xFFFF {
                    return Err(CborError::NonCanonical("argument not shortest form"));
                }
                v
            }
            27 => {
                let v = u64::from_be_bytes(self.take(8)?.try_into().unwrap());
                if v <= 0xFFFF_FFFF {
                    return Err(CborError::NonCanonical("argument not shortest form"));
                }
                v
            }
            31 => return Err(CborError::ForbiddenItem("indefinite-length item")),
            _ => return Err(CborError::ForbiddenItem("reserved additional info")),
        };
        Ok((major, arg))
    }

    fn value(&mut self, depth: usize) -> Result<Value, CborError> {
        if depth > MAX_DEPTH {
            return Err(CborError::DepthExceeded);
        }
        let (major, arg) = self.type_arg()?;
        match major {
            0 => Ok(Value::Uint(arg)),
            1 => Ok(Value::Nint(arg)),
            2 => Ok(Value::Bytes(self.take(arg as usize)?.to_vec())),
            3 => {
                let raw = self.take(arg as usize)?;
                let s = std::str::from_utf8(raw).map_err(|_| CborError::InvalidUtf8)?;
                Ok(Value::Text(s.to_owned()))
            }
            4 => {
                let mut items = Vec::with_capacity(arg as usize);
                for _ in 0..arg {
                    items.push(self.value(depth + 1)?);
                }
                Ok(Value::Array(items))
            }
            5 => {
                let mut map = BTreeMap::new();
                let mut prev_key_enc: Option<Vec<u8>> = None;
                for _ in 0..arg {
                    let key = self.value(depth + 1)?;
                    let Value::Text(k) = key else {
                        return Err(CborError::ForbiddenItem("non-text map key"));
                    };
                    let kb = to_bytes(&Value::Text(k.clone()));
                    if let Some(prev) = &prev_key_enc {
                        if kb <= *prev {
                            return Err(CborError::NonCanonical("map keys not in canonical order"));
                        }
                    }
                    prev_key_enc = Some(kb);
                    let v = self.value(depth + 1)?;
                    map.insert(k, v);
                }
                Ok(Value::Map(map))
            }
            7 => match arg {
                20 => Ok(Value::Bool(false)),
                21 => Ok(Value::Bool(true)),
                22 => Ok(Value::Null),
                _ => Err(CborError::ForbiddenItem("floating point, tag, or simple value")),
            },
            6 => Err(CborError::ForbiddenItem("cbor tag")),
            _ => unreachable!(),
        }
    }
}

/// Strict decode: input MUST be the deterministic encoding of the returned value,
/// with no trailing bytes. Guarantees decode(bytes) -> v implies to_bytes(v) == bytes.
pub fn from_bytes(data: &[u8]) -> Result<Value, CborError> {
    let mut r = Reader { data, pos: 0 };
    let v = r.value(0)?;
    if r.pos != data.len() {
        return Err(CborError::TrailingBytes);
    }
    Ok(v)
}
