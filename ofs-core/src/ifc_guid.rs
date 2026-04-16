//! IFC GlobalId conversion (RFC 4122 UUID <-> IFC base64 22-char format)

/// Convert a UUID to IFC GlobalId format (22 chars, base64-like encoding per ISO 10303-21)
pub fn uuid_to_ifc_guid(uuid: &uuid::Uuid) -> String {
    let bytes = uuid.as_bytes();
    let mut result = String::with_capacity(22);
    let chars: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$";

    // Encode 16 bytes (128 bits) into 22 base-64 chars
    let mut num: u128 = 0;
    for &b in bytes {
        num = (num << 8) | b as u128;
    }

    for _i in (0..22).rev() {
        let idx = (num & 63) as usize;
        result.insert(0, chars[idx] as char);
        num >>= 6;
    }

    result
}

/// Convert an IFC GlobalId back to UUID
pub fn ifc_guid_to_uuid(guid: &str) -> Option<uuid::Uuid> {
    if guid.len() != 22 {
        return None;
    }
    let chars: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$";

    let mut num: u128 = 0;
    for ch in guid.bytes() {
        let idx = chars.iter().position(|&c| c == ch)?;
        num = (num << 6) | idx as u128;
    }

    let mut bytes = [0u8; 16];
    for i in (0..16).rev() {
        bytes[i] = (num & 0xFF) as u8;
        num >>= 8;
    }

    Some(uuid::Uuid::from_bytes(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let uuid = uuid::Uuid::new_v4();
        let guid = uuid_to_ifc_guid(&uuid);
        assert_eq!(guid.len(), 22);
        let back = ifc_guid_to_uuid(&guid).expect("Should parse back");
        assert_eq!(back, uuid);
    }

    #[test]
    fn rejects_invalid_length() {
        assert!(ifc_guid_to_uuid("too_short").is_none());
        assert!(ifc_guid_to_uuid("this_is_way_too_long_for_ifc").is_none());
    }
}
