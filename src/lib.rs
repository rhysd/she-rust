use std::os::raw::c_int;

#[link(name = "mcl", kind = "static")]
#[link(name = "mclshe384_256", kind = "static")]
#[link(name = "gmp")]
#[link(name = "stdc++")]
#[link(name = "crypto")]
#[allow(non_snake_case)]
extern "C" {
    // global functions
    fn sheInit(curve: c_int, compiledTimeVar: c_int) -> c_int;
    fn sheInitG1only(curve: c_int, compiledTimeVar: c_int) -> c_int;
    fn sheSetTryNum(tryNum: usize);
    fn sheSecretKeySetByCSPRNG(sec: *mut SecretKey) -> c_int;
    fn sheGetPublicKey(pubkey: *mut PublicKey, sec: *const SecretKey);
    fn sheEncG1(c: *mut CipherTextG1, pubkey: *const PublicKey, m: i64) -> c_int;
    fn sheEncG2(c: *mut CipherTextG2, pubkey: *const PublicKey, m: i64) -> c_int;
    fn sheEncGT(c: *mut CipherTextGT, pubkey: *const PublicKey, m: i64) -> c_int;
    fn sheDecG1(m: *mut i64, sec: *const SecretKey, c: *const CipherTextG1) -> c_int;
    fn sheDecG2(m: *mut i64, sec: *const SecretKey, c: *const CipherTextG2) -> c_int;
    fn sheDecGT(m: *mut i64, sec: *const SecretKey, c: *const CipherTextGT) -> c_int;
    fn sheAddG1(c: *mut CipherTextG1, x: *const CipherTextG1, y: *const CipherTextG1) -> c_int;
    fn sheAddG2(c: *mut CipherTextG2, x: *const CipherTextG2, y: *const CipherTextG2) -> c_int;
    fn sheAddGT(c: *mut CipherTextGT, x: *const CipherTextGT, y: *const CipherTextGT) -> c_int;
    fn sheSubG1(c: *mut CipherTextG1, x: *const CipherTextG1, y: *const CipherTextG1) -> c_int;
    fn sheSubG2(c: *mut CipherTextG2, x: *const CipherTextG2, y: *const CipherTextG2) -> c_int;
    fn sheSubGT(c: *mut CipherTextGT, x: *const CipherTextGT, y: *const CipherTextGT) -> c_int;
    fn sheMulG1(c: *mut CipherTextG1, x: *const CipherTextG1, y: i64) -> c_int;
    fn sheMulG2(c: *mut CipherTextG2, x: *const CipherTextG2, y: i64) -> c_int;
    fn sheMulGT(c: *mut CipherTextGT, x: *const CipherTextGT, y: i64) -> c_int;
    fn sheMul(c: *mut CipherTextGT, x: *const CipherTextG1, y: *const CipherTextG2) -> c_int;
    fn sheNegG1(c: *mut CipherTextG1, x: *const CipherTextG1) -> c_int;
    fn sheNegG2(c: *mut CipherTextG2, x: *const CipherTextG2) -> c_int;
    fn sheNegGT(c: *mut CipherTextGT, x: *const CipherTextGT) -> c_int;
    fn sheSetRangeForDLP(hashSize: usize) -> c_int;
    fn sheSetRangeForG1DLP(hashSize: usize) -> c_int;
    fn sheSetRangeForG2DLP(hashSize: usize) -> c_int;
    fn sheSetRangeForGTDLP(hashSize: usize) -> c_int;
}

#[allow(non_camel_case_types)]
pub enum CurveType {
    BN254 = 0,
    BN381 = 1,
    SNARK = 4,
    BLS12_381 = 5,
    SECP192K1 = 100,
    SECP224K1 = 101,
    SECP256K1 = 102,
    NIST_P192 = 105,
    NIST_P224 = 106,
    NIST_P256 = 107,
}
#[derive(Debug, PartialEq, Clone)]
/// `SheError` type for error
pub enum SheError {
    /// can't decrypt
    CantDecrypt,
    /// internal error
    InternalError,
}

const MCLBN_FP_UNIT_SIZE: usize = 6;
const MCLBN_FR_UNIT_SIZE: usize = 4;
/*
const FR_SIZE: usize = MCLBN_FR_UNIT_SIZE;
const G1_SIZE: usize = MCLBN_FP_UNIT_SIZE * 3;
const G2_SIZE: usize = MCLBN_FP_UNIT_SIZE * 6;
const GT_SIZE: usize = MCLBN_FP_UNIT_SIZE * 12;

const SEC_SIZE: usize = FR_SIZE * 2;
const PUB_SIZE: usize = G1_SIZE + G2_SIZE;
const G1_CIPHER_SIZE: usize = G1_SIZE * 2;
const G2_CIPHER_SIZE: usize = G2_SIZE * 2;
const GT_CIPHER_SIZE: usize = GT_SIZE * 4;
*/
const MCLBN_COMPILED_TIME_VAR: c_int = (MCLBN_FR_UNIT_SIZE * 10 + MCLBN_FP_UNIT_SIZE) as c_int;

macro_rules! common_impl {
    ($t:ty) => {
        impl $t {
            pub fn zero() -> $t {
                Default::default()
            }
            pub unsafe fn uninit() -> $t {
                std::mem::MaybeUninit::uninit().assume_init()
            }
            pub fn clear(&mut self) {
                *self = <$t>::zero()
            }
        }
    };
}

macro_rules! dec_impl {
    ($func_name:ident, $class:ident, $dec_fn:ident) => {
        impl SecretKey {
            pub fn $func_name(&self, c: *const $class) -> Result<i64, SheError> {
                let mut v: i64 = 0;
                if unsafe { $dec_fn(&mut v, self, c) } == 0 {
                    return Ok(v);
                } else {
                    Err(SheError::CantDecrypt)
                }
            }
        }
    };
}

macro_rules! enc_impl {
    ($func_name:ident, $class:ident, $enc_fn:ident) => {
        impl PublicKey {
            pub fn $func_name(&self, m: i64) -> $class {
                let mut v = unsafe { $class::uninit() };
                unsafe {
                    $enc_fn(&mut v, self, m);
                }
                v
            }
        }
    };
}

macro_rules! add_impl {
    ($func_name:ident, $class:ident, $add_fn:ident) => {
        pub fn $func_name(c1: &$class, c2: &$class) -> $class {
            let mut v = unsafe { $class::uninit() };
            unsafe {
                $add_fn(&mut v, c1, c2);
            }
            v
        }
    };
}

macro_rules! sub_impl {
    ($func_name:ident, $class:ident, $sub_fn:ident) => {
        pub fn $func_name(c1: &$class, c2: &$class) -> $class {
            let mut v = unsafe { $class::uninit() };
            unsafe {
                $sub_fn(&mut v, c1, c2);
            }
            v
        }
    };
}

macro_rules! mul_impl {
    ($func_name:ident, $class:ident, $mul_fn:ident) => {
        pub fn $func_name(c: &$class, x: i64) -> $class {
            let mut v = unsafe { $class::uninit() };
            unsafe {
                $mul_fn(&mut v, c, x);
            }
            v
        }
    };
}

macro_rules! neg_impl {
    ($func_name:ident, $class:ident, $neg_fn:ident) => {
        pub fn $func_name(c: &$class) -> $class {
            let mut v = unsafe { $class::uninit() };
            unsafe {
                $neg_fn(&mut v, c);
            }
            v
        }
    };
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct Fp {
    d: [u64; MCLBN_FP_UNIT_SIZE],
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct Fr {
    d: [u64; MCLBN_FR_UNIT_SIZE],
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct Fp2 {
    d: [Fp; 2],
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct G1 {
    pub x: Fp,
    pub y: Fp,
    pub z: Fp,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct G2 {
    pub x: Fp2,
    pub y: Fp2,
    pub z: Fp2,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct GT {
    d: [Fp; 12],
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct SecretKey {
    pub x: Fr,
    pub y: Fr,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct PublicKey {
    pub xP: G1,
    pub yQ: G2,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct CipherTextG1 {
    pub S: G1,
    pub T: G1,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct CipherTextG2 {
    pub S: G2,
    pub T: G2,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct CipherTextGT {
    pub g: [GT; 4],
}

common_impl![SecretKey];
common_impl![PublicKey];
common_impl![CipherTextG1];
common_impl![CipherTextG2];
common_impl![CipherTextGT];

dec_impl![dec_g1, CipherTextG1, sheDecG1];
dec_impl![dec_g2, CipherTextG2, sheDecG2];
dec_impl![dec_gt, CipherTextGT, sheDecGT];

impl SecretKey {
    pub fn set_by_csprng(&mut self) {
        if !unsafe { sheSecretKeySetByCSPRNG(self) == 0 } {
            panic!("sheSecretKeySetByCSPRNG")
        }
    }
    pub fn get_publickey(&self) -> PublicKey {
        let mut v = unsafe { PublicKey::uninit() };
        unsafe {
            sheGetPublicKey(&mut v, self);
        }
        v
    }
}

enc_impl![enc_g1, CipherTextG1, sheEncG1];
enc_impl![enc_g2, CipherTextG2, sheEncG2];
enc_impl![enc_gt, CipherTextGT, sheEncGT];

impl PublicKey {}

mul_impl![mul_g1, CipherTextG1, sheMulG1];
mul_impl![mul_g2, CipherTextG2, sheMulG2];
mul_impl![mul_gt, CipherTextGT, sheMulGT];

pub fn mul(c1: &CipherTextG1, c2: &CipherTextG2) -> CipherTextGT {
    let mut v = unsafe { CipherTextGT::uninit() };
    unsafe {
        sheMul(&mut v, c1, c2);
    }
    v
}

/*
serialize_impl![
    Fp,
    mclBn_getFpByteSize(),
    mclBnFp_serialize,
    mclBnFp_deserialize
];
*/

// for 2 level homomorphic encryption (curve = BN254 or BLS12_381)
pub fn init(curve: CurveType) -> bool {
    unsafe { sheInit(curve as c_int, MCLBN_COMPILED_TIME_VAR) == 0 }
}

// for only lifted-ElGamal encryption (curve = SECP256K1)
pub fn init_g1_only(curve: CurveType) -> bool {
    unsafe { sheInitG1only(curve as c_int, MCLBN_COMPILED_TIME_VAR) == 0 }
}

/*
    dec() can decrypt Enc(x) such that |x| <= hash_size * try_num
    The table size of DLP is hash_size * 4 bytes
    decryption time is alpha + beta * int(x/hash_size)
    where alpha and beta are constant
*/
pub fn set_try_num(try_num: usize) {
    unsafe { sheSetTryNum(try_num) }
}

// make hash_size entry table for all DLP
pub fn set_range_for_dlp(hash_size: usize) -> bool {
    unsafe { sheSetRangeForDLP(hash_size) == 0 }
}

// make hash_size entry table for G1 DLP
pub fn set_range_for_g1_dlp(hash_size: usize) -> bool {
    unsafe { sheSetRangeForG1DLP(hash_size) == 0 }
}

// make hash_size entry table for G2 DLP
pub fn set_range_for_g2_dlp(hash_size: usize) -> bool {
    unsafe { sheSetRangeForG2DLP(hash_size) == 0 }
}

// make hash_size entry table for GT DLP
pub fn set_range_for_gt_dlp(hash_size: usize) -> bool {
    unsafe { sheSetRangeForGTDLP(hash_size) == 0 }
}

add_impl![add_g1, CipherTextG1, sheAddG1];
add_impl![add_g2, CipherTextG2, sheAddG2];
add_impl![add_gt, CipherTextGT, sheAddGT];

sub_impl![sub_g1, CipherTextG1, sheSubG1];
sub_impl![sub_g2, CipherTextG2, sheSubG2];
sub_impl![sub_gt, CipherTextGT, sheSubGT];

neg_impl![neg_g1, CipherTextG1, sheNegG1];
neg_impl![neg_g2, CipherTextG2, sheNegG2];
neg_impl![neg_gt, CipherTextGT, sheNegGT];
