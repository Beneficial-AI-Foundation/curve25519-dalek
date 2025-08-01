// -*- mode: rust; -*-
//
// This file is part of curve25519-dalek.
// Copyright (c) 2016-2021 isis lovecruft
// Copyright (c) 2016-2019 Henry de Valence
// See LICENSE for licensing information.
//
// Authors:
// - isis agora lovecruft <isis@patternsinthevoid.net>
// - Henry de Valence <hdevalence@hdevalence.ca>

//! This module contains various constants (such as curve parameters
//! and useful field elements like `sqrt(-1)`), as well as
//! lookup tables of pre-computed points.

use super::field::FieldElement2625;
use super::scalar::Scalar29;
use crate::edwards::EdwardsPoint;

#[cfg(feature = "precomputed-tables")]
use crate::{
    backend::serial::curve_models::AffineNielsPoint,
    edwards::EdwardsBasepointTable,
    window::{LookupTable, NafLookupTable8},
};

/// The value of minus one, equal to `-&FieldElement::ONE`
pub(crate) const MINUS_ONE: FieldElement2625 = FieldElement2625::from_limbs([
    67108844, 33554431, 67108863, 33554431, 67108863, 33554431, 67108863, 33554431, 67108863,
    33554431,
]);

/// sqrt(-486664)
#[cfg(feature = "digest")]
pub(crate) const ED25519_SQRTAM2: FieldElement2625 = FieldElement2625::from_limbs([
    54885894, 25242303, 55597453, 9067496, 51808079, 33312638, 25456129, 14121551, 54921728,
    3972023,
]);

/// Edwards `d` value, equal to `-121665/121666 mod p`.
pub(crate) const EDWARDS_D: FieldElement2625 = FieldElement2625::from_limbs([
    56195235, 13857412, 51736253, 6949390, 114729, 24766616, 60832955, 30306712, 48412415, 21499315,
]);

/// Edwards `2*d` value, equal to `2*(-121665/121666) mod p`.
pub(crate) const EDWARDS_D2: FieldElement2625 = FieldElement2625::from_limbs([
    45281625, 27714825, 36363642, 13898781, 229458, 15978800, 54557047, 27058993, 29715967, 9444199,
]);

/// One minus edwards `d` value squared, equal to `(1 - (-121665/121666) mod p) pow 2`
pub(crate) const ONE_MINUS_EDWARDS_D_SQUARED: FieldElement2625 = FieldElement2625::from_limbs([
    6275446, 16937061, 44170319, 29780721, 11667076, 7397348, 39186143, 1766194, 42675006, 672202,
]);

/// Edwards `d` value minus one squared, equal to `(((-121665/121666) mod p) - 1) pow 2`
pub(crate) const EDWARDS_D_MINUS_ONE_SQUARED: FieldElement2625 = FieldElement2625::from_limbs([
    15551776, 22456977, 53683765, 23429360, 55212328, 10178283, 40474537, 4729243, 61826754,
    23438029,
]);

/// `= sqrt(a*d - 1)`, where `a = -1 (mod p)`, `d` are the Edwards curve parameters.
pub(crate) const SQRT_AD_MINUS_ONE: FieldElement2625 = FieldElement2625::from_limbs([
    24849947, 33400850, 43495378, 6347714, 46036536, 32887293, 41837720, 18186727, 66238516,
    14525638,
]);

/// `= 1/sqrt(a-d)`, where `a = -1 (mod p)`, `d` are the Edwards curve parameters.
pub(crate) const INVSQRT_A_MINUS_D: FieldElement2625 = FieldElement2625::from_limbs([
    6111466, 4156064, 39310137, 12243467, 41204824, 120896, 20826367, 26493656, 6093567, 31568420,
]);

/// Precomputed value of one of the square roots of -1 (mod p)
pub(crate) const SQRT_M1: FieldElement2625 = FieldElement2625::from_limbs([
    34513072, 25610706, 9377949, 3500415, 12389472, 33281959, 41962654, 31548777, 326685, 11406482,
]);

/// `APLUS2_OVER_FOUR` is (A+2)/4. (This is used internally within the Montgomery ladder.)
pub(crate) const APLUS2_OVER_FOUR: FieldElement2625 =
    FieldElement2625::from_limbs([121666, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

/// `MONTGOMERY_A` is equal to 486662, which is a constant of the curve equation
/// for Curve25519 in its Montgomery form. (This is used internally within the
/// Elligator map.)
pub(crate) const MONTGOMERY_A: FieldElement2625 =
    FieldElement2625::from_limbs([486662, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

/// `MONTGOMERY_A_NEG` is equal to -486662. (This is used internally within the
/// Elligator map.)
pub(crate) const MONTGOMERY_A_NEG: FieldElement2625 = FieldElement2625::from_limbs([
    66622183, 33554431, 67108863, 33554431, 67108863, 33554431, 67108863, 33554431, 67108863,
    33554431,
]);

/// `L` is the order of base point, i.e. 2^252 +
/// 27742317777372353535851937790883648493
pub(crate) const L: Scalar29 = Scalar29 {
    limbs: [
        0x1cf5d3ed, 0x009318d2, 0x1de73596, 0x1df3bd45, 0x0000014d, 0x00000000, 0x00000000,
        0x00000000, 0x00100000,
    ],
};

/// `L` * `LFACTOR` = -1 (mod 2^29)
pub(crate) const LFACTOR: u32 = 0x12547e1b;

/// `R` = R % L where R = 2^261
pub(crate) const R: Scalar29 = Scalar29 {
    limbs: [
        0x114df9ed, 0x1a617303, 0x0f7c098c, 0x16793167, 0x1ffd656e, 0x1fffffff, 0x1fffffff,
        0x1fffffff, 0x000fffff,
    ],
};

/// `RR` = (R^2) % L where R = 2^261
pub(crate) const RR: Scalar29 = Scalar29 {
    limbs: [
        0x0b5f9d12, 0x1e141b17, 0x158d7f3d, 0x143f3757, 0x1972d781, 0x042feb7c, 0x1ceec73d,
        0x1e184d1e, 0x0005046d,
    ],
};

/// The Ed25519 basepoint, as an `EdwardsPoint`.
///
/// This is called `_POINT` to distinguish it from
/// `ED25519_BASEPOINT_TABLE`, which should be used for scalar
/// multiplication (it's much faster).
pub const ED25519_BASEPOINT_POINT: EdwardsPoint = EdwardsPoint {
    X: FieldElement2625::from_limbs([
        52811034, 25909283, 16144682, 17082669, 27570973, 30858332, 40966398, 8378388, 20764389,
        8758491,
    ]),
    Y: FieldElement2625::from_limbs([
        40265304, 26843545, 13421772, 20132659, 26843545, 6710886, 53687091, 13421772, 40265318,
        26843545,
    ]),
    Z: FieldElement2625::from_limbs([1, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    T: FieldElement2625::from_limbs([
        28827043, 27438313, 39759291, 244362, 8635006, 11264893, 19351346, 13413597, 16611511,
        27139452,
    ]),
};

/// The 8-torsion subgroup \\(\mathcal E \[8\]\\).
///
/// In the case of Curve25519, it is cyclic; the \\(i\\)-th element of
/// the array is \\([i]P\\), where \\(P\\) is a point of order \\(8\\)
/// generating \\(\mathcal E\[8\]\\).
///
/// Thus \\(\mathcal E\[4\]\\) is the points indexed by `0,2,4,6`, and
/// \\(\mathcal E\[2\]\\) is the points indexed by `0,4`.
/// The Ed25519 basepoint has y = 4/5.  This is called `_POINT` to
/// distinguish it from `_TABLE`, which should be used for scalar
/// multiplication (it's much faster).
pub const EIGHT_TORSION: [EdwardsPoint; 8] = EIGHT_TORSION_INNER_DOC_HIDDEN;

/// Inner item used to hide limb constants from cargo doc output.
#[doc(hidden)]
pub const EIGHT_TORSION_INNER_DOC_HIDDEN: [EdwardsPoint; 8] = [
    EdwardsPoint {
        X: FieldElement2625::from_limbs([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        Y: FieldElement2625::from_limbs([1, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        Z: FieldElement2625::from_limbs([1, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        T: FieldElement2625::from_limbs([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    },
    EdwardsPoint {
        X: FieldElement2625::from_limbs([
            21352778, 5345713, 4660180, 25206575, 24143089, 14568123, 30185756, 21306662, 33579924,
            8345318,
        ]),
        Y: FieldElement2625::from_limbs([
            6952903, 1265500, 60246523, 7057497, 4037696, 5447722, 35427965, 15325401, 19365852,
            31985330,
        ]),
        Z: FieldElement2625::from_limbs([1, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        T: FieldElement2625::from_limbs([
            41846657, 21581751, 11716001, 27684820, 48915701, 16297738, 20670665, 24995334,
            3541542, 28543251,
        ]),
    },
    EdwardsPoint {
        X: FieldElement2625::from_limbs([
            32595773, 7943725, 57730914, 30054016, 54719391, 272472, 25146209, 2005654, 66782178,
            22147949,
        ]),
        Y: FieldElement2625::from_limbs([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        Z: FieldElement2625::from_limbs([1, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        T: FieldElement2625::from_limbs([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    },
    EdwardsPoint {
        X: FieldElement2625::from_limbs([
            21352778, 5345713, 4660180, 25206575, 24143089, 14568123, 30185756, 21306662, 33579924,
            8345318,
        ]),
        Y: FieldElement2625::from_limbs([
            60155942, 32288931, 6862340, 26496934, 63071167, 28106709, 31680898, 18229030,
            47743011, 1569101,
        ]),
        Z: FieldElement2625::from_limbs([1, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        T: FieldElement2625::from_limbs([
            25262188, 11972680, 55392862, 5869611, 18193162, 17256693, 46438198, 8559097, 63567321,
            5011180,
        ]),
    },
    EdwardsPoint {
        X: FieldElement2625::from_limbs([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        Y: FieldElement2625::from_limbs([
            67108844, 33554431, 67108863, 33554431, 67108863, 33554431, 67108863, 33554431,
            67108863, 33554431,
        ]),
        Z: FieldElement2625::from_limbs([1, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        T: FieldElement2625::from_limbs([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    },
    EdwardsPoint {
        X: FieldElement2625::from_limbs([
            45756067, 28208718, 62448683, 8347856, 42965774, 18986308, 36923107, 12247769,
            33528939, 25209113,
        ]),
        Y: FieldElement2625::from_limbs([
            60155942, 32288931, 6862340, 26496934, 63071167, 28106709, 31680898, 18229030,
            47743011, 1569101,
        ]),
        Z: FieldElement2625::from_limbs([1, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        T: FieldElement2625::from_limbs([
            41846657, 21581751, 11716001, 27684820, 48915701, 16297738, 20670665, 24995334,
            3541542, 28543251,
        ]),
    },
    EdwardsPoint {
        X: FieldElement2625::from_limbs([
            34513072, 25610706, 9377949, 3500415, 12389472, 33281959, 41962654, 31548777, 326685,
            11406482,
        ]),
        Y: FieldElement2625::from_limbs([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        Z: FieldElement2625::from_limbs([1, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        T: FieldElement2625::from_limbs([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    },
    EdwardsPoint {
        X: FieldElement2625::from_limbs([
            45756067, 28208718, 62448683, 8347856, 42965774, 18986308, 36923107, 12247769,
            33528939, 25209113,
        ]),
        Y: FieldElement2625::from_limbs([
            6952903, 1265500, 60246523, 7057497, 4037696, 5447722, 35427965, 15325401, 19365852,
            31985330,
        ]),
        Z: FieldElement2625::from_limbs([1, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        T: FieldElement2625::from_limbs([
            25262188, 11972680, 55392862, 5869611, 18193162, 17256693, 46438198, 8559097, 63567321,
            5011180,
        ]),
    },
];

/// Table containing precomputed multiples of the Ed25519 basepoint \\(B = (x, 4/5)\\).
#[cfg(feature = "precomputed-tables")]
pub static ED25519_BASEPOINT_TABLE: &'static EdwardsBasepointTable =
    &ED25519_BASEPOINT_TABLE_INNER_DOC_HIDDEN;

/// Inner constant, used to avoid filling the docs with precomputed points.
#[doc(hidden)]
#[cfg(feature = "precomputed-tables")]
static ED25519_BASEPOINT_TABLE_INNER_DOC_HIDDEN: EdwardsBasepointTable = EdwardsBasepointTable([
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                93076338, 52752828, 29566454, 37215328, 54414518, 37569218, 94653489, 21800160,
                61029707, 35602036,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                54563134, 934261, 64385954, 3049989, 66381436, 9406985, 12720692, 5043384,
                19500929, 18085054,
            ]),
            xy2d: FieldElement2625::from_limbs([
                58370664, 4489569, 9688441, 18769238, 10184608, 21191052, 29287918, 11864899,
                42594502, 29115885,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                54292951, 54132516, 45527619, 11784319, 41753206, 30803714, 55390960, 29739860,
                66750418, 23343128,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                45405608, 6903824, 27185491, 6451973, 37531140, 24000426, 51492312, 11189267,
                40279186, 28235350,
            ]),
            xy2d: FieldElement2625::from_limbs([
                26966623, 11152617, 32442495, 15396054, 14353839, 20802097, 63980037, 24013313,
                51636816, 29387734,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                82745136, 23865874, 24204772, 25642034, 67725840, 16869169, 94896463, 52336674,
                28944398, 32004408,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                16568933, 4717097, 55552716, 32452109, 15682895, 21747389, 16354576, 21778470,
                7689661, 11199574,
            ]),
            xy2d: FieldElement2625::from_limbs([
                30464137, 27578307, 55329429, 17883566, 23220364, 15915852, 7512774, 10017326,
                49359771, 23634074,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                50071967, 13921891, 78054670, 27521000, 27105051, 17470053, 105291517, 15006021,
                70393432, 27277891,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                23599295, 25248385, 55915199, 25867015, 13236773, 10506355, 7464579, 9656445,
                13059162, 10374397,
            ]),
            xy2d: FieldElement2625::from_limbs([
                7798537, 16710257, 3033922, 2874086, 28997861, 2835604, 32406664, 29715387,
                66467155, 33453106,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                77970208, 11473153, 27284546, 35535607, 37044514, 46132292, 99976748, 48069538,
                118779423, 44373810,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                4708026, 6336745, 20377586, 9066809, 55836755, 6594695, 41455196, 12483687,
                54440373, 5581305,
            ]),
            xy2d: FieldElement2625::from_limbs([
                19563141, 16186464, 37722007, 4097518, 10237984, 29206317, 28542349, 13850243,
                43430843, 17738489,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                51736881, 20691677, 32573249, 4720197, 107781206, 39429941, 115029100, 18329611,
                124398787, 21468653,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                58559652, 109982, 15149363, 2178705, 22900618, 4543417, 3044240, 17864545, 1762327,
                14866737,
            ]),
            xy2d: FieldElement2625::from_limbs([
                48909169, 17603008, 56635573, 1707277, 49922944, 3916100, 38872452, 3959420,
                27914454, 4383652,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                72262591, 43463716, 68832610, 30776557, 97632468, 39071304, 86589715, 38784565,
                43156424, 18378665,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                36839857, 30090922, 7665485, 10083793, 28475525, 1649722, 20654025, 16520125,
                30598449, 7715701,
            ]),
            xy2d: FieldElement2625::from_limbs([
                28881826, 14381568, 9657904, 3680757, 46927229, 7843315, 35708204, 1370707,
                29794553, 32145132,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                14499452, 64379265, 33917749, 62854211, 95603724, 14271266, 97399599, 10876453,
                33954766, 35936157,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                59913433, 30899068, 52378708, 462250, 39384538, 3941371, 60872247, 3696004,
                34808032, 15351954,
            ]),
            xy2d: FieldElement2625::from_limbs([
                27431194, 8222322, 16448760, 29646437, 48401861, 11938354, 34147463, 30583916,
                29551812, 10109425,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                53451805, 20399000, 102933977, 45331528, 88556249, 40073815, 64730579, 31926875,
                77201646, 28790260,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                27939166, 14210322, 4677035, 16277044, 44144402, 21156292, 34600109, 12005537,
                49298737, 12803509,
            ]),
            xy2d: FieldElement2625::from_limbs([
                17228999, 17892808, 65875336, 300139, 65883994, 21839654, 30364212, 24516238,
                18016356, 4397660,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                56150002, 25864224, 4776340, 18600194, 27850027, 17952220, 40489757, 14544524,
                49631360, 34537070,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                29253598, 15796703, 64244882, 23645547, 10057022, 3163536, 7332899, 29434304,
                46061167, 9934962,
            ]),
            xy2d: FieldElement2625::from_limbs([
                5793284, 16271923, 42977250, 23438027, 29188559, 1206517, 52360934, 4559894,
                36984942, 22656481,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                39464893, 55615857, 83391519, 22517938, 28414020, 52096600, 24191032, 38096129,
                53770554, 39054999,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                12650548, 32057319, 9052870, 11355358, 49428827, 25154267, 49678271, 12264342,
                10874051, 13524335,
            ]),
            xy2d: FieldElement2625::from_limbs([
                25556948, 30508442, 714650, 2510400, 23394682, 23139102, 33119037, 5080568,
                44580805, 5376627,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                108129445, 29543378, 50095164, 30016803, 60382070, 35475328, 44787558, 57661420,
                71644630, 35123438,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                64853442, 14606629, 45416424, 25514613, 28430648, 8775819, 36614302, 3044289,
                31848280, 12543772,
            ]),
            xy2d: FieldElement2625::from_limbs([
                45080285, 2943892, 35251351, 6777305, 13784462, 29262229, 39731668, 31491700,
                7718481, 14474653,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                69494160, 36008644, 44477543, 33601034, 62670928, 51428448, 67765827, 26317766,
                91425031, 28300864,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                13741529, 10911568, 33875447, 24950694, 46931033, 32521134, 33040650, 20129900,
                46379407, 8321685,
            ]),
            xy2d: FieldElement2625::from_limbs([
                21060490, 31341688, 15712756, 29218333, 1639039, 10656336, 23845965, 21679594,
                57124405, 608371,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                53436113, 18466845, 56219170, 25997372, 61071954, 11305546, 68232832, 60328286,
                94338261, 33578318,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                43864724, 33260226, 55364135, 14712570, 37643165, 31524814, 12797023, 27114124,
                65475458, 16678953,
            ]),
            xy2d: FieldElement2625::from_limbs([
                37608244, 4770661, 51054477, 14001337, 7830047, 9564805, 65600720, 28759386,
                49939598, 4904952,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                91168402, 48171434, 86146020, 18514523, 86874956, 18648002, 72278074, 16191879,
                69237100, 29227598,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                50127693, 4124965, 58568254, 22900634, 30336521, 19449185, 37302527, 916032,
                60226322, 30567899,
            ]),
            xy2d: FieldElement2625::from_limbs([
                44477957, 12419371, 59974635, 26081060, 50629959, 16739174, 285431, 2763829,
                15736322, 4143876,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                69488197, 11839344, 62998462, 27565766, 78383161, 34349388, 67321664, 18959768,
                23527083, 17096164,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                33431108, 22423954, 49269897, 17927531, 8909498, 8376530, 34483524, 4087880,
                51919953, 19138217,
            ]),
            xy2d: FieldElement2625::from_limbs([
                1767664, 7197987, 53903638, 31531796, 54017513, 448825, 5799055, 4357868, 62334673,
                17231393,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                6721947, 47388255, 43585475, 32003117, 93463156, 21691110, 90474010, 29604699,
                74499753, 36314231,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                4409022, 2052381, 23373853, 10530217, 7676779, 20668478, 21302352, 29290375,
                1244379, 20634787,
            ]),
            xy2d: FieldElement2625::from_limbs([
                62687625, 7169618, 4982368, 30596842, 30256824, 30776892, 14086412, 9208236,
                15886429, 16489664,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                69104920, 43930080, 81455230, 46865633, 60234728, 17116020, 120524529, 33952799,
                36502408, 32841498,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                41801399, 9795879, 64331450, 14878808, 33577029, 14780362, 13348553, 12076947,
                36272402, 5113181,
            ]),
            xy2d: FieldElement2625::from_limbs([
                49338080, 11797795, 31950843, 13929123, 41220562, 12288343, 36767763, 26218045,
                13847710, 5387222,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                48526682, 30138214, 84933706, 64767897, 89853205, 56666252, 75871923, 37172217,
                47508201, 43925422,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                20246567, 19185054, 22358228, 33010720, 18507282, 23140436, 14554436, 24808340,
                32232923, 16763880,
            ]),
            xy2d: FieldElement2625::from_limbs([
                9648486, 10094563, 26416693, 14745928, 36734546, 27081810, 11094160, 15689506,
                3140038, 17044340,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                50948773, 39027126, 31895587, 38299426, 75932378, 43920116, 39884063, 43003044,
                38334409, 33920726,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                19153450, 11523972, 56012374, 27051289, 42461232, 5420646, 28344573, 8041113,
                719605, 11671788,
            ]),
            xy2d: FieldElement2625::from_limbs([
                8678006, 2694440, 60300850, 2517371, 4964326, 11152271, 51675948, 18287915,
                27000812, 23358879,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                119059805, 40688742, 75748150, 30739554, 59873175, 43976173, 67672928, 38890528,
                73859840, 19033405,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                11836410, 29574944, 26297893, 16080799, 23455045, 15735944, 1695823, 24735310,
                8169719, 16220347,
            ]),
            xy2d: FieldElement2625::from_limbs([
                48993007, 8653646, 17578566, 27461813, 59083086, 17541668, 55964556, 30926767,
                61118155, 19388398,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                43800347, 22586119, 82322091, 23473217, 36255258, 22504427, 27884328, 36401716,
                69764724, 35292826,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                39571412, 19301410, 41772562, 25551651, 57738101, 8129820, 21651608, 30315096,
                48021414, 22549153,
            ]),
            xy2d: FieldElement2625::from_limbs([
                1533110, 3437855, 23735889, 459276, 29970501, 11335377, 26030092, 5821408,
                10478196, 8544890,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                32173083, 50979553, 24896205, 37475929, 22579055, 63698010, 19270447, 45771905,
                84897880, 63712868,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                36555903, 31326030, 51530034, 23407230, 13243888, 517024, 15479401, 29701199,
                30460519, 1052596,
            ]),
            xy2d: FieldElement2625::from_limbs([
                55493970, 13323617, 32618793, 8175907, 51878691, 12596686, 27491595, 28942073,
                3179267, 24075541,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                99055914, 52742212, 62468279, 18214510, 51982886, 27514722, 52352086, 17142691,
                19072639, 24043372,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                11685058, 11822410, 3158003, 19601838, 33402193, 29389366, 5977895, 28339415,
                473098, 5040608,
            ]),
            xy2d: FieldElement2625::from_limbs([
                46817982, 8198641, 39698732, 11602122, 1290375, 30754672, 28326861, 1721092,
                47550222, 30422825,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                74990396, 10687936, 74687587, 7738377, 48157852, 31000479, 88929649, 8076148,
                39240368, 11538388,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                47173198, 3899860, 18283497, 26752864, 51380203, 22305220, 8754524, 7446702,
                61432810, 5797015,
            ]),
            xy2d: FieldElement2625::from_limbs([
                55813245, 29760862, 51326753, 25589858, 12708868, 25098233, 2014098, 24503858,
                64739691, 27677090,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                111745333, 55540121, 106535706, 34700805, 86065554, 50194990, 68301593, 29840232,
                82232482, 44365936,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                14352079, 30134717, 48166819, 10822654, 32750596, 4699007, 67038501, 15776355,
                38222085, 21579878,
            ]),
            xy2d: FieldElement2625::from_limbs([
                38867681, 25481956, 62129901, 28239114, 29416930, 1847569, 46454691, 17069576,
                4714546, 23953777,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                15200313, 41923004, 86787964, 15970073, 35236190, 35513882, 24611598, 29010600,
                55362987, 45894651,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                12876937, 23074376, 33134380, 6590940, 60801088, 14872439, 9613953, 8241152,
                15370987, 9608631,
            ]),
            xy2d: FieldElement2625::from_limbs([
                62965568, 21540023, 8446280, 33162829, 4407737, 13629032, 59383996, 15866073,
                38898243, 24740332,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                26660609, 51431209, 75502596, 33912478, 59707572, 34547419, 43204630, 34413128,
                87680086, 41974987,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                14620696, 13067227, 51661590, 8264466, 14106269, 15080814, 33531827, 12516406,
                45534429, 21077682,
            ]),
            xy2d: FieldElement2625::from_limbs([
                236881, 10476226, 57258, 18877408, 6472997, 2466984, 17258519, 7256740, 8791136,
                15069930,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                68385255, 24182513, 90058498, 17231624, 43615824, 61406677, 81820737, 38428660,
                36445723, 31223040,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                5855666, 4990204, 53397016, 7294283, 59304582, 1924646, 65685689, 25642053,
                34039526, 9234252,
            ]),
            xy2d: FieldElement2625::from_limbs([
                20590503, 24535444, 31529743, 26201766, 64402029, 10650547, 31559055, 21944845,
                18979185, 13396066,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                24474268, 38522535, 22267081, 37961786, 91172745, 25229251, 48291976, 13594781,
                33514650, 40576390,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                55541958, 26988926, 45743778, 15928891, 40950559, 4315420, 41160136, 29637754,
                45628383, 12868081,
            ]),
            xy2d: FieldElement2625::from_limbs([
                38473832, 13504660, 19988037, 31421671, 21078224, 6443208, 45662757, 2244499,
                54653067, 25465048,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                36513317, 13793478, 61256044, 33873567, 41385691, 60844964, 100195408, 8957936,
                51875216, 39094952,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                55478669, 22050529, 58989363, 25911358, 2620055, 1022908, 43398120, 31985447,
                50980335, 18591624,
            ]),
            xy2d: FieldElement2625::from_limbs([
                23152952, 775386, 27395463, 14006635, 57407746, 4649511, 1689819, 892185, 55595587,
                18348483,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                76878974, 43141169, 93604957, 37878551, 68665374, 30004407, 94562682, 38317558,
                47929249, 39421565,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                34343820, 1927589, 31726409, 28801137, 23962433, 17534932, 27846558, 5931263,
                37359161, 17445976,
            ]),
            xy2d: FieldElement2625::from_limbs([
                27461885, 30576896, 22380809, 1815854, 44075111, 30522493, 7283489, 18406359,
                47582163, 7734628,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                59098581, 57518046, 55988459, 39750469, 29344157, 20123547, 74694158, 30377805,
                85658360, 48856500,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                34450527, 27383209, 59436070, 22502750, 6258877, 13504381, 10458790, 27135971,
                58236621, 8424745,
            ]),
            xy2d: FieldElement2625::from_limbs([
                24687186, 8613276, 36441818, 30320886, 1863891, 31723888, 19206233, 7134917,
                55824382, 32725512,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                11334880, 24336410, 75134156, 46261950, 84632755, 23078360, 77352601, 18868970,
                62042829, 50053268,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                8911542, 6887158, 57524604, 26595841, 11145640, 24010752, 17303924, 19430194,
                6536640, 10543906,
            ]),
            xy2d: FieldElement2625::from_limbs([
                38162480, 15479762, 49642029, 568875, 65611181, 11223453, 64439674, 16928857,
                39873154, 8876770,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                41365946, 54541999, 118567760, 32707823, 101191041, 32758142, 33627041, 15824473,
                66504438, 24514614,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                10330056, 70051, 7957388, 24551765, 9764901, 15609756, 27698697, 28664395, 1657393,
                3084098,
            ]),
            xy2d: FieldElement2625::from_limbs([
                10477963, 26084172, 12119565, 20303627, 29016246, 28188843, 31280318, 14396151,
                36875289, 15272408,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                54820536, 36723894, 28813182, 16658753, 92225296, 27923965, 109043770, 54472724,
                42094105, 35504935,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                40928506, 9489186, 11053416, 18808271, 36055143, 5825629, 58724558, 24786899,
                15341278, 8373727,
            ]),
            xy2d: FieldElement2625::from_limbs([
                28685821, 7759505, 52730348, 21551571, 35137043, 4079241, 298136, 23321830,
                64230656, 15190419,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                34175950, 47360767, 52771378, 51314432, 110213106, 10940926, 75778582, 36296824,
                108184414, 60233859,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                65528476, 21825014, 41129205, 22109408, 49696989, 22641577, 9291593, 17306653,
                54954121, 6048604,
            ]),
            xy2d: FieldElement2625::from_limbs([
                36803549, 14843443, 1539301, 11864366, 20201677, 1900163, 13934231, 5128323,
                11213262, 9168384,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                40828313, 44562278, 19408959, 32613674, 115624762, 29225850, 62020803, 22449281,
                20470156, 50710163,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                43972811, 9282191, 14855179, 18164354, 59746048, 19145871, 44324911, 14461607,
                14042978, 5230683,
            ]),
            xy2d: FieldElement2625::from_limbs([
                29969548, 30812838, 50396996, 25001989, 9175485, 31085458, 21556950, 3506042,
                61174973, 21104723,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                63964099, 42299092, 19704002, 38135710, 46678177, 6830682, 45824694, 42525944,
                38569674, 48880994,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                47644235, 10110287, 49846336, 30050539, 43608476, 1355668, 51585814, 15300987,
                46594746, 9168259,
            ]),
            xy2d: FieldElement2625::from_limbs([
                61755510, 4488612, 43305616, 16314346, 7780487, 17915493, 38160505, 9601604,
                33087103, 24543045,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                47665675, 18041531, 46311396, 21109108, 104393280, 43783891, 39664534, 52108332,
                61111992, 49219103,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                23294591, 16921819, 44458082, 25083453, 27844203, 11461195, 13099750, 31094076,
                18151675, 13417686,
            ]),
            xy2d: FieldElement2625::from_limbs([
                42385932, 29377914, 35958184, 5988918, 40250079, 6685064, 1661597, 21002991,
                15271675, 18101767,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                78541887, 20325766, 75348494, 28274914, 65123427, 32828713, 48410099, 35721975,
                60187562, 20114249,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                35672693, 15575145, 30436815, 12192228, 44645511, 9395378, 57191156, 24915434,
                12215109, 12028277,
            ]),
            xy2d: FieldElement2625::from_limbs([
                14098381, 6555944, 23007258, 5757252, 51681032, 20603929, 30123439, 4617780,
                50208775, 32898803,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                63082644, 51868028, 79002030, 47273095, 52299401, 35401816, 51288864, 43708440,
                91082124, 20869957,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                40577025, 29858441, 65199965, 2534300, 35238307, 17004076, 18341389, 22134481,
                32013173, 23450893,
            ]),
            xy2d: FieldElement2625::from_limbs([
                41629544, 10876442, 55337778, 18929291, 54739296, 1838103, 21911214, 6354752,
                4425632, 32716610,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                56675456, 18941465, 89338721, 30463384, 53917697, 34331160, 116802352, 55088400,
                71833867, 47599401,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                19268631, 26250011, 1555348, 8692754, 45634805, 23643767, 6347389, 32142648,
                47586572, 17444675,
            ]),
            xy2d: FieldElement2625::from_limbs([
                42244775, 12986007, 56209986, 27995847, 55796492, 33405905, 19541417, 8180106,
                9282262, 10282508,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                108012627, 37982977, 58447667, 20360168, 71207265, 52943606, 15522533, 8372215,
                72651459, 22851748,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                56546323, 14895632, 26814552, 16880582, 49628109, 31065071, 64326972, 6993760,
                49014979, 10114654,
            ]),
            xy2d: FieldElement2625::from_limbs([
                47001790, 32625013, 31422703, 10427861, 59998115, 6150668, 38017109, 22025285,
                25953724, 33448274,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                62874448, 59069571, 57989737, 36600431, 69210472, 54501569, 86498882, 39648727,
                63793584, 46385556,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                51110167, 7578151, 5310217, 14408357, 33560244, 33329692, 31575953, 6326196,
                7381791, 31132593,
            ]),
            xy2d: FieldElement2625::from_limbs([
                46206085, 3296810, 24736065, 17226043, 18374253, 7318640, 6295303, 8082724,
                51746375, 12339663,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                27724736, 35845589, 73197064, 19369633, 68901590, 39412065, 80957277, 15768921,
                92200031, 14856293,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                48242193, 8331042, 24373479, 8541013, 66406866, 24284974, 12927299, 20858939,
                44926390, 24541532,
            ]),
            xy2d: FieldElement2625::from_limbs([
                55685435, 28132841, 11632844, 3405020, 30536730, 21880393, 39848098, 13866389,
                30146206, 9142070,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                71032974, 18246915, 120400605, 23499470, 79400683, 32886065, 39406089, 9326383,
                58871006, 37725725,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                51186905, 16037936, 6713787, 16606682, 45496729, 2790943, 26396185, 3731949,
                345228, 28091483,
            ]),
            xy2d: FieldElement2625::from_limbs([
                45781307, 13448258, 25284571, 1143661, 20614966, 24705045, 2031538, 21163201,
                50855680, 19972348,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                98125037, 16832002, 93480255, 52657630, 62081513, 14854136, 17477601, 37397089,
                28012649, 50703444,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                62033029, 9368965, 58546785, 28953529, 51858910, 6970559, 57918991, 16292056,
                58241707, 3507939,
            ]),
            xy2d: FieldElement2625::from_limbs([
                29439664, 3537914, 23333589, 6997794, 49553303, 22536363, 51899661, 18503164,
                57943934, 6580395,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                54922984, 59429075, 83547131, 10826159, 58412047, 27318820, 84969307, 24280585,
                65013061, 42858998,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                20714545, 29217521, 29088194, 7406487, 11426967, 28458727, 14792666, 18945815,
                5289420, 33077305,
            ]),
            xy2d: FieldElement2625::from_limbs([
                50443312, 22903641, 60948518, 20248671, 9192019, 31751970, 17271489, 12349094,
                26939669, 29802138,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                54218947, 9373457, 98704712, 16374214, 21471720, 13221525, 39825369, 54760304,
                63410056, 33672318,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                22263325, 26994382, 3984569, 22379786, 51994855, 32987646, 28311252, 5358056,
                43789084, 541963,
            ]),
            xy2d: FieldElement2625::from_limbs([
                16259200, 3261970, 2309254, 18019958, 50223152, 28972515, 24134069, 16848603,
                53771797, 20002236,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                76487005, 20414245, 111371745, 20809166, 95307144, 59864765, 64709178, 32837080,
                67799289, 48430675,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                24977353, 33240048, 58884894, 20089345, 28432342, 32378079, 54040059, 21257083,
                44727879, 6618998,
            ]),
            xy2d: FieldElement2625::from_limbs([
                65570671, 11685645, 12944378, 13682314, 42719353, 19141238, 8044828, 19737104,
                32239828, 27901670,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                48505798, 38317421, 66182613, 42439735, 105805247, 30367115, 76890510, 23204372,
                32779358, 5095274,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                34100715, 28339925, 34843976, 29869215, 9460460, 24227009, 42507207, 14506723,
                21639561, 30924196,
            ]),
            xy2d: FieldElement2625::from_limbs([
                50707921, 20442216, 25239337, 15531969, 3987758, 29055114, 65819361, 26690896,
                17874573, 558605,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                53508716, 10240080, 76280747, 16131052, 46239610, 43154131, 100608350, 38634582,
                69194755, 38674192,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                44903700, 31034903, 50727262, 414690, 42089314, 2170429, 30634760, 25190818,
                35108870, 27794547,
            ]),
            xy2d: FieldElement2625::from_limbs([
                60263160, 15791201, 8550074, 32241778, 29928808, 21462176, 27534429, 26362287,
                44757485, 12961481,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                42616785, 57538092, 10368192, 11582341, 110820435, 31309143, 83642793, 8206995,
                104023076, 28394792,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                55987368, 30172197, 2307365, 6362031, 66973409, 8868176, 50273234, 7031274,
                7589640, 8945490,
            ]),
            xy2d: FieldElement2625::from_limbs([
                34956097, 8917966, 6661220, 21876816, 65916803, 17761038, 7251488, 22372252,
                24099108, 19098262,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                72128384, 25646961, 71352990, 18840075, 107284455, 40007595, 47990681, 20265406,
                127985831, 56828126,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                10853575, 10721687, 26480089, 5861829, 44113045, 1972174, 65242217, 22996533,
                63745412, 27113307,
            ]),
            xy2d: FieldElement2625::from_limbs([
                50106456, 5906789, 221599, 26991285, 7828207, 20305514, 24362660, 31546264,
                53242455, 7421391,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                75248772, 27007934, 99366509, 27663885, 97484582, 1886180, 113042620, 48995682,
                95935221, 29431402,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                6267067, 9695052, 7709135, 16950835, 34239795, 31668296, 14795159, 25714308,
                13746020, 31812384,
            ]),
            xy2d: FieldElement2625::from_limbs([
                28584883, 7787108, 60375922, 18503702, 22846040, 25983196, 63926927, 33190907,
                4771361, 25134474,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                92058101, 6376278, 39642383, 25379823, 48462709, 23623825, 100652432, 54967168,
                70678489, 44897024,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                26514970, 4740088, 27912651, 3697550, 19331575, 22082093, 6809885, 4608608,
                7325975, 18753361,
            ]),
            xy2d: FieldElement2625::from_limbs([
                55490446, 19000001, 42787651, 7655127, 65739590, 5214311, 39708324, 10258389,
                49462170, 25367739,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                11431185, 49377439, 93679108, 47883555, 85138853, 38350513, 35662684, 49135095,
                76389221, 29580744,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                66948081, 23228174, 44253547, 29249434, 46247496, 19933429, 34297962, 22372809,
                51563772, 4387440,
            ]),
            xy2d: FieldElement2625::from_limbs([
                46309467, 12194511, 3937617, 27748540, 39954043, 9340369, 42594872, 8548136,
                20617071, 26072431,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                66170039, 29623845, 58394552, 49679149, 91711988, 27329038, 53333511, 55233041,
                91454545, 10325459,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                47253587, 31985546, 44906155, 8714033, 14007766, 6928528, 16318175, 32543743,
                4766742, 3552007,
            ]),
            xy2d: FieldElement2625::from_limbs([
                45357481, 16823515, 1351762, 32751011, 63099193, 3950934, 3217514, 14481909,
                10988822, 29559670,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                15564288, 19242862, 70210106, 39238579, 97555643, 25503075, 79785990, 27049088,
                58813011, 46850436,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                57666574, 6624295, 36809900, 21640754, 62437882, 31497052, 31521203, 9614054,
                37108040, 12074673,
            ]),
            xy2d: FieldElement2625::from_limbs([
                4771172, 33419193, 14290748, 20464580, 27992297, 14998318, 65694928, 31997715,
                29832612, 17163397,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                7064865, 59567690, 115055764, 62041325, 48217593, 30641695, 92934105, 38847728,
                39986203, 46656021,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                64810282, 2439669, 59642254, 1719964, 39841323, 17225986, 32512468, 28236839,
                36752793, 29363474,
            ]),
            xy2d: FieldElement2625::from_limbs([
                37102324, 10162315, 33928688, 3981722, 50626726, 20484387, 14413973, 9515896,
                19568978, 9628812,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                33053784, 33753789, 83003454, 35137490, 94489106, 28973996, 49269969, 61002024,
                60817076, 36992171,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                48129987, 3884492, 19469877, 12726490, 15913552, 13614290, 44147131, 70103,
                7463304, 4176122,
            ]),
            xy2d: FieldElement2625::from_limbs([
                39984863, 10659916, 11482427, 17484051, 12771466, 26919315, 34389459, 28231680,
                24216881, 5944158,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                76002989, 41005405, 64444714, 57343111, 106137209, 21165315, 19345745, 48235228,
                78741856, 5847884,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                26942781, 31239115, 9129563, 28647825, 26024104, 11769399, 55590027, 6367193,
                57381634, 4782139,
            ]),
            xy2d: FieldElement2625::from_limbs([
                19916442, 28726022, 44198159, 22140040, 25606323, 27581991, 33253852, 8220911,
                6358847, 31680575,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                67910273, 31472729, 16569427, 44619599, 29875703, 33651059, 75017251, 29073951,
                53570360, 34941586,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                19646058, 5720633, 55692158, 12814208, 11607948, 12749789, 14147075, 15156355,
                45242033, 11835259,
            ]),
            xy2d: FieldElement2625::from_limbs([
                19299512, 1155910, 28703737, 14890794, 2925026, 7269399, 26121523, 15467869,
                40548314, 5052482,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                64091413, 43612637, 69089700, 37518674, 22160965, 12322533, 60677741, 20936246,
                12228556, 26550755,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                32944382, 14922211, 44263970, 5188527, 21913450, 24834489, 4001464, 13238564,
                60994061, 8653814,
            ]),
            xy2d: FieldElement2625::from_limbs([
                22865569, 28901697, 27603667, 21009037, 14348957, 8234005, 24808405, 5719875,
                28483275, 2841751,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                117796741, 32441125, 66781144, 21446575, 21886281, 51556090, 65220896, 33238773,
                87040921, 20815228,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                55452759, 10087520, 58243976, 28018288, 47830290, 30498519, 3999227, 13239134,
                62331395, 19644223,
            ]),
            xy2d: FieldElement2625::from_limbs([
                1382174, 21859713, 17266789, 9194690, 53784508, 9720080, 20403944, 11284705,
                53095046, 3093229,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                83759766, 56070931, 66044684, 35125060, 58779117, 40907184, 66806439, 16271224,
                43059443, 26862581,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                45197768, 27626490, 62497547, 27994275, 35364760, 22769138, 24123613, 15193618,
                45456747, 16815042,
            ]),
            xy2d: FieldElement2625::from_limbs([
                57172930, 29264984, 41829040, 4372841, 2087473, 10399484, 31870908, 14690798,
                17361620, 11864968,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                55801216, 39764803, 80315437, 39360751, 105200035, 19587230, 54777658, 26067830,
                41530403, 50868174,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                14668443, 21284197, 26039038, 15305210, 25515617, 4542480, 10453892, 6577524,
                9145645, 27110552,
            ]),
            xy2d: FieldElement2625::from_limbs([
                5974855, 3053895, 57675815, 23169240, 35243739, 3225008, 59136222, 3936127,
                61456591, 30504127,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                97734231, 28825031, 41552902, 20761565, 46624288, 41249530, 17097187, 50805368,
                106217947, 35358062,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                63555773, 9865098, 61880298, 4272700, 61435032, 16864731, 14911343, 12196514,
                45703375, 7047411,
            ]),
            xy2d: FieldElement2625::from_limbs([
                20093258, 9920966, 55970670, 28210574, 13161586, 12044805, 34252013, 4124600,
                34765036, 23296865,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                46320021, 14084653, 53577151, 41396578, 19119037, 19731827, 71861240, 24839791,
                45429205, 35842469,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                40289628, 30270716, 29965058, 3039786, 52635099, 2540456, 29457502, 14625692,
                42289247, 12570231,
            ]),
            xy2d: FieldElement2625::from_limbs([
                66045306, 22002608, 16920317, 12494842, 1278292, 27685323, 45948920, 30055751,
                55134159, 4724942,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                85069815, 21778897, 62967895, 23851901, 58232301, 32143814, 54201480, 24894499,
                104641427, 35458286,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                23134274, 19275300, 56426866, 31942495, 20684484, 15770816, 54119114, 3190295,
                26955097, 14109738,
            ]),
            xy2d: FieldElement2625::from_limbs([
                15308788, 5320727, 36995055, 19235554, 22902007, 7767164, 29425325, 22276870,
                31960941, 11934971,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                39713134, 41990227, 71218507, 12222638, 109589860, 14818667, 87747037, 38429459,
                77600255, 34934149,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                53949449, 9197840, 3875503, 24618324, 65725151, 27674630, 33518458, 16176658,
                21432314, 12180697,
            ]),
            xy2d: FieldElement2625::from_limbs([
                55321537, 11500837, 13787581, 19721842, 44678184, 10140204, 1465425, 12689540,
                56807545, 19681548,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                72522936, 18168390, 46101199, 43198001, 79943833, 34740580, 64485947, 32212200,
                26128230, 39587344,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                40771450, 19788269, 32496024, 19900513, 17847800, 20885276, 3604024, 8316894,
                41233830, 23117073,
            ]),
            xy2d: FieldElement2625::from_limbs([
                3296484, 6223048, 24680646, 21307972, 44056843, 5903204, 58246567, 28915267,
                12376616, 3188849,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                29190450, 18895386, 27549112, 32370916, 70628929, 22857130, 32049514, 26245319,
                50999629, 57256556,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                52364359, 24245275, 735817, 32955454, 46701176, 28496527, 25246077, 17758763,
                18640740, 32593455,
            ]),
            xy2d: FieldElement2625::from_limbs([
                60180029, 17123636, 10361373, 5642961, 4910474, 12345252, 35470478, 33060001,
                10530746, 1053335,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                104951742, 52922057, 120679510, 54991489, 47651803, 56453479, 102755357, 30605445,
                24018830, 48581076,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                44516310, 30409154, 64819587, 5953842, 53668675, 9425630, 25310643, 13003497,
                64794073, 18408815,
            ]),
            xy2d: FieldElement2625::from_limbs([
                39688860, 32951110, 59064879, 31885314, 41016598, 13987818, 39811242, 187898,
                43942445, 31022696,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                45364447, 19743956, 68953703, 38575859, 123783328, 17642957, 76825530, 49821353,
                62038646, 34280530,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                29370903, 27500434, 7334070, 18212173, 9385286, 2247707, 53446902, 28714970,
                30007387, 17731091,
            ]),
            xy2d: FieldElement2625::from_limbs([
                66172485, 16086690, 23751945, 33011114, 65941325, 28365395, 9137108, 730663,
                9835848, 4555336,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                43732410, 34964877, 44855110, 54209249, 97976497, 49381408, 17693929, 34099128,
                55123565, 45977077,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                31117226, 21338698, 53606025, 6561946, 57231997, 20796761, 61990178, 29457725,
                29120152, 13924425,
            ]),
            xy2d: FieldElement2625::from_limbs([
                49707966, 19321222, 19675798, 30819676, 56101901, 27695611, 57724924, 22236731,
                7240930, 33317044,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                35747087, 22207651, 119210280, 27698212, 111764387, 54956091, 68331198, 37943914,
                70402500, 51557120,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                50424044, 19110186, 11038543, 11054958, 53307689, 30215898, 42789283, 7733546,
                12796905, 27218610,
            ]),
            xy2d: FieldElement2625::from_limbs([
                58349431, 22736595, 41689999, 10783768, 36493307, 23807620, 38855524, 3647835,
                3222231, 22393970,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                85714958, 35247531, 108769341, 51938590, 71221215, 43599452, 23603892, 31506198,
                59558087, 36039416,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                9255298, 30423235, 54952701, 32550175, 13098012, 24339566, 16377219, 31451620,
                47306788, 30519729,
            ]),
            xy2d: FieldElement2625::from_limbs([
                44379556, 7496159, 61366665, 11329248, 19991973, 30206930, 35390715, 9936965,
                37011176, 22935634,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                88987435, 28553134, 71447199, 47198328, 64071998, 13160959, 86817760, 5415496,
                59748361, 29445138,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                27736842, 10103576, 12500508, 8502413, 63695848, 23920873, 10436917, 32004156,
                43449720, 25422331,
            ]),
            xy2d: FieldElement2625::from_limbs([
                19492550, 21450067, 37426887, 32701801, 63900692, 12403436, 30066266, 8367329,
                13243957, 8709688,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                79123950, 36355692, 95306994, 10151020, 91926984, 28811298, 55914672, 27908697,
                72259831, 40828617,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                2831347, 21062286, 1478974, 6122054, 23825128, 20820846, 31097298, 6083058,
                31021603, 23760822,
            ]),
            xy2d: FieldElement2625::from_limbs([
                64578913, 31324785, 445612, 10720828, 53259337, 22048494, 43601132, 16354464,
                15067285, 19406725,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                74949787, 47592304, 100852864, 49488446, 66380650, 29911725, 88512851, 34612017,
                47729401, 21151211,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                915865, 17085158, 15608284, 24765302, 42751837, 6060029, 49737545, 8410996,
                59888403, 16527024,
            ]),
            xy2d: FieldElement2625::from_limbs([
                32922597, 32997445, 20336073, 17369864, 10903704, 28169945, 16957573, 52992,
                23834301, 6588044,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                32752011, 44787382, 70490858, 24839565, 22652987, 22810329, 17159698, 50243539,
                46794283, 32248439,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                62419196, 9166775, 41398568, 22707125, 11576751, 12733943, 7924251, 30802151,
                1976122, 26305405,
            ]),
            xy2d: FieldElement2625::from_limbs([
                21251203, 16309901, 64125849, 26771309, 30810596, 12967303, 156041, 30183180,
                12331344, 25317235,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                75760459, 29077399, 118132091, 28557436, 80111370, 36505236, 96163290, 28447461,
                77116999, 28886530,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                31486061, 15114593, 52847614, 12951353, 14369431, 26166587, 16347320, 19892343,
                8684154, 23021480,
            ]),
            xy2d: FieldElement2625::from_limbs([
                19443825, 11385320, 24468943, 23895364, 43189605, 2187568, 40845657, 27467510,
                31316347, 14219878,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                38514355, 1193784, 99354083, 11392484, 31092169, 49277233, 94254877, 40546840,
                29126554, 42761822,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                32382916, 1110093, 18477781, 11028262, 39697101, 26006320, 62128346, 10843781,
                59151264, 19118701,
            ]),
            xy2d: FieldElement2625::from_limbs([
                2814918, 7836403, 27519878, 25686276, 46214848, 22000742, 45614304, 8550129,
                28346258, 1994730,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                47530546, 41639976, 53108344, 29605809, 69894701, 17323124, 47591912, 40729325,
                22628101, 41669612,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                36703732, 955510, 55975026, 18476362, 34661776, 20276352, 41457285, 3317159,
                57165847, 930271,
            ]),
            xy2d: FieldElement2625::from_limbs([
                51805164, 26720662, 28856489, 1357446, 23421993, 1057177, 24091212, 32165462,
                44343487, 22903716,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                44357614, 28250434, 54201256, 54339997, 51297351, 25757378, 52269845, 50554643,
                65241844, 41953401,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                35139535, 2106402, 62372504, 1362500, 12813763, 16200670, 22981545, 27263159,
                18009407, 17781660,
            ]),
            xy2d: FieldElement2625::from_limbs([
                49887941, 24009210, 39324209, 14166834, 29815394, 7444469, 29551787, 29827013,
                19288548, 1325865,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                82209002, 51273111, 110293748, 32549332, 107767535, 49063838, 79485593, 30075285,
                100274970, 25511681,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                20909212, 13023121, 57899112, 16251777, 61330449, 25459517, 12412150, 10018715,
                2213263, 19676059,
            ]),
            xy2d: FieldElement2625::from_limbs([
                32529814, 22479743, 30361438, 16864679, 57972923, 1513225, 22922121, 6382134,
                61341936, 8371347,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                77032307, 44825931, 79725657, 37099153, 104219359, 31832804, 12891686, 25361300,
                40665920, 44040575,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                44511638, 26541766, 8587002, 25296571, 4084308, 20584370, 361725, 2610596,
                43187334, 22099236,
            ]),
            xy2d: FieldElement2625::from_limbs([
                5408392, 32417741, 62139741, 10561667, 24145918, 14240566, 31319731, 29318891,
                19985174, 30118346,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                53114388, 50171252, 81658109, 36895530, 99264821, 13648975, 49531796, 8849296,
                67173894, 41925115,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                58787919, 21504805, 31204562, 5839400, 46481576, 32497154, 47665921, 6922163,
                12743482, 23753914,
            ]),
            xy2d: FieldElement2625::from_limbs([
                64747493, 12678784, 28815050, 4759974, 43215817, 4884716, 23783145, 11038569,
                18800704, 255233,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                61839168, 31780545, 13957885, 41545147, 23132994, 34283205, 80502710, 42621388,
                86367551, 52355070,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                64172210, 22726896, 56676774, 14516792, 63468078, 4372540, 35173943, 2209389,
                65584811, 2055793,
            ]),
            xy2d: FieldElement2625::from_limbs([
                580882, 16705327, 5468415, 30871414, 36182444, 18858431, 59905517, 24560042,
                37087844, 7394434,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                90947654, 35377159, 118479284, 48797157, 75426955, 29821327, 45436683, 30062226,
                62287122, 48354352,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                13345610, 9759151, 3371034, 17416641, 16353038, 8577942, 31129804, 13496856,
                58052846, 7402517,
            ]),
            xy2d: FieldElement2625::from_limbs([
                2286874, 29118501, 47066405, 31546095, 53412636, 5038121, 11006906, 17794080,
                8205060, 1607563,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                81522931, 25552299, 70440693, 63900646, 89358013, 27960243, 85473524, 30647473,
                30019586, 24525154,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                39420813, 1585952, 56333811, 931068, 37988643, 22552112, 52698034, 12029092,
                9944378, 8024,
            ]),
            xy2d: FieldElement2625::from_limbs([
                4368715, 29844802, 29874199, 18531449, 46878477, 22143727, 50994269, 32555346,
                58966475, 5640029,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                77408455, 13746482, 11661824, 16234854, 74739102, 5998373, 76918751, 16859867,
                82328661, 19226648,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                27425505, 27835351, 3055005, 10660664, 23458024, 595578, 51710259, 32381236,
                48766680, 9742716,
            ]),
            xy2d: FieldElement2625::from_limbs([
                6744077, 2427284, 26042789, 2720740, 66260958, 1118973, 32324614, 7406442,
                12420155, 1994844,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                81121366, 62084143, 115833273, 23975961, 107732385, 29617991, 121184249, 22644627,
                91428792, 27108098,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                16412671, 29047065, 10772640, 15929391, 50040076, 28895810, 10555944, 23070383,
                37006495, 28815383,
            ]),
            xy2d: FieldElement2625::from_limbs([
                22397363, 25786748, 57815702, 20761563, 17166286, 23799296, 39775798, 6199365,
                21880021, 21303672,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                62825538, 5368522, 35991846, 41717820, 103894664, 36763558, 83666014, 42445160,
                75949308, 38512191,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                51661137, 709326, 60189418, 22684253, 37330941, 6522331, 45388683, 12130071,
                52312361, 5005756,
            ]),
            xy2d: FieldElement2625::from_limbs([
                64994094, 19246303, 23019041, 15765735, 41839181, 6002751, 10183197, 20315106,
                50713577, 31378319,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                115191953, 35186435, 80575154, 59113763, 110577275, 16573535, 35094956, 30497327,
                22208661, 35554900,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                3065054, 32141671, 41510189, 33192999, 49425798, 27851016, 58944651, 11248526,
                63417650, 26140247,
            ]),
            xy2d: FieldElement2625::from_limbs([
                10379208, 27508878, 8877318, 1473647, 37817580, 21046851, 16690914, 2553332,
                63976176, 16400288,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                82825513, 34808697, 115745037, 41000704, 58659945, 6344163, 45011593, 26268851,
                26894936, 42686498,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                24158868, 12938817, 11085297, 25376834, 39045385, 29097348, 36532400, 64451,
                60291780, 30861549,
            ]),
            xy2d: FieldElement2625::from_limbs([
                13488534, 7794716, 22236231, 5989356, 25426474, 20976224, 2350709, 30135921,
                62420857, 2364225,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                83443897, 9132433, 92749446, 40233319, 68834491, 42072368, 55301839, 21856974,
                15445874, 25756331,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                29004188, 25687351, 28661401, 32914020, 54314860, 25611345, 31863254, 29418892,
                66830813, 17795152,
            ]),
            xy2d: FieldElement2625::from_limbs([
                60986784, 18687766, 38493958, 14569918, 56250865, 29962602, 10343411, 26578142,
                37280576, 22738620,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                94190495, 37018415, 14099041, 29036828, 68725166, 27348827, 96651499, 15372178,
                84402661, 34515140,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                20263915, 11434237, 61343429, 11236809, 13505955, 22697330, 50997518, 6493121,
                47724353, 7639713,
            ]),
            xy2d: FieldElement2625::from_limbs([
                64278047, 18715199, 25403037, 25339236, 58791851, 17380732, 18006286, 17510682,
                29994676, 17746311,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                76878673, 38757082, 110060329, 19923038, 106166724, 21992806, 42495722, 53248081,
                35924287, 34263895,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                12286395, 13076066, 45333675, 32377809, 42105665, 4057651, 35090736, 24663557,
                16102006, 13205847,
            ]),
            xy2d: FieldElement2625::from_limbs([
                13733362, 5599946, 10557076, 3195751, 61550873, 8536969, 41568694, 8525971,
                10151379, 10394400,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                71133505, 17416880, 89545125, 12276533, 58009849, 64422764, 86807091, 11743038,
                100915394, 42488844,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                51229064, 29029191, 58528116, 30620370, 14634844, 32856154, 57659786, 3137093,
                55571978, 11721157,
            ]),
            xy2d: FieldElement2625::from_limbs([
                17555920, 28540494, 8268605, 2331751, 44370049, 9761012, 9319229, 8835153,
                57903375, 32274386,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                66647436, 25724417, 87722981, 16688287, 59594098, 28747312, 89409167, 34059860,
                73217325, 27371016,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                62038564, 12367916, 36445330, 3234472, 32617080, 25131790, 29880582, 20071101,
                40210373, 25686972,
            ]),
            xy2d: FieldElement2625::from_limbs([
                35133562, 5726538, 26934134, 10237677, 63935147, 32949378, 24199303, 3795095,
                7592688, 18562353,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                21594413, 18590204, 84575271, 63031641, 32537082, 36294330, 73516586, 12018832,
                38852812, 37852843,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                46458361, 21592935, 39872588, 570497, 3767144, 31836892, 13891941, 31985238,
                13717173, 10805743,
            ]),
            xy2d: FieldElement2625::from_limbs([
                52432215, 17910135, 15287173, 11927123, 24177847, 25378864, 66312432, 14860608,
                40169934, 27690595,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                80071405, 38866230, 57048095, 45212711, 85964149, 25600230, 80395126, 54300159,
                62727806, 9882021,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                18512060, 11319350, 46985740, 15090308, 18818594, 5271736, 44380960, 3666878,
                43141434, 30255002,
            ]),
            xy2d: FieldElement2625::from_limbs([
                60319844, 30408388, 16192428, 13241070, 15898607, 19348318, 57023983, 26893321,
                64705764, 5276064,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                97278672, 28236783, 93415069, 55358004, 94923826, 40623698, 74261714, 37239413,
                68558087, 13082860,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                10342807, 3098505, 2119311, 193222, 25702612, 12233820, 23697382, 15056736,
                46092426, 25352431,
            ]),
            xy2d: FieldElement2625::from_limbs([
                33958735, 3261607, 22745853, 7948688, 19370557, 18376767, 40936887, 6482813,
                56808784, 22494330,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                32869439, 61700319, 25609741, 49233102, 56421094, 51637792, 26112419, 36075440,
                44444575, 40459246,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                29506904, 4457497, 3377935, 23757988, 36598817, 12935079, 1561737, 3841096,
                38105225, 26896789,
            ]),
            xy2d: FieldElement2625::from_limbs([
                10340844, 26924055, 48452231, 31276001, 12621150, 20215377, 30878496, 21730062,
                41524312, 5181965,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                25940096, 20896407, 17324187, 56801490, 58437394, 15029093, 91505116, 17103509,
                64786011, 21165857,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                45343161, 9916822, 65808455, 4079497, 66080518, 11909558, 1782390, 12641087,
                20603771, 26992690,
            ]),
            xy2d: FieldElement2625::from_limbs([
                48226577, 21881051, 24849421, 11501709, 13161720, 28785558, 1925522, 11914390,
                4662781, 7820689,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                79349895, 33128449, 75241554, 42948365, 32846759, 31954812, 29749455, 45727356,
                83245615, 48818451,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                56758909, 18873868, 58896884, 2330219, 49446315, 19008651, 10658212, 6671822,
                19012087, 3772772,
            ]),
            xy2d: FieldElement2625::from_limbs([
                3753511, 30133366, 10617073, 2028709, 14841030, 26832768, 28718731, 17791548,
                20527770, 12988982,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                52286341, 27757162, 63400876, 12689772, 66209881, 22639565, 110034681, 56543919,
                70408527, 54683910,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                50331161, 18301130, 57466446, 4978982, 3308785, 8755439, 6943197, 6461331,
                41525717, 8991217,
            ]),
            xy2d: FieldElement2625::from_limbs([
                49882601, 1816361, 65435576, 27467992, 31783887, 25378441, 34160718, 7417949,
                36866577, 1507264,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                29692644, 40384323, 56610063, 37889327, 88054838, 21647935, 38221255, 41763822,
                14606361, 22907359,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                63627275, 8707080, 32188102, 5672294, 22096700, 1711240, 34088169, 9761486,
                4170404, 31469107,
            ]),
            xy2d: FieldElement2625::from_limbs([
                55521375, 14855944, 62981086, 32022574, 40459774, 15084045, 22186522, 16002000,
                52832027, 25153633,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                62297389, 47315460, 35404986, 31070512, 63796392, 41423478, 59995291, 23934339,
                80349708, 44520301,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                59366301, 25297669, 52340529, 19898171, 43876480, 12387165, 4498947, 14147411,
                29514390, 4302863,
            ]),
            xy2d: FieldElement2625::from_limbs([
                53695440, 21146572, 20757301, 19752600, 14785142, 8976368, 62047588, 31410058,
                17846987, 19582505,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                64864393, 32799703, 62511833, 32488122, 60861691, 35009730, 112569999, 24339641,
                61886162, 46204698,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                57202067, 17484121, 21134159, 12198166, 40044289, 708125, 387813, 13770293,
                47974538, 10958662,
            ]),
            xy2d: FieldElement2625::from_limbs([
                22470984, 12369526, 23446014, 28113323, 45588061, 23855708, 55336367, 21979976,
                42025033, 4271861,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                109048144, 57055220, 47199530, 48916026, 61124505, 35713623, 67184238, 62830334,
                101691505, 42024103,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                15854951, 4148314, 58214974, 7259001, 11666551, 13824734, 36577666, 2697371,
                24154791, 24093489,
            ]),
            xy2d: FieldElement2625::from_limbs([
                15446137, 17747788, 29759746, 14019369, 30811221, 23944241, 35526855, 12840103,
                24913809, 9815020,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                62399559, 27940162, 35267365, 21265538, 52665326, 44353845, 125114051, 46993199,
                85843991, 43020669,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                11933045, 9281483, 5081055, 28370608, 64480701, 28648802, 59381042, 22658328,
                44380208, 16199063,
            ]),
            xy2d: FieldElement2625::from_limbs([
                14576810, 379472, 40322331, 25237195, 37682355, 22741457, 67006097, 1876698,
                30801119, 2164795,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                15995067, 36754305, 13672554, 13712240, 47730029, 62461217, 121136116, 51612593,
                53616055, 34822483,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                56818250, 29895392, 63822271, 10948817, 23037027, 3794475, 63638526, 20954210,
                50053494, 3565903,
            ]),
            xy2d: FieldElement2625::from_limbs([
                29210069, 24135095, 61189071, 28601646, 10834810, 20226706, 50596761, 22733718,
                39946641, 19523900,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                121055819, 49063018, 83772567, 25398281, 38758921, 42573554, 37925442, 29785008,
                69352974, 19552452,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                61955989, 29753495, 57802388, 27482848, 16243068, 14684434, 41435776, 17373631,
                13491505, 4641841,
            ]),
            xy2d: FieldElement2625::from_limbs([
                10813398, 643330, 47920349, 32825515, 30292061, 16954354, 27548446, 25833190,
                14476988, 20787001,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                77400943, 9984944, 73590300, 41834336, 59857349, 40587174, 27282936, 31910173,
                106304917, 12651322,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                35923332, 32741048, 22271203, 11835308, 10201545, 15351028, 17099662, 3988035,
                21721536, 30405492,
            ]),
            xy2d: FieldElement2625::from_limbs([
                10202177, 27008593, 35735631, 23979793, 34958221, 25434748, 54202543, 3852693,
                13216206, 14842320,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                51293205, 22953365, 60569911, 26295436, 60124204, 26972653, 35608016, 47320255,
                106783330, 43454614,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                14465486, 19721101, 34974879, 18815558, 39665676, 12990491, 33046193, 15796406,
                60056998, 25514317,
            ]),
            xy2d: FieldElement2625::from_limbs([
                30924398, 25274812, 6359015, 20738097, 16508376, 9071735, 41620263, 15413634,
                9524356, 26535554,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                12274182, 20378885, 99736504, 65323537, 73845487, 13267304, 72346523, 28444948,
                82772379, 37590215,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                64157555, 8903984, 17349946, 601635, 50676049, 28941875, 53376124, 17665097,
                44850385, 4659090,
            ]),
            xy2d: FieldElement2625::from_limbs([
                50192582, 28601458, 36715152, 18395610, 20774811, 15897498, 5736189, 15026997,
                64930608, 20098846,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                58249865, 31335375, 28571665, 56953346, 66634395, 23448733, 63307367, 33832526,
                23440561, 33264224,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                10226222, 27625730, 15139955, 120818, 52241171, 5218602, 32937275, 11551483,
                50536904, 26111567,
            ]),
            xy2d: FieldElement2625::from_limbs([
                17932739, 21117156, 43069306, 10749059, 11316803, 7535897, 22503767, 5561594,
                63462240, 3898660,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                74858752, 32584864, 50769132, 33537967, 42090752, 15122142, 65535333, 40706961,
                88940025, 34799664,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                26958440, 18896406, 4314585, 8346991, 61431100, 11960071, 34519569, 32934396,
                36706772, 16838219,
            ]),
            xy2d: FieldElement2625::from_limbs([
                54942968, 9166946, 33491384, 13673479, 29787085, 13096535, 6280834, 14587357,
                44770839, 13987524,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                109867800, 7778773, 88224864, 49127028, 62275597, 28196653, 62807965, 28429792,
                59639082, 30696363,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                9681908, 26817309, 35157219, 13591837, 60225043, 386949, 31622781, 6439245,
                52527852, 4091396,
            ]),
            xy2d: FieldElement2625::from_limbs([
                58682418, 1470726, 38999185, 31957441, 3978626, 28430809, 47486180, 12092162,
                29077877, 18812444,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                72378032, 26694705, 120987516, 25533715, 25932562, 35317984, 61502753, 28048550,
                47091016, 2357888,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                32264008, 18146780, 61721128, 32394338, 65017541, 29607531, 23104803, 20684524,
                5727337, 189038,
            ]),
            xy2d: FieldElement2625::from_limbs([
                14609104, 24599962, 61108297, 16931650, 52531476, 25810533, 40363694, 10942114,
                41219933, 18669734,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                87622345, 39112362, 51504250, 41383962, 93522806, 31535027, 45729895, 41026212,
                13913676, 28416557,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                41534488, 11967825, 29233242, 12948236, 60354399, 4713226, 58167894, 14059179,
                12878652, 8511905,
            ]),
            xy2d: FieldElement2625::from_limbs([
                41452044, 3393630, 64153449, 26478905, 64858154, 9366907, 36885446, 6812973,
                5568676, 30426776,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                78738868, 12144453, 69225203, 47160468, 94487748, 49231348, 49700110, 20050058,
                119822531, 8070816,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                27117677, 23547054, 35826092, 27984343, 1127281, 12772488, 37262958, 10483305,
                55556115, 32525717,
            ]),
            xy2d: FieldElement2625::from_limbs([
                10637467, 27866368, 5674780, 1072708, 40765276, 26572129, 65424888, 9177852,
                39615702, 15431202,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                87633990, 44446997, 121475255, 12779441, 104724694, 16150073, 105977209, 14943140,
                52052074, 25618500,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                37084402, 5626925, 66557297, 23573344, 753597, 11981191, 25244767, 30314666,
                63752313, 9594023,
            ]),
            xy2d: FieldElement2625::from_limbs([
                43356201, 2636869, 61944954, 23450613, 585133, 7877383, 11345683, 27062142,
                13352334, 22577348,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                65177046, 28146973, 70413512, 54223994, 84124668, 62231772, 104433876, 25801948,
                53893326, 33235227,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                20239939, 6607058, 6203985, 3483793, 48721888, 32775202, 46385121, 15077869,
                44358105, 14523816,
            ]),
            xy2d: FieldElement2625::from_limbs([
                27406023, 27512775, 27423595, 29057038, 4996213, 10002360, 38266833, 29008937,
                36936121, 28748764,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                78483087, 12660714, 17861383, 21013599, 78044431, 34653658, 53222787, 24462691,
                106490683, 44912934,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                54378055, 10311866, 1510375, 10778093, 64989409, 24408729, 32676002, 11149336,
                40985213, 4985767,
            ]),
            xy2d: FieldElement2625::from_limbs([
                48012542, 341146, 60911379, 33315398, 15756972, 24757770, 66125820, 13794113,
                47694557, 17933176,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                73598907, 45494717, 25495922, 59382504, 75777235, 24803115, 70476466, 40524436,
                65417798, 58104073,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                1656478, 13457317, 15370807, 6364910, 13605745, 8362338, 47934242, 28078708,
                50312267, 28522993,
            ]),
            xy2d: FieldElement2625::from_limbs([
                44835530, 20030007, 67044178, 29220208, 48503227, 22632463, 46537798, 26546453,
                67009010, 23317098,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                84856310, 43593691, 86477162, 29503840, 46478228, 51067577, 99101545, 17696455,
                104957364, 28042459,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                31932008, 28568291, 47496481, 16366579, 22023614, 88450, 11371999, 29810185,
                4882241, 22927527,
            ]),
            xy2d: FieldElement2625::from_limbs([
                29796488, 37186, 19818052, 10115756, 55279832, 3352735, 18551198, 3272828,
                61917932, 29392022,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                12501267, 4044383, 58495907, 53716478, 101787674, 38691029, 47878485, 30024734,
                330069, 29895023,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                6384877, 2899513, 17807477, 7663917, 64749976, 12363164, 25366522, 24980540,
                66837568, 12071498,
            ]),
            xy2d: FieldElement2625::from_limbs([
                58743349, 29511910, 25133447, 29037077, 60897836, 2265926, 34339246, 1936674,
                61949167, 3829362,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                28425947, 27718999, 66531773, 28857233, 120000172, 40425360, 75030413, 26986644,
                26333139, 47822096,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                56041645, 11871230, 27385719, 22994888, 62522949, 22365119, 10004785, 24844944,
                45347639, 8930323,
            ]),
            xy2d: FieldElement2625::from_limbs([
                45911060, 17158396, 25654215, 31829035, 12282011, 11008919, 1541940, 4757911,
                40617363, 17145491,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                80646107, 25794941, 113612887, 44516357, 61186043, 20336366, 53952279, 39771685,
                118274028, 47369420,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                49686272, 15157789, 18705543, 29619, 24409717, 33293956, 27361680, 9257833,
                65152338, 31777517,
            ]),
            xy2d: FieldElement2625::from_limbs([
                42063564, 23362465, 15366584, 15166509, 54003778, 8423555, 37937324, 12361134,
                48422886, 4578289,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                91688613, 3711569, 68451186, 22374305, 107212592, 47679386, 44564334, 14074918,
                21964432, 41789689,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                60580251, 31142934, 9442965, 27628844, 12025639, 32067012, 64127349, 31885225,
                13006805, 2355433,
            ]),
            xy2d: FieldElement2625::from_limbs([
                50803946, 19949172, 60476436, 28412082, 16974358, 22643349, 27202043, 1719366,
                1141648, 20758196,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                54244901, 53888877, 58790596, 56090772, 60298717, 28710537, 13475065, 30420460,
                32674894, 47269477,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                11423316, 28086373, 32344215, 8962751, 24989809, 9241752, 53843611, 16086211,
                38367983, 17912338,
            ]),
            xy2d: FieldElement2625::from_limbs([
                65699196, 12530727, 60740138, 10847386, 19531186, 19422272, 55399715, 7791793,
                39862921, 4383346,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                38137947, 38825878, 65842854, 23817442, 121762491, 50287029, 62246456, 62202414,
                27193555, 39799623,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                51914908, 5362277, 65324971, 2695833, 4960227, 12840725, 23061898, 3260492,
                22510453, 8577507,
            ]),
            xy2d: FieldElement2625::from_limbs([
                54476394, 11257345, 34415870, 13548176, 66387860, 10879010, 31168030, 13952092,
                37537372, 29918525,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                70986166, 23981692, 99525555, 38959755, 56104456, 19897796, 70868632, 45489751,
                72720723, 41718449,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                50833043, 14667796, 15906460, 12155291, 44997715, 24514713, 32003001, 24722143,
                5773084, 25132323,
            ]),
            xy2d: FieldElement2625::from_limbs([
                43320746, 25300131, 1950874, 8937633, 18686727, 16459170, 66203139, 12376319,
                31632953, 190926,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                109624102, 17415545, 58684872, 13378745, 81271271, 6901327, 58820115, 38062995,
                41767308, 29926903,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                8884438, 27670423, 6023973, 10104341, 60227295, 28612898, 18722940, 18768427,
                65436375, 827624,
            ]),
            xy2d: FieldElement2625::from_limbs([
                34388281, 17265135, 34605316, 7101209, 13354605, 2659080, 65308289, 19446395,
                42230385, 1541285,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                70010192, 32436744, 70989239, 57049475, 116596786, 29941649, 45306746, 29986950,
                87565708, 31669398,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                27019610, 12299467, 53450576, 31951197, 54247203, 28692960, 47568713, 28538373,
                29439640, 15138866,
            ]),
            xy2d: FieldElement2625::from_limbs([
                21536104, 26928012, 34661045, 22864223, 44700786, 5175813, 61688824, 17193268,
                7779327, 109896,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                97388589, 48203181, 59063992, 39979989, 80748484, 32810922, 28698389, 45734550,
                23177718, 33000357,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                26572828, 3405927, 35407164, 12890904, 47843196, 5335865, 60615096, 2378491,
                4439158, 20275085,
            ]),
            xy2d: FieldElement2625::from_limbs([
                44392139, 3489069, 57883598, 33221678, 18875721, 32414337, 14819433, 20822905,
                49391106, 28092994,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                62052362, 50120982, 83062524, 37322183, 56672364, 49181491, 66287909, 35731656,
                75658945, 18440266,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                48635543, 16596774, 66727204, 15663610, 22860960, 15585581, 39264755, 29971692,
                43848403, 25125843,
            ]),
            xy2d: FieldElement2625::from_limbs([
                34628313, 15707274, 58902952, 27902350, 29464557, 2713815, 44383727, 15860481,
                45206294, 1494192,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                47546754, 53021470, 41524990, 24254879, 80236705, 34314140, 21923481, 16529112,
                75851568, 46521448,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                38643965, 1553204, 32536856, 23080703, 42417258, 33148257, 58194238, 30620535,
                37205105, 15553882,
            ]),
            xy2d: FieldElement2625::from_limbs([
                21877890, 3230008, 9881174, 10539357, 62311749, 2841331, 11543572, 14513274,
                19375923, 20906471,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                75941133, 52613378, 80362373, 38692006, 72146734, 37633208, 24880817, 60886148,
                69971515, 9455042,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                29306751, 5123106, 20245049, 19404543, 9592565, 8447059, 65031740, 30564351,
                15511448, 4789663,
            ]),
            xy2d: FieldElement2625::from_limbs([
                46429108, 7004546, 8824831, 24119455, 63063159, 29803695, 61354101, 108892,
                23513200, 16652362,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                100961536, 37699212, 62632834, 26975308, 77878902, 26398889, 60458447, 54172563,
                115898528, 43767290,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                2756062, 8598110, 7383731, 26694540, 22312758, 32449420, 21179800, 2600940,
                57120566, 21047965,
            ]),
            xy2d: FieldElement2625::from_limbs([
                42463153, 13317461, 36659605, 17900503, 21365573, 22684775, 11344423, 864440,
                64609187, 16844368,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                107784906, 6148327, 49924452, 19080277, 85891792, 33278434, 44547329, 33765731,
                69828620, 38495428,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                65784982, 3911312, 60160120, 14759764, 37081714, 7851206, 21690126, 8518463,
                26699843, 5276295,
            ]),
            xy2d: FieldElement2625::from_limbs([
                53958991, 27125364, 9396248, 365013, 24703301, 23065493, 1321585, 149635, 51656090,
                7159368,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                77096625, 30149672, 84616825, 43059961, 76840398, 31388917, 89464872, 41866607,
                89586081, 25151046,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                18155857, 17049442, 19744715, 9006923, 15154154, 23015456, 24256459, 28689437,
                44560690, 9334108,
            ]),
            xy2d: FieldElement2625::from_limbs([
                2986088, 28642539, 10776627, 30080588, 10620589, 26471229, 45695018, 14253544,
                44521715, 536905,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                71486582, 41670267, 91675941, 15495313, 78733938, 46619030, 74499414, 44144056,
                77946923, 51688439,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                47766460, 867879, 9277171, 30335973, 52677291, 31567988, 19295825, 17757482,
                6378259, 699185,
            ]),
            xy2d: FieldElement2625::from_limbs([
                7895007, 4057113, 60027092, 20476675, 49222032, 33231305, 66392824, 15693154,
                62063800, 20180469,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                59371282, 27685029, 119651408, 26147511, 78494517, 46756047, 31730677, 22591592,
                63190227, 23885106,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                10188286, 17783598, 59772502, 13427542, 22223443, 14896287, 30743455, 7116568,
                45322357, 5427592,
            ]),
            xy2d: FieldElement2625::from_limbs([
                696102, 13206899, 27047647, 22922350, 15285304, 23701253, 10798489, 28975712,
                19236242, 12477404,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                55879406, 44798227, 50054593, 25513566, 66320635, 58940896, 63211193, 44734935,
                43939347, 41288075,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                17800790, 19518253, 40108434, 21787760, 23887826, 3149671, 23466177, 23016261,
                10322026, 15313801,
            ]),
            xy2d: FieldElement2625::from_limbs([
                26246234, 11968874, 32263343, 28085704, 6830754, 20231401, 51314159, 33452449,
                42659621, 10890803,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                35743198, 43825794, 54448238, 27287163, 83799070, 54046319, 119235514, 50039361,
                92289660, 28219547,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                66522290, 10376443, 34522450, 22268075, 19801892, 10997610, 2276632, 9482883,
                316878, 13820577,
            ]),
            xy2d: FieldElement2625::from_limbs([
                57226037, 29044064, 64993357, 16457135, 56008783, 11674995, 30756178, 26039378,
                30696929, 29841583,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                100097781, 23951019, 12499365, 41465219, 56491606, 21622917, 59766047, 57123466,
                34759345, 7392472,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                58253184, 15927860, 9866406, 29905021, 64711949, 16898650, 36699387, 24419436,
                25112946, 30627788,
            ]),
            xy2d: FieldElement2625::from_limbs([
                64604801, 33117465, 25621773, 27875660, 15085041, 28074555, 42223985, 20028237,
                5537437, 19640113,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                55883261, 2320284, 57524584, 10149186, 100773065, 5808646, 119341477, 31824763,
                98343453, 39645030,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                57475529, 116425, 26083934, 2897444, 60744427, 30866345, 609720, 15878753,
                60138459, 24519663,
            ]),
            xy2d: FieldElement2625::from_limbs([
                39351007, 247743, 51914090, 24551880, 23288160, 23542496, 43239268, 6503645,
                20650474, 1804084,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                106627923, 49010854, 76081380, 42024039, 82749485, 37994278, 70230858, 56779150,
                94951478, 33352103,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                51801891, 2839643, 22530074, 10026331, 4602058, 5048462, 28248656, 5031932,
                55733782, 12714368,
            ]),
            xy2d: FieldElement2625::from_limbs([
                20807691, 26283607, 29286140, 11421711, 39232341, 19686201, 45881388, 1035545,
                47375635, 12796919,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                79185725, 52807577, 58323861, 21705509, 42096072, 49955115, 49517368, 20654993,
                70589528, 51926048,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                34747315, 5457596, 28548107, 7833186, 7303070, 21600887, 42745799, 17632556,
                33734809, 2771024,
            ]),
            xy2d: FieldElement2625::from_limbs([
                45719598, 421931, 26597266, 6860826, 22486084, 26817260, 49971378, 29344205,
                42556581, 15673396,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                46924223, 35892647, 19788684, 57487908, 63107597, 24813538, 46837679, 38287685,
                70836007, 20619983,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                6120100, 814863, 55314462, 32931715, 6812204, 17806661, 2019593, 7975683, 31123697,
                22595451,
            ]),
            xy2d: FieldElement2625::from_limbs([
                30069250, 22119100, 30434653, 2958439, 18399564, 32578143, 12296868, 9204260,
                50676426, 9648164,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                32705413, 32003455, 97814521, 41005496, 55303257, 43186244, 70414129, 38803035,
                108209395, 22176929,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                17219846, 2375039, 35537917, 27978816, 47649184, 9219902, 294711, 15298639,
                2662509, 17257359,
            ]),
            xy2d: FieldElement2625::from_limbs([
                65935918, 25995736, 62742093, 29266687, 45762450, 25120105, 32087528, 32331655,
                32247247, 19164571,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                14312609, 34775988, 17395389, 58408721, 62163121, 58424228, 106019982, 23916613,
                51081240, 20175586,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                65680039, 23875441, 57873182, 6549686, 59725795, 33085767, 23046501, 9803137,
                17597934, 2346211,
            ]),
            xy2d: FieldElement2625::from_limbs([
                18510781, 15337574, 26171504, 981392, 44867312, 7827555, 43617730, 22231079,
                3059832, 21771562,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                77250443, 39637338, 84938156, 31606788, 76938955, 13613135, 41552228, 28009845,
                33606651, 37146527,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                33114149, 17665080, 40583177, 20211034, 33076704, 8716171, 1151462, 1521897,
                66126199, 26716628,
            ]),
            xy2d: FieldElement2625::from_limbs([
                34169699, 29298616, 23947180, 33230254, 34035889, 21248794, 50471177, 3891703,
                26353178, 693168,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                97483084, 35150011, 117333688, 46741361, 71709207, 33961335, 76694157, 33153763,
                31375463, 47924397,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                52738210, 25781902, 1510300, 6434173, 48324075, 27291703, 32732229, 20445593,
                17901440, 16011505,
            ]),
            xy2d: FieldElement2625::from_limbs([
                18171223, 21619806, 54608461, 15197121, 56070717, 18324396, 47936623, 17508055,
                8764034, 12309598,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                73084753, 28311243, 47649501, 23872684, 55567586, 14015781, 110551971, 34782749,
                17544095, 22960650,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                5811932, 31839139, 3442886, 31285122, 48741515, 25194890, 49064820, 18144304,
                61543482, 12348899,
            ]),
            xy2d: FieldElement2625::from_limbs([
                35709185, 11407554, 25755363, 6891399, 63851926, 14872273, 42259511, 8141294,
                56476330, 32968952,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                121542424, 34248456, 62032718, 46854775, 81124121, 19103037, 124519055, 22225380,
                30944592, 1130208,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                8247747, 26843490, 40546482, 25845122, 52706924, 18905521, 4652151, 2488540,
                23550156, 33283200,
            ]),
            xy2d: FieldElement2625::from_limbs([
                17294297, 29765994, 7026747, 15626851, 22990044, 113481, 2267737, 27646286,
                66700045, 33416712,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                83199930, 17300505, 85708115, 40895109, 69246500, 32332774, 63744702, 48105367,
                70369388, 26388160,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                62198760, 20221544, 18550886, 10864893, 50649539, 26262835, 44079994, 20349526,
                54360141, 2701325,
            ]),
            xy2d: FieldElement2625::from_limbs([
                58534169, 16099414, 4629974, 17213908, 46322650, 27548999, 57090500, 9276970,
                11329923, 1862132,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                14763057, 17650824, 103299457, 3689865, 70620756, 43867957, 45157775, 45773662,
                58070900, 32614131,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                8894987, 30108338, 6150752, 3013931, 301220, 15693451, 35127648, 30644714,
                51670695, 11595569,
            ]),
            xy2d: FieldElement2625::from_limbs([
                15214943, 3537601, 40870142, 19495559, 4418656, 18323671, 13947275, 10730794,
                53619402, 29190761,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                64570539, 41237224, 99867876, 33817540, 104232996, 25598978, 111885603, 23365795,
                68085971, 34254425,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                54642373, 4195083, 57897332, 550903, 51543527, 12917919, 19118110, 33114591,
                36574330, 19216518,
            ]),
            xy2d: FieldElement2625::from_limbs([
                31788442, 19046775, 4799988, 7372237, 8808585, 18806489, 9408236, 23502657,
                12493931, 28145115,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                41428258, 5260743, 47873055, 27269961, 63412921, 16566086, 94327144, 36161552,
                29375954, 6024730,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                842132, 30759739, 62345482, 24831616, 26332017, 21148791, 11831879, 6985184,
                57168503, 2854095,
            ]),
            xy2d: FieldElement2625::from_limbs([
                62261602, 25585100, 2516241, 27706719, 9695690, 26333246, 16512644, 960770,
                12121869, 16648078,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                51890193, 48221527, 53772634, 35568148, 97707150, 33090294, 35603941, 25672367,
                20237805, 36392843,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                47820798, 4453151, 15298546, 17376044, 22115042, 17581828, 12544293, 20083975,
                1068880, 21054527,
            ]),
            xy2d: FieldElement2625::from_limbs([
                57549981, 17035596, 33238497, 13506958, 30505848, 32439836, 58621956, 30924378,
                12521377, 4845654,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                106019188, 44298538, 64150483, 43754095, 74868174, 54020263, 70518210, 32681031,
                127735421, 20668560,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                43547042, 6230155, 46726851, 10655313, 43068279, 21933259, 10477733, 32314216,
                63995636, 13974497,
            ]),
            xy2d: FieldElement2625::from_limbs([
                12966261, 15550616, 35069916, 31939085, 21025979, 32924988, 5642324, 7188737,
                18895762, 12629579,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                14741860, 18607545, 89286071, 21833194, 68388604, 41613031, 11758139, 34343875,
                32195180, 37450109,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                10758205, 15755439, 62598914, 9243697, 62229442, 6879878, 64904289, 29988312,
                58126794, 4429646,
            ]),
            xy2d: FieldElement2625::from_limbs([
                64654951, 15725972, 46672522, 23143759, 61304955, 22514211, 59972993, 21911536,
                18047435, 18272689,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                41935825, 55801698, 29759954, 45331216, 111955344, 51288407, 78101976, 54258026,
                49488161, 57700395,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                21987233, 700364, 42603816, 14972007, 59334599, 27836036, 32155025, 2581431,
                37149879, 8773374,
            ]),
            xy2d: FieldElement2625::from_limbs([
                41540495, 454462, 53896929, 16126714, 25240068, 8594567, 20656846, 12017935,
                59234475, 19634276,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                73137027, 39817509, 103205921, 55807152, 66289943, 36016203, 102376553, 61640820,
                65387074, 30777706,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                54829870, 16624276, 987579, 27631834, 32908202, 1248608, 7719845, 29387734,
                28408819, 6816612,
            ]),
            xy2d: FieldElement2625::from_limbs([
                56750770, 25316602, 19549650, 21385210, 22082622, 16147817, 20613181, 13982702,
                56769294, 5067942,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                36602859, 29732664, 79183544, 13582411, 47230892, 35998382, 47389577, 12746131,
                72440074, 57002919,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                30528792, 3601899, 65151774, 4619784, 39747042, 18118043, 24180792, 20984038,
                27679907, 31905504,
            ]),
            xy2d: FieldElement2625::from_limbs([
                9402385, 19597367, 32834042, 10838634, 40528714, 20317236, 26653273, 24868867,
                22611443, 20839026,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                89299435, 34672460, 22736440, 48684895, 103757035, 27563109, 86298488, 62459921,
                71963721, 40176570,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                58798126, 30600981, 58846284, 30166382, 56707132, 33282502, 13424425, 29987205,
                26404408, 13001963,
            ]),
            xy2d: FieldElement2625::from_limbs([
                35867026, 18138731, 64114613, 8939345, 11562230, 20713762, 41044498, 21932711,
                51703708, 11020692,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                68974887, 59159374, 59210213, 23253421, 12483314, 47031979, 70284499, 21130268,
                28761761, 34961166,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                66660290, 31776765, 13018550, 3194501, 57528444, 22392694, 24760584, 29207344,
                25577410, 20175752,
            ]),
            xy2d: FieldElement2625::from_limbs([
                42818486, 4759344, 66418211, 31701615, 2066746, 10693769, 37513074, 9884935,
                57739938, 4745409,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                57967561, 39604145, 47577802, 29213020, 102956929, 43498706, 51646855, 55797011,
                78040786, 21622500,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                50547351, 14112679, 59096219, 4817317, 59068400, 22139825, 44255434, 10856640,
                46638094, 13434653,
            ]),
            xy2d: FieldElement2625::from_limbs([
                22759470, 23480998, 50342599, 31683009, 13637441, 23386341, 1765143, 20900106,
                28445306, 28189722,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                29875044, 46048045, 69904399, 63322533, 68819482, 48735613, 56913146, 24765756,
                9074233, 34721612,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                40903181, 11014232, 57266213, 30918946, 40200743, 7532293, 48391976, 24018933,
                3843902, 9367684,
            ]),
            xy2d: FieldElement2625::from_limbs([
                56139269, 27150720, 9591133, 9582310, 11349256, 108879, 16235123, 8601684,
                66969667, 4242894,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                89201818, 53917740, 65066069, 21585919, 99295616, 55591475, 60534521, 36025091,
                106800361, 16625499,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                56051142, 3042015, 13770083, 24296510, 584235, 33009577, 59338006, 2602724,
                39757248, 14247412,
            ]),
            xy2d: FieldElement2625::from_limbs([
                6314156, 23289540, 34336361, 15957556, 56951134, 168749, 58490057, 14290060,
                27108877, 32373552,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                58522248, 26383465, 80350645, 44514587, 34117848, 19759835, 100656839, 22495542,
                107069276, 34536304,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                22833421, 9293594, 34459416, 19935764, 57971897, 14756818, 44180005, 19583651,
                56629059, 17356469,
            ]),
            xy2d: FieldElement2625::from_limbs([
                59340277, 3326785, 38997067, 10783823, 19178761, 14905060, 22680049, 13906969,
                51175174, 3797898,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                88830182, 29341685, 54902740, 42864613, 63226624, 19901321, 90849087, 30845199,
                87600846, 59066711,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                9209251, 18419377, 53852306, 27386633, 66377847, 15289672, 25947805, 15286587,
                30997318, 26851369,
            ]),
            xy2d: FieldElement2625::from_limbs([
                7392013, 16618386, 23946583, 25514540, 53843699, 32020573, 52911418, 31232855,
                17649997, 33304352,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                57807757, 52915036, 97718388, 30504888, 41933794, 32270679, 51867297, 24028707,
                64875610, 41216577,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                49550191, 1763593, 33994528, 15908609, 37067994, 21380136, 7335079, 25082233,
                63934189, 3440182,
            ]),
            xy2d: FieldElement2625::from_limbs([
                47219164, 27577423, 42997570, 23865561, 10799742, 16982475, 40449, 29122597,
                4862399, 1133,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                34252636, 25680474, 61686474, 48415381, 50789832, 41510573, 74366924, 33866292,
                36513872, 26175010,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                63335436, 31988495, 28985339, 7499440, 24445838, 9325937, 29727763, 16527196,
                18278453, 15405622,
            ]),
            xy2d: FieldElement2625::from_limbs([
                62726958, 8508651, 47210498, 29880007, 61124410, 15149969, 53795266, 843522,
                45233802, 13626196,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                69390312, 20067376, 56193445, 30944521, 68988221, 49718638, 56324981, 37508223,
                80449702, 15928662,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                31727126, 26374577, 48671360, 25270779, 2875792, 17164102, 41838969, 26539605,
                43656557, 5964752,
            ]),
            xy2d: FieldElement2625::from_limbs([
                4100401, 27594980, 49929526, 6017713, 48403027, 12227140, 40424029, 11344143,
                2538215, 25983677,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                57675240, 6123112, 78268667, 31397823, 97125143, 48520672, 46633880, 35039852,
                66479607, 17595569,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                40304287, 4260918, 11851389, 9658551, 35091757, 16367491, 46903439, 20363143,
                11659921, 22439314,
            ]),
            xy2d: FieldElement2625::from_limbs([
                26180377, 10015009, 36264640, 24973138, 5418196, 9480663, 2231568, 23384352,
                33100371, 32248261,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                82229958, 28352560, 56718958, 48982252, 39598926, 17561924, 88779810, 38041106,
                61177053, 19088051,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                16166467, 24070699, 56004733, 6023907, 35182066, 32189508, 2340059, 17299464,
                56373093, 23514607,
            ]),
            xy2d: FieldElement2625::from_limbs([
                28042865, 29997343, 54982337, 12259705, 63391366, 26608532, 6766452, 24864833,
                18036435, 5803270,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                66291264, 40318343, 78912424, 35140016, 78067310, 30883266, 23855390, 4598332,
                60949433, 19436993,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                36077558, 19298237, 17332028, 31170912, 31312681, 27587249, 696308, 50292,
                47013125, 11763583,
            ]),
            xy2d: FieldElement2625::from_limbs([
                66514282, 31040148, 34874710, 12643979, 12650761, 14811489, 665117, 20940800,
                47335652, 22840869,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                97573435, 55845991, 62981386, 20819953, 86944190, 60003250, 109821551, 35630203,
                50088706, 34546902,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                18357166, 26559999, 7766381, 16342475, 37783946, 411173, 14578841, 8080033,
                55534529, 22952821,
            ]),
            xy2d: FieldElement2625::from_limbs([
                19598397, 10334610, 12555054, 2555664, 18821899, 23214652, 21873262, 16014234,
                26224780, 16452269,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                36884920, 5145195, 73053412, 49940397, 71085598, 35564328, 122839923, 25936244,
                46575034, 37253081,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                14187449, 3448569, 56472628, 22743496, 44444983, 30120835, 7268409, 22663988,
                27394300, 12015369,
            ]),
            xy2d: FieldElement2625::from_limbs([
                19695742, 16087646, 28032085, 12999827, 6817792, 11427614, 20244189, 32241655,
                53849736, 30151970,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                97968948, 12735207, 65220619, 28854697, 50133957, 35811371, 126051714, 45852742,
                58558339, 23160969,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                61389038, 22309106, 65198214, 15569034, 26642876, 25966672, 61319509, 18435777,
                62132699, 12651792,
            ]),
            xy2d: FieldElement2625::from_limbs([
                64260450, 9953420, 11531313, 28271553, 26895122, 20857343, 53990043, 17036529,
                9768697, 31021214,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                109498250, 35449081, 66821165, 28850346, 82457582, 25397901, 32767512, 46319882,
                72048958, 44232657,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                18860224, 15980149, 48121624, 31991861, 40875851, 22482575, 59264981, 13944023,
                42736516, 16582018,
            ]),
            xy2d: FieldElement2625::from_limbs([
                51604604, 4970267, 37215820, 4175592, 46115652, 31354675, 55404809, 15444559,
                56105103, 7989036,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                98599278, 39122492, 64696060, 35736814, 34772016, 38086117, 35030594, 39754637,
                47422750, 52308692,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                49800177, 17674491, 35586086, 33551600, 34221481, 16375548, 8680158, 17182719,
                28550067, 26697300,
            ]),
            xy2d: FieldElement2625::from_limbs([
                38981977, 27866340, 16837844, 31733974, 60258182, 12700015, 37068883, 4364037,
                1155602, 5988841,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                88999280, 20281524, 121593716, 12154347, 59276991, 48854927, 90257846, 29083950,
                91727270, 41837612,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                33972757, 23041680, 9975415, 6841041, 35549071, 16356535, 3070187, 26528504,
                1466168, 10740210,
            ]),
            xy2d: FieldElement2625::from_limbs([
                65599446, 18066246, 53605478, 22898515, 32799043, 909394, 53169961, 27774712,
                34944214, 18227391,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                71069668, 19286628, 39082773, 51190812, 47704004, 46701299, 82676190, 34505938,
                63848542, 32980496,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                24740822, 5052253, 37014733, 8961360, 25877428, 6165135, 42740684, 14397371,
                59728495, 27410326,
            ]),
            xy2d: FieldElement2625::from_limbs([
                38220480, 3510802, 39005586, 32395953, 55870735, 22922977, 51667400, 19101303,
                65483377, 27059617,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                67902144, 24323953, 75945165, 27318724, 39747955, 31184838, 100261706, 62223612,
                57202662, 32932579,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                5666214, 525582, 20782575, 25516013, 42570364, 14657739, 16099374, 1468826,
                60937436, 18367850,
            ]),
            xy2d: FieldElement2625::from_limbs([
                62249590, 29775088, 64191105, 26806412, 7778749, 11688288, 36704511, 23683193,
                65549940, 23690785,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                10896313, 25834728, 67933138, 34027032, 114757419, 36564017, 25248957, 48337770,
                36527387, 17796587,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                10566929, 12612572, 35164652, 11118702, 54475488, 12362878, 21752402, 8822496,
                24003793, 14264025,
            ]),
            xy2d: FieldElement2625::from_limbs([
                27713843, 26198459, 56100623, 9227529, 27050101, 2504721, 23886875, 20436907,
                13958494, 27821979,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                110736080, 38421656, 39861735, 37454952, 29838368, 25342141, 102328328, 23512649,
                74449384, 51698795,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                4646495, 25543308, 44342840, 22021777, 23184552, 8566613, 31366726, 32173371,
                52042079, 23179239,
            ]),
            xy2d: FieldElement2625::from_limbs([
                49838347, 12723031, 50115803, 14878793, 21619651, 27356856, 27584816, 3093888,
                58265170, 3849920,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                58043933, 35657603, 92670503, 51983125, 61869038, 43137389, 99585908, 24536476,
                72111157, 18004172,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                55051311, 22376525, 21115584, 20189277, 8808711, 21523724, 16489529, 13378448,
                41263148, 12741425,
            ]),
            xy2d: FieldElement2625::from_limbs([
                61162478, 10645102, 36197278, 15390283, 63821882, 26435754, 24306471, 15852464,
                28834118, 25908360,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                49773097, 24447374, 109686448, 42989383, 58636779, 32971069, 54018092, 34010272,
                87570721, 39045736,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                13669229, 17458950, 54626889, 23351392, 52539093, 21661233, 42112877, 11293806,
                38520660, 24132599,
            ]),
            xy2d: FieldElement2625::from_limbs([
                28497909, 6272777, 34085870, 14470569, 8906179, 32328802, 18504673, 19389266,
                29867744, 24758489,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                50901822, 47071627, 39309233, 19856633, 24009063, 60734973, 60741262, 53933471,
                22853427, 29542421,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                24191359, 16712145, 53177067, 15217830, 14542237, 1646131, 18603514, 22516545,
                12876622, 31441985,
            ]),
            xy2d: FieldElement2625::from_limbs([
                17902668, 4518229, 66697162, 30725184, 26878216, 5258055, 54248111, 608396,
                16031844, 3723494,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                105584936, 12763726, 46662418, 41131935, 33001347, 54091119, 17558840, 59235974,
                23896952, 29240187,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                47103464, 21542479, 31520463, 605201, 2543521, 5991821, 64163800, 7229063,
                57189218, 24727572,
            ]),
            xy2d: FieldElement2625::from_limbs([
                28816026, 298879, 38943848, 17633493, 19000927, 31888542, 54428030, 30605106,
                49057085, 31471516,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                16000882, 33209536, 70601955, 55661665, 37604267, 20394642, 79686603, 49595699,
                47393623, 7847706,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                10151868, 10572098, 27312476, 7922682, 14825339, 4723128, 34252933, 27035413,
                57088296, 3852847,
            ]),
            xy2d: FieldElement2625::from_limbs([
                55678375, 15697595, 45987307, 29133784, 5386313, 15063598, 16514493, 17622322,
                29330898, 18478208,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                41609110, 29175637, 51885955, 26653220, 83724594, 35606215, 70412565, 33569921,
                106668931, 45868821,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                15683501, 27551389, 18109119, 23573784, 15337967, 27556609, 50391428, 15921865,
                16103996, 29823217,
            ]),
            xy2d: FieldElement2625::from_limbs([
                43939021, 22773182, 13588191, 31925625, 63310306, 32479502, 47835256, 5402698,
                37293151, 23713330,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                90299521, 35939014, 34394523, 37016585, 104314072, 32025298, 55842007, 8911516,
                109011869, 36294143,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                21374101, 30000182, 33584214, 9874410, 15377179, 11831242, 33578960, 6134906,
                4931255, 11987849,
            ]),
            xy2d: FieldElement2625::from_limbs([
                67101132, 30575573, 50885377, 7277596, 105524, 33232381, 35628324, 13861387,
                37032554, 10117929,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                37607694, 22809559, 40945095, 13051538, 41483300, 38644074, 127892224, 40258509,
                79998882, 15728939,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                45136504, 21783052, 66157804, 29135591, 14704839, 2695116, 903376, 23126293,
                12885166, 8311031,
            ]),
            xy2d: FieldElement2625::from_limbs([
                49592363, 5352193, 10384213, 19742774, 7506450, 13453191, 26423267, 4384730,
                1888765, 28119028,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                108400371, 64001550, 120723127, 30371924, 98005322, 19632702, 101966083, 20846561,
                47644429, 30214188,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                43500868, 30888657, 66582772, 4651135, 5765089, 4618330, 6092245, 14845197,
                17151279, 23700316,
            ]),
            xy2d: FieldElement2625::from_limbs([
                42278406, 20820711, 51942885, 10367249, 37577956, 33289075, 22825804, 26467153,
                50242379, 16176524,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                43525570, 40119392, 87172552, 37352659, 129477549, 40913655, 69115045, 23191005,
                38362610, 56911354,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                56482264, 29068029, 53788301, 28429114, 3432135, 27161203, 23632036, 31613822,
                32808309, 1099883,
            ]),
            xy2d: FieldElement2625::from_limbs([
                15030958, 5768825, 39657628, 30667132, 60681485, 18193060, 51830967, 26745081,
                2051440, 18328567,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                63746522, 26315059, 74626753, 43379423, 90664713, 33849800, 72257261, 52954675,
                44422508, 50188091,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                4577067, 16802144, 13249840, 18250104, 19958762, 19017158, 18559669, 22794883,
                8402477, 23690159,
            ]),
            xy2d: FieldElement2625::from_limbs([
                38702534, 32502850, 40318708, 32646733, 49896449, 22523642, 9453450, 18574360,
                17983009, 9967138,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                41346351, 40079153, 93694351, 43523701, 24709297, 34774792, 65430873, 7806336,
                84616260, 37205991,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                56688388, 29436320, 14584638, 15971087, 51340543, 8861009, 26556809, 27979875,
                48555541, 22197296,
            ]),
            xy2d: FieldElement2625::from_limbs([
                2839082, 14284142, 4029895, 3472686, 14402957, 12689363, 40466743, 8459446,
                61503401, 25932490,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                62269556, 30018987, 76853824, 2871047, 92222842, 36741449, 109106914, 32705364,
                84366947, 25576692,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                18164541, 22959256, 49953981, 32012014, 19237077, 23809137, 23357532, 18337424,
                26908269, 12150756,
            ]),
            xy2d: FieldElement2625::from_limbs([
                36843994, 25906566, 5112248, 26517760, 65609056, 26580174, 43167, 28016731,
                34806789, 16215818,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                60209940, 43378825, 54804084, 29153342, 102820586, 27277595, 99683352, 46087336,
                59605791, 24879084,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                39765323, 17038963, 39957339, 22831480, 946345, 16291093, 254968, 7168080,
                21676107, 31611404,
            ]),
            xy2d: FieldElement2625::from_limbs([
                21260942, 25129680, 50276977, 21633609, 43430902, 3968120, 63456915, 27338965,
                63552672, 25641356,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                16544735, 46804798, 50304435, 49100673, 62525860, 46311689, 64646555, 24874095,
                48201831, 23891632,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                64693606, 17976703, 18312302, 4964443, 51836334, 20900867, 26820650, 16690659,
                25459437, 28989823,
            ]),
            xy2d: FieldElement2625::from_limbs([
                41964155, 11425019, 28423002, 22533875, 60963942, 17728207, 9142794, 31162830,
                60676445, 31909614,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                44004193, 39807907, 16964146, 29785560, 109103755, 54812425, 39651637, 50764205,
                73444554, 40804420,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                36775618, 13979674, 7503222, 21186118, 55152142, 28932738, 36836594, 2682241,
                25993170, 21075909,
            ]),
            xy2d: FieldElement2625::from_limbs([
                4364628, 5930691, 32304656, 23509878, 59054082, 15091130, 22857016, 22955477,
                31820367, 15075278,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                98987979, 24635738, 84367624, 33645057, 126175891, 28636721, 91271651, 23903545,
                116247489, 46387475,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                19073683, 14851414, 42705695, 21694263, 7625277, 11091125, 47489674, 2074448,
                57694925, 14905376,
            ]),
            xy2d: FieldElement2625::from_limbs([
                24483648, 21618865, 64589997, 22007013, 65555733, 15355505, 41826784, 9253128,
                27628530, 25998952,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                84706452, 41895034, 86464480, 34106618, 26198469, 30377849, 71702187, 24396849,
                120106852, 48851446,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                510886, 14337390, 35323607, 16638631, 6328095, 2713355, 46891447, 21690211,
                8683220, 2921426,
            ]),
            xy2d: FieldElement2625::from_limbs([
                18606791, 11874196, 27155355, 28272950, 43077121, 6265445, 41930624, 32275507,
                4674689, 13890525,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                13609605, 13069022, 106845367, 20498522, 91469449, 43147405, 82086020, 43389536,
                71498550, 33842827,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                9922506, 33035038, 13613106, 5883594, 48350519, 33120168, 54804801, 8317627,
                23388070, 16052080,
            ]),
            xy2d: FieldElement2625::from_limbs([
                12719997, 11937594, 35138804, 28525742, 26900119, 8561328, 46953177, 21921452,
                52354592, 22741539,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                83070703, 47704840, 93825794, 32888599, 111423399, 47157999, 78938436, 41022275,
                38286735, 34483706,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                11038231, 21972036, 39798381, 26237869, 56610336, 17246600, 43629330, 24182562,
                45715720, 2465073,
            ]),
            xy2d: FieldElement2625::from_limbs([
                20017144, 29231206, 27915241, 1529148, 12396362, 15675764, 13817261, 23896366,
                2463390, 28932292,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                50749967, 20890520, 122152544, 38550884, 65852441, 34628003, 76692421, 12851106,
                71112760, 46228148,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                65377275, 18398561, 63845933, 16143081, 19294135, 13385325, 14741514, 24450706,
                7903885, 2348101,
            ]),
            xy2d: FieldElement2625::from_limbs([
                24536016, 17039225, 12715591, 29692277, 1511292, 10047386, 63266518, 26425272,
                38731325, 10048126,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                54486638, 27349611, 97827688, 2591311, 56491836, 12192839, 85982162, 59811773,
                34811106, 15221631,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                40630742, 22450567, 11546243, 31701949, 9180879, 7656409, 45764914, 2095754,
                29769758, 6593415,
            ]),
            xy2d: FieldElement2625::from_limbs([
                35114656, 30646970, 4176911, 3264766, 12538965, 32686321, 26312344, 27435754,
                30958053, 8292160,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                98538667, 53149747, 96282394, 15632447, 12174511, 64348770, 99917693, 37531617,
                93251999, 30405555,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                22648882, 1402143, 44308880, 13746058, 7936347, 365344, 58440231, 31879998,
                63350620, 31249806,
            ]),
            xy2d: FieldElement2625::from_limbs([
                51616947, 8012312, 64594134, 20851969, 43143017, 23300402, 65496150, 32018862,
                50444388, 8194477,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                27338047, 26047012, 59694639, 10140404, 48082437, 26964542, 94386054, 42409807,
                95681149, 36559595,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                26287105, 4821776, 25476601, 29408529, 63344350, 17765447, 49100281, 1182478,
                41014043, 20474836,
            ]),
            xy2d: FieldElement2625::from_limbs([
                59937691, 3178079, 23970071, 6201893, 49913287, 29065239, 45232588, 19571804,
                32208682, 32356184,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                50451143, 36372074, 56822501, 14811297, 73133531, 46903936, 39793359, 56611021,
                39436277, 22014573,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                15941010, 24148500, 45741813, 8062054, 31876073, 33315803, 51830470, 32110002,
                15397330, 29424239,
            ]),
            xy2d: FieldElement2625::from_limbs([
                8934485, 20068965, 43822466, 20131190, 34662773, 14047985, 31170398, 32113411,
                39603297, 15087183,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                115860466, 31397939, 24524912, 16876564, 82629290, 27193655, 118715321, 11461894,
                83897392, 27685489,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                65161459, 16013772, 21750665, 3714552, 49707082, 17498998, 63338576, 23231111,
                31322513, 21938797,
            ]),
            xy2d: FieldElement2625::from_limbs([
                21426636, 27904214, 53460576, 28206894, 38296674, 28633461, 48833472, 18933017,
                13040861, 21441484,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                78402740, 46032517, 107081326, 48638180, 104910306, 14748870, 14555558, 20137329,
                68722574, 38451366,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                41213962, 15323293, 58619073, 25496531, 25967125, 20128972, 2825959, 28657387,
                43137087, 22287016,
            ]),
            xy2d: FieldElement2625::from_limbs([
                51184079, 28324551, 49665331, 6410663, 3622847, 10243618, 20615400, 12405433,
                43355834, 25118015,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                127126414, 46110638, 114026375, 9025185, 50036385, 4333800, 71487300, 35986461,
                23097948, 32988414,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                4565804, 17528778, 20084411, 25711615, 1724998, 189254, 24767264, 10103221,
                48596551, 2424777,
            ]),
            xy2d: FieldElement2625::from_limbs([
                366633, 21577626, 8173089, 26664313, 30788633, 5745705, 59940186, 1344108,
                63466311, 12412658,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                110215918, 41244716, 82038279, 33386174, 102006892, 53695876, 91271559, 51782359,
                63967361, 44733816,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                18289503, 18829478, 8056944, 16430056, 45379140, 7842513, 61107423, 32067534,
                48424218, 22110928,
            ]),
            xy2d: FieldElement2625::from_limbs([
                476239, 6601091, 60956074, 23831056, 17503544, 28690532, 27672958, 13403813,
                11052904, 5219329,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                87787372, 25178693, 34436965, 42403554, 129207969, 48129182, 98295834, 29580701,
                9014761, 58529808,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                53464795, 23204192, 51146355, 5075807, 65594203, 22019831, 34006363, 9160279,
                8473550, 30297594,
            ]),
            xy2d: FieldElement2625::from_limbs([
                24900749, 14435722, 17209120, 18261891, 44516588, 9878982, 59419555, 17218610,
                42540382, 11788947,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                63990690, 22159237, 53306774, 48351872, 76761311, 26708527, 47071426, 43965164,
                42540393, 32095740,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                51449703, 16736705, 44641714, 10215877, 58011687, 7563910, 11871841, 21049238,
                48595538, 8464117,
            ]),
            xy2d: FieldElement2625::from_limbs([
                43708233, 8348506, 52522913, 32692717, 63158658, 27181012, 14325288, 8628612,
                33313881, 25183915,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                46921853, 28586496, 89476219, 38825978, 66011746, 28765593, 109412060, 23317576,
                58168128, 61290594,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                60160060, 31759219, 34483180, 17533252, 32635413, 26180187, 15989196, 20716244,
                28358191, 29300528,
            ]),
            xy2d: FieldElement2625::from_limbs([
                43547083, 30755372, 34757181, 31892468, 57961144, 10429266, 50471180, 4072015,
                61757200, 5596588,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                105981130, 30164382, 79421759, 39767609, 3117141, 49632997, 29266238, 36111653,
                68877164, 15373192,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                59865506, 30307471, 62515396, 26001078, 66980936, 32642186, 66017961, 29049440,
                42448372, 3442909,
            ]),
            xy2d: FieldElement2625::from_limbs([
                36898293, 5124042, 14181784, 8197961, 18964734, 21615339, 22597930, 7176455,
                48523386, 13365929,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                59231455, 32054473, 75433536, 38244510, 73370723, 34444877, 24538106, 24984246,
                57419264, 30522764,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                25008885, 22782833, 62803832, 23916421, 16265035, 15721635, 683793, 21730648,
                15723478, 18390951,
            ]),
            xy2d: FieldElement2625::from_limbs([
                57448220, 12374378, 40101865, 26528283, 59384749, 21239917, 11879681, 5400171,
                519526, 32318556,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                22258378, 50776631, 59239045, 14613015, 44588609, 30603508, 46754982, 40870398,
                16648396, 41160072,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                59027556, 25089834, 58885552, 9719709, 19259459, 18206220, 23994941, 28272877,
                57640015, 4763277,
            ]),
            xy2d: FieldElement2625::from_limbs([
                45409620, 9220968, 51378240, 1084136, 41632757, 30702041, 31088446, 25789909,
                55752334, 728111,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                26047201, 55357393, 127317403, 50587064, 91200930, 9158118, 62835319, 20998873,
                104852291, 28056158,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                17510331, 33231575, 5854288, 8403524, 17133918, 30441820, 38997856, 12327944,
                10750447, 10014012,
            ]),
            xy2d: FieldElement2625::from_limbs([
                56796096, 3936951, 9156313, 24656749, 16498691, 32559785, 39627812, 32887699,
                3424690, 7540221,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                97431206, 26590321, 78469868, 29411114, 74542167, 4989747, 127146306, 50791643,
                57864597, 48812477,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                13054543, 30774935, 19155473, 469045, 54626067, 4566041, 5631406, 2711395, 1062915,
                28418087,
            ]),
            xy2d: FieldElement2625::from_limbs([
                47868616, 22299832, 37599834, 26054466, 61273100, 13005410, 61042375, 12194496,
                32960380, 1459310,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                86960860, 40582355, 90778216, 43574797, 75695366, 26896524, 67503060, 27452546,
                85746866, 55933926,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                31395515, 15098109, 26581030, 8030562, 50580950, 28547297, 9012485, 25970078,
                60465776, 28111795,
            ]),
            xy2d: FieldElement2625::from_limbs([
                57916680, 31207054, 65111764, 4529533, 25766844, 607986, 67095642, 9677542,
                34813975, 27098423,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                64664330, 33404494, 96457765, 8186664, 68982624, 12489862, 103283149, 25714738,
                59256019, 58970434,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                51872508, 18120922, 7766469, 746860, 26346930, 23332670, 39775412, 10754587,
                57677388, 5203575,
            ]),
            xy2d: FieldElement2625::from_limbs([
                31834314, 14135496, 66338857, 5159117, 20917671, 16786336, 59640890, 26216907,
                31809242, 7347066,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                57502122, 21680191, 87523322, 46588417, 80825387, 21862550, 86906833, 21343176,
                82301739, 31466941,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                54445282, 31372712, 1168161, 29749623, 26747876, 19416341, 10609329, 12694420,
                33473243, 20172328,
            ]),
            xy2d: FieldElement2625::from_limbs([
                33184999, 11180355, 15832085, 22169002, 65475192, 225883, 15089336, 22530529,
                60973201, 14480052,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                98417562, 27934433, 98139703, 31657332, 82783410, 26971548, 72605071, 13685226,
                27595050, 42291707,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                46790012, 18404192, 10933842, 17376410, 8335351, 26008410, 36100512, 20943827,
                26498113, 66511,
            ]),
            xy2d: FieldElement2625::from_limbs([
                22644435, 24792703, 50437087, 4884561, 64003250, 19995065, 30540765, 29267685,
                53781076, 26039336,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                106199862, 9834843, 85726071, 30873119, 63706907, 53801357, 75314402, 13585436,
                117090263, 48669869,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                23711543, 32881517, 31206560, 25191721, 6164646, 23844445, 33572981, 32128335,
                8236920, 16492939,
            ]),
            xy2d: FieldElement2625::from_limbs([
                43198286, 20038905, 40809380, 29050590, 25005589, 25867162, 19574901, 10071562,
                6708380, 27332008,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                69210217, 28624377, 86811594, 35922006, 118790560, 34602105, 72409880, 42883131,
                29955600, 55430554,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                3096359, 9271816, 45488000, 18032587, 52260867, 25961494, 41216721, 20918836,
                57191288, 6216607,
            ]),
            xy2d: FieldElement2625::from_limbs([
                34493015, 338662, 41913253, 2510421, 37895298, 19734218, 24822829, 27407865,
                40341383, 7525078,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                44042196, 53123240, 83242349, 25658253, 130828162, 34333218, 66198527, 30771936,
                47722230, 45548532,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                21691500, 19929806, 66467532, 19187410, 3285880, 30070836, 42044197, 9718257,
                59631427, 13381417,
            ]),
            xy2d: FieldElement2625::from_limbs([
                18445390, 29352196, 14979845, 11622458, 65381754, 29971451, 23111647, 27179185,
                28535281, 15779576,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                30098034, 36644094, 124983340, 16662133, 45801924, 44862842, 53040409, 12021729,
                77064149, 17251075,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                9734894, 18977602, 59635230, 24415696, 2060391, 11313496, 48682835, 9924398,
                20194861, 13380996,
            ]),
            xy2d: FieldElement2625::from_limbs([
                40730762, 25589224, 44941042, 15789296, 49053522, 27385639, 65123949, 15707770,
                26342023, 10146099,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                41091971, 33334488, 88448054, 33513043, 86854119, 30675731, 37471583, 35781471,
                21612325, 33008704,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                54031477, 1184227, 23562814, 27583990, 46757619, 27205717, 25764460, 12243797,
                46252298, 11649657,
            ]),
            xy2d: FieldElement2625::from_limbs([
                57077370, 11262625, 27384172, 2271902, 26947504, 17556661, 39943, 6114064,
                33514190, 2333242,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                112784121, 54687041, 75228644, 40774344, 45278341, 58092729, 60429112, 54438225,
                91459440, 20104430,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                62992557, 22282898, 43222677, 4843614, 37020525, 690622, 35572776, 23147595,
                8317859, 12352766,
            ]),
            xy2d: FieldElement2625::from_limbs([
                18200138, 19078521, 34021104, 30857812, 43406342, 24451920, 43556767, 31266881,
                20712162, 6719373,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                26656189, 39629685, 59250307, 35440503, 105873684, 37816756, 78226393, 29791221,
                26224234, 30256974,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                49939907, 18700334, 63713187, 17184554, 47154818, 14050419, 21728352, 9493610,
                18620611, 17125804,
            ]),
            xy2d: FieldElement2625::from_limbs([
                53785524, 13325348, 11432106, 5964811, 18609221, 6062965, 61839393, 23828875,
                36407290, 17074774,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                43248307, 55875704, 94070219, 35195292, 34695751, 16816491, 79357372, 28313792,
                80844205, 35488493,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                25089769, 6742589, 17081145, 20148166, 21909292, 17486451, 51972569, 29789085,
                45830866, 5473615,
            ]),
            xy2d: FieldElement2625::from_limbs([
                31883658, 25593331, 1083431, 21982029, 22828470, 13290673, 59983779, 12469655,
                29111212, 28103418,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                91353792, 52058456, 107954750, 36345970, 52111264, 50221109, 91476329, 39943270,
                56813276, 34006814,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                41468082, 30136590, 5217915, 16224624, 19987036, 29472163, 42872612, 27639183,
                15766061, 8407814,
            ]),
            xy2d: FieldElement2625::from_limbs([
                46701865, 13990230, 15495425, 16395525, 5377168, 15166495, 58191841, 29165478,
                59040954, 2276717,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                30157899, 46478498, 116505677, 42800183, 87003891, 36922573, 43281276, 38650650,
                89849239, 26251014,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                2041139, 19298082, 7783686, 13876377, 41161879, 20201972, 24051123, 13742383,
                51471265, 13295221,
            ]),
            xy2d: FieldElement2625::from_limbs([
                33338218, 25048699, 12532112, 7977527, 9106186, 31839181, 49388668, 28941459,
                62657506, 18884987,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                47063564, 39008528, 52762315, 40001577, 28862070, 35438083, 64639597, 29412551,
                74879432, 43175028,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                23208049, 7979712, 33071466, 8149229, 1758231, 22719437, 30945527, 31860109,
                33606523, 18786461,
            ]),
            xy2d: FieldElement2625::from_limbs([
                1439939, 17283952, 66028874, 32760649, 4625401, 10647766, 62065063, 1220117,
                30494170, 22113633,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                62071265, 20526136, 64138304, 30492664, 82749837, 26852765, 40369837, 34480481,
                65424524, 20220784,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                13908495, 30005160, 30919927, 27280607, 45587000, 7989038, 9021034, 9078865,
                3353509, 4033511,
            ]),
            xy2d: FieldElement2625::from_limbs([
                37445433, 18440821, 32259990, 33209950, 24295848, 20642309, 23161162, 8839127,
                27485041, 7356032,
            ]),
        },
    ]),
    LookupTable([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                76769853, 34259874, 79088928, 28184277, 65480320, 14661172, 60762722, 36179446,
                95539899, 50337029,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                43269631, 25243016, 41163352, 7480957, 49427195, 25200248, 44562891, 14150564,
                15970762, 4099461,
            ]),
            xy2d: FieldElement2625::from_limbs([
                29262576, 16756590, 26350592, 24760869, 8529670, 22346382, 13617292, 23617289,
                11465738, 8317062,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                41615764, 26591503, 99609063, 24135380, 44070139, 31252209, 82007500, 37402886,
                88078197, 28396915,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                46724414, 19206718, 48772458, 13884721, 34069410, 2842113, 45498038, 29904543,
                11177094, 14989547,
            ]),
            xy2d: FieldElement2625::from_limbs([
                42612143, 21838415, 16959895, 2278463, 12066309, 10137771, 13515641, 2581286,
                38621356, 9930239,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                49357223, 31456605, 83653163, 54099563, 118302919, 18605349, 18345766, 53705111,
                83400343, 28240393,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                33879670, 2553287, 32678213, 9875984, 8534129, 6889387, 57432090, 6957616, 4368891,
                9788741,
            ]),
            xy2d: FieldElement2625::from_limbs([
                16660737, 7281060, 56278106, 12911819, 20108584, 25452756, 45386327, 24941283,
                16250551, 22443329,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                47343357, 35944957, 117666696, 14161978, 69014150, 39969338, 71798447, 10604806,
                104027325, 4782745,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                65754325, 14736940, 59741422, 20261545, 7710541, 19398842, 57127292, 4383044,
                22546403, 437323,
            ]),
            xy2d: FieldElement2625::from_limbs([
                31665558, 21373968, 50922033, 1491338, 48740239, 3294681, 27343084, 2786261,
                36475274, 19457415,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                52641566, 32870716, 33734756, 41002983, 19294359, 14334329, 47418233, 35909750,
                47824192, 27440058,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                15121312, 17758270, 6377019, 27523071, 56310752, 20596586, 18952176, 15496498,
                37728731, 11754227,
            ]),
            xy2d: FieldElement2625::from_limbs([
                64471568, 20071356, 8488726, 19250536, 12728760, 31931939, 7141595, 11724556,
                22761615, 23420291,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                16918416, 11729663, 49025285, 36577418, 103201995, 53769203, 38367677, 21327038,
                32851221, 11717399,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                11166615, 7338049, 60386341, 4531519, 37640192, 26252376, 31474878, 3483633,
                65915689, 29523600,
            ]),
            xy2d: FieldElement2625::from_limbs([
                66923210, 9921304, 31456609, 20017994, 55095045, 13348922, 33142652, 6546660,
                47123585, 29606055,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                101757113, 44821142, 55911756, 25655328, 31703693, 37410335, 58571732, 20721383,
                36336829, 18068118,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                49102387, 12709067, 3991746, 27075244, 45617340, 23004006, 35973516, 17504552,
                10928916, 3011958,
            ]),
            xy2d: FieldElement2625::from_limbs([
                60151107, 17960094, 31696058, 334240, 29576716, 14796075, 36277808, 20749251,
                18008030, 10258577,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                44660220, 49210000, 74127342, 29144428, 36794597, 32352840, 65255398, 34921551,
                92236737, 6671742,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                29701166, 19180498, 56230743, 9279287, 67091296, 13127209, 21382910, 11042292,
                25838796, 4642684,
            ]),
            xy2d: FieldElement2625::from_limbs([
                46678630, 14955536, 42982517, 8124618, 61739576, 27563961, 30468146, 19653792,
                18423288, 4177476,
            ]),
        },
    ]),
]);

/// Odd multiples of the basepoint `[B, 3B, 5B, 7B, 9B, 11B, 13B, 15B, ..., 127B]`.
#[cfg(feature = "precomputed-tables")]
#[allow(dead_code)]
pub(crate) const AFFINE_ODD_MULTIPLES_OF_BASEPOINT: NafLookupTable8<AffineNielsPoint> =
    NafLookupTable8([
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                93076338, 52752828, 29566454, 37215328, 54414518, 37569218, 94653489, 21800160,
                61029707, 35602036,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                54563134, 934261, 64385954, 3049989, 66381436, 9406985, 12720692, 5043384,
                19500929, 18085054,
            ]),
            xy2d: FieldElement2625::from_limbs([
                58370664, 4489569, 9688441, 18769238, 10184608, 21191052, 29287918, 11864899,
                42594502, 29115885,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                82745136, 23865874, 24204772, 25642034, 67725840, 16869169, 94896463, 52336674,
                28944398, 32004408,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                16568933, 4717097, 55552716, 32452109, 15682895, 21747389, 16354576, 21778470,
                7689661, 11199574,
            ]),
            xy2d: FieldElement2625::from_limbs([
                30464137, 27578307, 55329429, 17883566, 23220364, 15915852, 7512774, 10017326,
                49359771, 23634074,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                77970208, 11473153, 27284546, 35535607, 37044514, 46132292, 99976748, 48069538,
                118779423, 44373810,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                4708026, 6336745, 20377586, 9066809, 55836755, 6594695, 41455196, 12483687,
                54440373, 5581305,
            ]),
            xy2d: FieldElement2625::from_limbs([
                19563141, 16186464, 37722007, 4097518, 10237984, 29206317, 28542349, 13850243,
                43430843, 17738489,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                72262591, 43463716, 68832610, 30776557, 97632468, 39071304, 86589715, 38784565,
                43156424, 18378665,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                36839857, 30090922, 7665485, 10083793, 28475525, 1649722, 20654025, 16520125,
                30598449, 7715701,
            ]),
            xy2d: FieldElement2625::from_limbs([
                28881826, 14381568, 9657904, 3680757, 46927229, 7843315, 35708204, 1370707,
                29794553, 32145132,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                44589852, 26862249, 14201701, 24808930, 43598457, 42399157, 85583074, 32192981,
                54046167, 47376308,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                60653668, 25714560, 3374701, 28813570, 40010246, 22982724, 31655027, 26342105,
                18853321, 19333481,
            ]),
            xy2d: FieldElement2625::from_limbs([
                4566811, 20590564, 38133974, 21313742, 59506191, 30723862, 58594505, 23123294,
                2207752, 30344648,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                41954014, 62923042, 96790006, 41423232, 60254202, 24130566, 121780363, 32891430,
                103106264, 17421994,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                25576264, 30851218, 7349803, 21739588, 16472781, 9300885, 3844789, 15725684,
                171356, 6466918,
            ]),
            xy2d: FieldElement2625::from_limbs([
                23103977, 13316479, 9739013, 17404951, 817874, 18515490, 8965338, 19466374,
                36393951, 16193876,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                100695917, 36735143, 64714733, 47558118, 50205389, 17283591, 84347261, 38283886,
                49034350, 9256799,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                41926547, 29380300, 32336397, 5036987, 45872047, 11360616, 22616405, 9761698,
                47281666, 630304,
            ]),
            xy2d: FieldElement2625::from_limbs([
                53388152, 2639452, 42871404, 26147950, 9494426, 27780403, 60554312, 17593437,
                64659607, 19263131,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                63957664, 28508356, 76391577, 40420576, 102310665, 32691407, 48168288, 15033783,
                92213982, 25659555,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                42782475, 15950225, 35307649, 18961608, 55446126, 28463506, 1573891, 30928545,
                2198789, 17749813,
            ]),
            xy2d: FieldElement2625::from_limbs([
                64009494, 10324966, 64867251, 7453182, 61661885, 30818928, 53296841, 17317989,
                34647629, 21263748,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                17735022, 27114469, 76149336, 40765111, 43325570, 26153544, 26948151, 45905235,
                38656900, 62179684,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                2154119, 14782993, 28737794, 11906199, 36205504, 26488101, 19338132, 16910143,
                50209922, 29794297,
            ]),
            xy2d: FieldElement2625::from_limbs([
                29935700, 6336041, 20999566, 30405369, 13628497, 24612108, 61639745, 22359641,
                56973806, 18684690,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                29792811, 31379227, 113441390, 20675662, 58452680, 54138549, 42892249, 32958636,
                31674345, 24275271,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                7606599, 22131225, 17376912, 15235046, 32822971, 7512882, 30227203, 14344178,
                9952094, 8804749,
            ]),
            xy2d: FieldElement2625::from_limbs([
                32575079, 3961822, 36404898, 17773250, 67073898, 1319543, 30641032, 7823672,
                63309858, 18878784,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                77823924, 52933642, 26572931, 18690221, 109143683, 23989794, 79129572, 53326100,
                38888709, 55889506,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                37146997, 554126, 63326061, 20925660, 49205290, 8620615, 53375504, 25938867,
                8752612, 31225894,
            ]),
            xy2d: FieldElement2625::from_limbs([
                4529887, 12416158, 60388162, 30157900, 15427957, 27628808, 61150927, 12724463,
                23658330, 23690055,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                102043267, 54823614, 45810225, 19657305, 54297192, 7413280, 66851983, 39718512,
                25005048, 18002658,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                5403481, 24654166, 61855580, 13522652, 14989680, 1879017, 43913069, 25724172,
                20315901, 421248,
            ]),
            xy2d: FieldElement2625::from_limbs([
                34818947, 1705239, 25347020, 7938434, 51632025, 1720023, 54809726, 32655885,
                64907986, 5517607,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                88543525, 16557377, 80359887, 30047148, 91602876, 27723948, 62710290, 52707861,
                7715736, 61648232,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                14461032, 6393639, 22681353, 14533514, 52493587, 3544717, 57780998, 24657863,
                59891807, 31628125,
            ]),
            xy2d: FieldElement2625::from_limbs([
                60864886, 31199953, 18524951, 11247802, 43517645, 21165456, 26204394, 27268421,
                63221077, 29979135,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                97491378, 10077555, 94805128, 42472719, 30231379, 17961119, 76201413, 41182329,
                41405214, 31798052,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                13670592, 720327, 7131696, 19360499, 66651570, 16947532, 3061924, 22871019,
                39814495, 20141336,
            ]),
            xy2d: FieldElement2625::from_limbs([
                44847187, 28379568, 38472030, 23697331, 49441718, 3215393, 1669253, 30451034,
                62323912, 29368533,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                74923758, 35244493, 27222384, 30715870, 48444195, 28125622, 116052444, 32330148,
                92609232, 35372537,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                39340596, 15199968, 52787715, 18781603, 18787729, 5464578, 11652644, 8722118,
                57056621, 5153960,
            ]),
            xy2d: FieldElement2625::from_limbs([
                5733861, 14534448, 59480402, 15892910, 30737296, 188529, 491756, 17646733,
                33071791, 15771063,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                85239571, 21331573, 119690709, 30172286, 44350959, 55826224, 68258766, 16209406,
                20222151, 32139086,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                52372801, 13847470, 52690845, 3802477, 48387139, 10595589, 13745896, 3112846,
                50361463, 2761905,
            ]),
            xy2d: FieldElement2625::from_limbs([
                45982696, 12273933, 15897066, 704320, 31367969, 3120352, 11710867, 16405685,
                19410991, 10591627,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                82008850, 34439758, 89319886, 49124188, 34309215, 29866047, 80308709, 27738519,
                71739865, 46909287,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                36631997, 23300851, 59535242, 27474493, 59924914, 29067704, 17551261, 13583017,
                37580567, 31071178,
            ]),
            xy2d: FieldElement2625::from_limbs([
                22641770, 21277083, 10843473, 1582748, 37504588, 634914, 15612385, 18139122,
                59415250, 22563863,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                76721854, 52814714, 41722368, 35285867, 53022548, 38255176, 93163883, 27627617,
                87963092, 33729456,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                61915349, 11733561, 59403492, 31381562, 29521830, 16845409, 54973419, 26057054,
                49464700, 796779,
            ]),
            xy2d: FieldElement2625::from_limbs([
                3855018, 8248512, 12652406, 88331, 2948262, 971326, 15614761, 9441028, 29507685,
                8583792,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                76968870, 14808584, 76708906, 57649718, 23400175, 24077237, 63783137, 37471119,
                56750251, 30681804,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                33709664, 3740344, 52888604, 25059045, 46197996, 22678812, 45207164, 6431243,
                21300862, 27646257,
            ]),
            xy2d: FieldElement2625::from_limbs([
                49811511, 9216232, 25043921, 18738174, 29145960, 3024227, 65580502, 530149,
                66809973, 22275500,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                23499366, 24936714, 38355445, 35908587, 82540167, 39280880, 46809413, 41143783,
                72530804, 49676198,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                45162189, 23851397, 9380591, 15192763, 36034862, 15525765, 5277811, 25040629,
                33286237, 31693326,
            ]),
            xy2d: FieldElement2625::from_limbs([
                62424427, 13336013, 49368582, 1581264, 30884213, 15048226, 66823504, 4736577,
                53805192, 29608355,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                25190215, 26304748, 58928336, 42665707, 64280342, 38580230, 61299598, 20659504,
                30387592, 32519377,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                14480213, 17057820, 2286692, 32980967, 14693157, 22197912, 49247898, 9909859,
                236428, 16857435,
            ]),
            xy2d: FieldElement2625::from_limbs([
                7877514, 29872867, 45886243, 25902853, 41998762, 6241604, 35694938, 15657879,
                56797932, 8609105,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                54245189, 32562161, 57887697, 19509733, 45323534, 37472546, 27606727, 59528498,
                74398957, 44973176,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                28964163, 20950093, 44929966, 26145892, 34786807, 18058153, 18187179, 27016486,
                42438836, 14869174,
            ]),
            xy2d: FieldElement2625::from_limbs([
                55703901, 1222455, 64329400, 24533246, 11330890, 9135834, 3589529, 19555234,
                53275553, 1207212,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                33323313, 35603165, 79328585, 6017848, 71286345, 23804207, 86644124, 44008367,
                55775078, 31816581,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                64814718, 27217688, 29891310, 4504619, 8548709, 21986323, 62140656, 12555980,
                34377058, 21436823,
            ]),
            xy2d: FieldElement2625::from_limbs([
                49069441, 9880212, 33350825, 24576421, 24446077, 15616561, 19302117, 9370836,
                55172180, 28526191,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                95404934, 26757208, 123864063, 4572839, 69249194, 43584425, 53559055, 41742046,
                41167331, 24643278,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                35101859, 30958612, 66105296, 3168612, 22836264, 10055966, 22893634, 13045780,
                28576558, 30704591,
            ]),
            xy2d: FieldElement2625::from_limbs([
                59987873, 21166324, 43296694, 15387892, 39447987, 19996270, 5059183, 19972934,
                30207804, 29631666,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                67444156, 16132892, 88330413, 37924284, 68147855, 57949418, 91481571, 24889160,
                62329722, 50712214,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                56922508, 1347520, 23300731, 27393371, 42651667, 8512932, 27610931, 24436993,
                3998295, 3835244,
            ]),
            xy2d: FieldElement2625::from_limbs([
                16327050, 22776956, 14746360, 22599650, 23700920, 11727222, 25900154, 21823218,
                34907363, 25105813,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                59807886, 12089757, 115624210, 41476837, 67589715, 26361580, 71355762, 44268661,
                67753061, 13128476,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                7174885, 26592113, 59892333, 6465478, 4145835, 17673606, 38764952, 22293290,
                1360980, 25805937,
            ]),
            xy2d: FieldElement2625::from_limbs([
                40179568, 6331649, 42386021, 20205884, 15635073, 6103612, 56391180, 6789942,
                7597240, 24095312,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                54776568, 36935932, 18757261, 41429535, 67215081, 34700142, 86560976, 61204154,
                26496794, 19612129,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                46701540, 24101444, 49515651, 25946994, 45338156, 9941093, 55509371, 31298943,
                1347425, 15381335,
            ]),
            xy2d: FieldElement2625::from_limbs([
                53576449, 26135856, 17092785, 3684747, 57829121, 27109516, 2987881, 10987137,
                52269096, 15465522,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                80033010, 26264316, 72380996, 10039544, 94605936, 30615493, 60406855, 30400829,
                120765849, 45301372,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                35668062, 24246990, 47788280, 25128298, 37456967, 19518969, 43459670, 10724644,
                7294162, 4471290,
            ]),
            xy2d: FieldElement2625::from_limbs([
                33813988, 3549109, 101112, 21464449, 4858392, 3029943, 59999440, 21424738,
                34313875, 1512799,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                29494960, 28240930, 51093230, 28823678, 92791151, 54796794, 77571888, 37795542,
                75765856, 10649531,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                63536751, 7572551, 62249759, 25202639, 32046232, 32318941, 29315141, 15424555,
                24706712, 28857648,
            ]),
            xy2d: FieldElement2625::from_limbs([
                47618751, 5819839, 19528172, 20715950, 40655763, 20611047, 4960954, 6496879,
                2790858, 28045273,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                85174457, 55843901, 111946683, 31021158, 32797785, 48944265, 78338887, 31144772,
                82688001, 38470222,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                49664705, 3638040, 57888693, 19234931, 40104182, 28143840, 28667142, 18386877,
                18584835, 3592929,
            ]),
            xy2d: FieldElement2625::from_limbs([
                12065039, 18867394, 6430594, 17107159, 1727094, 13096957, 61520237, 27056604,
                27026997, 13543966,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                68512926, 37577278, 94695528, 14209106, 95849194, 30038709, 51818051, 20241476,
                68980056, 42251074,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                17325298, 33376175, 65271265, 4931225, 31708266, 6292284, 23064744, 22072792,
                43945505, 9236924,
            ]),
            xy2d: FieldElement2625::from_limbs([
                51955585, 20268063, 61151838, 26383348, 4766519, 20788033, 21173534, 27030753,
                9509140, 7790046,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                24124086, 38918775, 28620390, 10538620, 59433851, 19581010, 60862718, 43500219,
                77600721, 32213801,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                7062127, 13930079, 2259902, 6463144, 32137099, 24748848, 41557343, 29331342,
                47345194, 13022814,
            ]),
            xy2d: FieldElement2625::from_limbs([
                18921826, 392002, 55817981, 6420686, 8000611, 22415972, 14722962, 26246290,
                20604450, 8079345,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                67710253, 26257798, 51499391, 46550521, 30228769, 53940987, 76234206, 43362242,
                77953697, 21034392,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                25817710, 8020883, 50134679, 21244805, 47057788, 8766556, 29308546, 22307963,
                49449920, 23874253,
            ]),
            xy2d: FieldElement2625::from_limbs([
                11081015, 13522660, 12474691, 29260223, 48687631, 9341946, 16850694, 18637605,
                6199839, 14303642,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                64518173, 19894035, 117213833, 43031641, 79641718, 39533880, 66531934, 41205092,
                117735515, 13989682,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                6921800, 4421166, 59739491, 30510778, 43106355, 30941531, 9363541, 3394240,
                50874187, 23872585,
            ]),
            xy2d: FieldElement2625::from_limbs([
                54293979, 23466866, 47184247, 20627378, 8313211, 5865878, 5948507, 32290343,
                52583140, 23139870,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                111574723, 24134616, 49842442, 23485580, 34844037, 45228427, 67103167, 25858409,
                38508586, 35097070,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                19879846, 15259900, 25020018, 14261729, 22075205, 25189303, 787540, 31325033,
                62422289, 16131171,
            ]),
            xy2d: FieldElement2625::from_limbs([
                39487053, 27893575, 34654176, 25620816, 60209846, 23603919, 8931189, 12275052,
                38626469, 33438928,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                105416367, 9568747, 62672739, 49685015, 106242995, 4547918, 18403901, 38581738,
                60829966, 33150322,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                7950033, 25841033, 47276506, 3884935, 62418883, 2342083, 50269031, 14194015,
                27013685, 3320257,
            ]),
            xy2d: FieldElement2625::from_limbs([
                35270691, 18076829, 46994271, 4273335, 43595882, 31742297, 58328702, 4594760,
                49180851, 18144010,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                30194115, 50068680, 49746331, 27470090, 40428285, 23271051, 70252167, 16153483,
                123511881, 27809602,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                27113466, 6865046, 4512771, 29327742, 29021084, 7405965, 33302911, 9322435,
                4307527, 32438240,
            ]),
            xy2d: FieldElement2625::from_limbs([
                29337813, 24673346, 10359233, 30347534, 57709483, 9930840, 60607771, 24076133,
                20985293, 22480923,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                14579237, 33467236, 85745988, 15769997, 101228358, 21649866, 82685456, 59023858,
                86175344, 24337101,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                4472119, 14702190, 10432042, 22460027, 708461, 18783996, 34234374, 30870323,
                63796457, 10370850,
            ]),
            xy2d: FieldElement2625::from_limbs([
                36957127, 19555637, 16244231, 24367549, 58999881, 13440043, 35147632, 8718974,
                43101064, 18487380,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                21818223, 34477173, 23913863, 22441963, 129271975, 14842154, 43035020, 9485973,
                53819529, 22318987,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                10874834, 4351765, 66252340, 17269436, 64427034, 30735311, 5883785, 28998531,
                44403022, 26064601,
            ]),
            xy2d: FieldElement2625::from_limbs([
                64017630, 9755550, 37507935, 22752543, 4031638, 29903925, 47267417, 32706846,
                39147952, 21635901,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                81365001, 44927611, 97395185, 43985591, 66242539, 38517499, 52937891, 37374973,
                73352483, 38476849,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                43460763, 24260930, 21493330, 30888969, 23329454, 24545577, 58286855, 12750266,
                22391140, 26198125,
            ]),
            xy2d: FieldElement2625::from_limbs([
                20477567, 24078713, 1674568, 4102219, 25208396, 13972305, 30389482, 19572626,
                1485666, 17679765,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                100511110, 23887606, 116505658, 30877106, 45483774, 25222431, 67931340, 37154158,
                32618865, 18610785,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                48647066, 166413, 55454758, 8889513, 21027475, 32728181, 43100067, 4690060,
                7520989, 16421303,
            ]),
            xy2d: FieldElement2625::from_limbs([
                14868391, 20996450, 64836606, 1042490, 27060176, 10253541, 53431276, 19516737,
                41808946, 2239538,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                50228416, 29594943, 62030348, 10307368, 70970997, 20292574, 126292474, 51543890,
                67827181, 15848795,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                5548701, 17911007, 33137864, 32764443, 31146554, 17931096, 64023370, 7290289,
                6361313, 32861205,
            ]),
            xy2d: FieldElement2625::from_limbs([
                63374742, 30320053, 4091667, 30955480, 44819449, 2212055, 52638826, 22391938,
                38484599, 7051029,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                50485560, 7033600, 57711425, 10740562, 72347547, 42328739, 7593987, 46950560,
                85560721, 41970063,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                40930651, 3776911, 39108529, 2508077, 19371703, 7626128, 4092943, 15778278,
                42044145, 24540103,
            ]),
            xy2d: FieldElement2625::from_limbs([
                44128555, 8867576, 8645499, 22222278, 11497130, 4344907, 10788462, 23382703,
                3547104, 15368835,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                81786515, 51902785, 74560130, 22753403, 52379722, 41395524, 57994925, 6818020,
                57707296, 16352835,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                21622574, 18581624, 36511951, 1212467, 36930308, 7910192, 20622927, 2438677,
                52628762, 29068327,
            ]),
            xy2d: FieldElement2625::from_limbs([
                6797431, 2854059, 4269865, 8037366, 32016522, 15223213, 34765784, 15297582,
                3559197, 26425254,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                107761639, 61759660, 79235166, 8794359, 48418924, 60111631, 87862210, 33613219,
                68436482, 40229362,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                52388944, 32880897, 37676257, 8253690, 32826330, 2707379, 25088512, 17182878,
                15053907, 11601568,
            ]),
            xy2d: FieldElement2625::from_limbs([
                43894091, 25425955, 50962615, 28097648, 30129084, 13258436, 39364589, 8197601,
                58181660, 15003422,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                13470722, 47835674, 31012390, 30525035, 89789519, 50713267, 39648035, 13815677,
                94028755, 62582101,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                54478677, 14782829, 56712503, 7094748, 41775828, 29409658, 9084386, 30179063,
                64014926, 32519086,
            ]),
            xy2d: FieldElement2625::from_limbs([
                6314429, 20018828, 12535891, 19610611, 10074031, 28087963, 50489447, 26314252,
                24553876, 32746308,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                105768482, 46629424, 103418946, 65789027, 85765355, 28316167, 56299027, 22780838,
                122676432, 32376204,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                5654403, 26425050, 39347935, 963424, 5032477, 19850195, 30011537, 11153401,
                63182039, 13343989,
            ]),
            xy2d: FieldElement2625::from_limbs([
                1130444, 29814849, 40569426, 8144467, 24179188, 6267924, 63847147, 2912740,
                63870704, 29186744,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                49722534, 11073633, 52865263, 50829611, 33921405, 38614719, 32360242, 35465390,
                50107050, 45035301,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                2003571, 2472803, 46902183, 1716406, 58609069, 15922982, 43766122, 27456369,
                33468339, 29346282,
            ]),
            xy2d: FieldElement2625::from_limbs([
                18834217, 8245144, 29896065, 3490830, 62967493, 7220277, 146130, 18459164,
                57533060, 30070422,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                77805507, 38474121, 73459597, 18553340, 107508318, 52705654, 33655873, 27331956,
                44498407, 13768350,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                23652128, 27647291, 43351590, 13262712, 65238054, 26296349, 11902126, 2949002,
                34445239, 25602117,
            ]),
            xy2d: FieldElement2625::from_limbs([
                55906958, 19046111, 28501158, 28224561, 14495533, 14714956, 32929972, 2643566,
                17034893, 11645825,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                38181639, 29751709, 73650473, 17760526, 80753587, 17992258, 72670209, 41214427,
                87524152, 37630124,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                6498441, 12053607, 10375600, 14764370, 24795955, 16159258, 57849421, 16071837,
                31008329, 3792564,
            ]),
            xy2d: FieldElement2625::from_limbs([
                47930485, 9176956, 54248931, 8732776, 58000258, 10333519, 96092, 29273884,
                13051277, 20121493,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                54190492, 49837594, 61282066, 10734597, 67926686, 36967416, 115462142, 30339271,
                37200685, 30036936,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                21193614, 19929501, 18841215, 29565554, 64002173, 11123558, 14111648, 6069945,
                30307604, 25935103,
            ]),
            xy2d: FieldElement2625::from_limbs([
                58539773, 2098685, 38301131, 15844175, 41633654, 16934366, 15145895, 5543861,
                64050790, 6595361,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                34107945, 34731353, 51956038, 5614778, 79079051, 30288154, 47460410, 22186730,
                30689695, 19628976,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                25043248, 19224237, 46048097, 32289319, 29339134, 12397721, 37385860, 12978240,
                57951631, 31419653,
            ]),
            xy2d: FieldElement2625::from_limbs([
                46038439, 28501736, 62566522, 12609283, 35236982, 30457796, 64113609, 14800343,
                6412849, 6276813,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                124528774, 39505727, 83050803, 41361190, 116071796, 37845759, 61633481, 38385016,
                71255100, 31629488,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                249426, 17196749, 35434953, 13884216, 11701636, 24553269, 51821986, 12900910,
                34844073, 16150118,
            ]),
            xy2d: FieldElement2625::from_limbs([
                2520516, 14697628, 15319213, 22684490, 62866663, 29666431, 13872507, 7473319,
                12419515, 2958466,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                101517167, 22298305, 98222207, 59471046, 61547444, 50370568, 97111094, 42539051,
                14298448, 49873561,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                19427905, 12004555, 9971383, 28189868, 32306269, 23648270, 34176633, 10760437,
                53354280, 5634974,
            ]),
            xy2d: FieldElement2625::from_limbs([
                30044319, 23677863, 60273406, 14563839, 9734978, 19808149, 30899064, 30835691,
                22828539, 23633348,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                25513026, 37111929, 37113703, 29589233, 77394412, 34745965, 95889446, 61766763,
                92876242, 37566563,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                42139852, 9176396, 16274786, 33467453, 52558621, 7190768, 1490604, 31312359,
                44767199, 18491072,
            ]),
            xy2d: FieldElement2625::from_limbs([
                4272877, 21431483, 45594743, 13027605, 59232641, 24151956, 38390319, 12906718,
                45915869, 15503563,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                29874396, 35808736, 25494239, 37976524, 43036007, 37144111, 18198811, 35141252,
                53490316, 47742788,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                59518553, 28520621, 59946871, 29462027, 3630300, 29398589, 60425462, 24588735,
                53129947, 28399367,
            ]),
            xy2d: FieldElement2625::from_limbs([
                18192774, 12787801, 32021061, 9158184, 48389348, 16385092, 11799402, 9492011,
                43154220, 15950102,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                68768204, 54638026, 33464925, 53430209, 66037964, 35360373, 22565155, 39168685,
                46605438, 51897954,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                57660336, 29715319, 64414626, 32753338, 16894121, 935644, 53848937, 22684138,
                10541713, 14174330,
            ]),
            xy2d: FieldElement2625::from_limbs([
                22888141, 12700209, 40301697, 6435658, 56329485, 5524686, 56715961, 6520808,
                15754965, 9355803,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                79549820, 26746924, 54931884, 38547877, 49672847, 19708985, 52599424, 12757151,
                93328625, 39524327,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                33888606, 13911610, 18921581, 1162763, 46616901, 13799218, 29525142, 21929286,
                59295464, 503508,
            ]),
            xy2d: FieldElement2625::from_limbs([
                57865531, 22043577, 17998312, 3038439, 52838371, 9832208, 43311531, 660991,
                25265267, 18977724,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                64010269, 23727746, 42277281, 48089313, 102316973, 34946803, 127880577, 38411468,
                114816699, 43712746,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                56859315, 32558245, 41017090, 22610758, 13704990, 23215119, 2475037, 32344984,
                12799418, 11135856,
            ]),
            xy2d: FieldElement2625::from_limbs([
                1867214, 27167702, 19772099, 16925005, 15366693, 25797692, 10829276, 15372827,
                26582557, 31642714,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                57265197, 20059797, 107314987, 30587501, 60553812, 25602102, 29690666, 37127097,
                103070929, 51772159,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                56432653, 6329655, 42770975, 4187982, 30677076, 9335071, 60103332, 14755050,
                9451294, 574767,
            ]),
            xy2d: FieldElement2625::from_limbs([
                52859018, 2867107, 56258365, 15719081, 5959372, 8703738, 29137781, 21575537,
                20249840, 31808689,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                74749335, 47235127, 9995910, 52200224, 92069015, 8964515, 33248715, 21201554,
                57573145, 31605506,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                56307055, 23891752, 3613811, 30787942, 49031222, 26667524, 26985478, 31973510,
                26785294, 29587427,
            ]),
            xy2d: FieldElement2625::from_limbs([
                30891460, 5254655, 47414930, 12769216, 42912782, 11830405, 7411958, 1394027,
                18778535, 18209370,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                61227949, 26179350, 57501473, 13585864, 102855675, 40344975, 54134826, 59707765,
                74122694, 12256219,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                5975515, 16302413, 24341148, 28270615, 18786096, 22405501, 28243950, 28328004,
                53412289, 4381960,
            ]),
            xy2d: FieldElement2625::from_limbs([
                9394648, 8758552, 26189703, 16642536, 35993528, 5117040, 5977877, 13955594,
                19244020, 24493735,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                111388362, 51822507, 30193028, 3993472, 110736308, 44014764, 107346699, 48464072,
                92830877, 56442511,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                7236795, 30433657, 63588571, 620817, 11118384, 24979014, 66780154, 19877679,
                16217590, 26311105,
            ]),
            xy2d: FieldElement2625::from_limbs([
                42540794, 21657271, 16455973, 23630199, 3992015, 21894417, 44876052, 19291718,
                55429803, 30442389,
            ]),
        },
        AffineNielsPoint {
            y_plus_x: FieldElement2625::from_limbs([
                69421833, 26972132, 58859271, 20240912, 119664007, 29643940, 93968457, 34515112,
                110902491, 44996669,
            ]),
            y_minus_x: FieldElement2625::from_limbs([
                3428668, 27807272, 41139948, 24786894, 4167808, 21423270, 52199622, 8021269,
                53172251, 18070808,
            ]),
            xy2d: FieldElement2625::from_limbs([
                30631113, 26363656, 21279866, 23275794, 18311406, 466071, 42527968, 7989982,
                29641567, 29446694,
            ]),
        },
    ]);
