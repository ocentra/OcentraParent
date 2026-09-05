use num_bigint::BigUint;

const P256_FIELD_MODULUS: [u8; 32] = [
    0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

const P256_CURVE_B: [u8; 32] = [
    0x5a, 0xc6, 0x35, 0xd8, 0xaa, 0x3a, 0x93, 0xe7, 0xb3, 0xeb, 0xbd, 0x55, 0x76, 0x98, 0x86, 0xbc,
    0x65, 0x1d, 0x06, 0xb0, 0xcc, 0x53, 0xb0, 0xf6, 0x3b, 0xce, 0x3c, 0x3e, 0x27, 0xd2, 0x60, 0x4b,
];

/// Proof that a canonical SEC1 uncompressed value is a finite point on P-256.
/// Coordinates stay private because parsing a key is not signing authority.
pub(super) struct ParsedP256PublicKey;

pub(super) fn parse_uncompressed_p256(public_key: &[u8; 65]) -> Option<ParsedP256PublicKey> {
    if public_key[0] != 0x04 {
        return None;
    }

    let modulus = BigUint::from_bytes_be(&P256_FIELD_MODULUS);
    let x = BigUint::from_bytes_be(&public_key[1..33]);
    let y = BigUint::from_bytes_be(&public_key[33..65]);
    if x >= modulus || y >= modulus {
        return None;
    }

    let curve_b = BigUint::from_bytes_be(&P256_CURVE_B);
    let x_squared = (&x * &x) % &modulus;
    let x_cubed = (x_squared * &x) % &modulus;
    let three_x = (BigUint::from(3_u8) * &x) % &modulus;
    let minus_three_x = if three_x == BigUint::from(0_u8) {
        BigUint::from(0_u8)
    } else {
        &modulus - three_x
    };
    let expected_y_squared = (x_cubed + minus_three_x + curve_b) % &modulus;
    let actual_y_squared = (&y * &y) % &modulus;

    (actual_y_squared == expected_y_squared).then_some(ParsedP256PublicKey)
}
