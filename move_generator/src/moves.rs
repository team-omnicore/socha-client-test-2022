use crate::bitboard::Bitboard;

const NOT_FILE_A: u64 = !0x0101010101010101;
const NOT_FILE_B: u64 = !0x0202020202020202;
const NOT_FILE_C: u64 = !0x0404040404040404;
const NOT_FILE_D: u64 = !0x0808080808080808;
const NOT_FILE_E: u64 = !0x1010101010101010;
const NOT_FILE_F: u64 = !0x2020202020202020;
const NOT_FILE_G: u64 = !0x4040404040404040;
const NOT_FILE_H: u64 = !0x8080808080808080;

const LOOKUP_ROBBEN: [u64; 64] = [
    0x20400,
    0x50800,
    0xA1100,
    0x142200,
    0x284400,
    0x508800,
    0xA01000,
    0x402000,
    0x2040004,
    0x5080008,
    0xA110011,
    0x14220022,
    0x28440044,
    0x50880088,
    0xA0100010,
    0x40200020,
    0x204000402,
    0x508000805,
    0xA1100110A,
    0x1422002214,
    0x2844004428,
    0x5088008850,
    0xA0100010A0,
    0x4020002040,
    0x20400040200,
    0x50800080500,
    0xA1100110A00,
    0x142200221400,
    0x284400442800,
    0x508800885000,
    0xA0100010A000,
    0x402000204000,
    0x2040004020000,
    0x5080008050000,
    0xA1100110A0000,
    0x14220022140000,
    0x28440044280000,
    0x50880088500000,
    0xA0100010A00000,
    0x40200020400000,
    0x204000402000000,
    0x508000805000000,
    0xA1100110A000000,
    0x1422002214000000,
    0x2844004428000000,
    0x5088008850000000,
    0xA0100010A0000000,
    0x4020002040000000,
    0x400040200000000,
    0x800080500000000,
    0x1100110A00000000,
    0x2200221400000000,
    0x4400442800000000,
    0x8800885000000000,
    0x100010A000000000,
    0x2000204000000000,
    0x4020000000000,
    0x8050000000000,
    0x110A0000000000,
    0x22140000000000,
    0x44280000000000,
    0x88500000000000,
    0x10A00000000000,
    0x20400000000000,
];

pub fn muschel_gen_moves(muschel: Bitboard) -> Bitboard {
    let muschel_loc = muschel.bits;

    let clip_file_a = muschel_loc & NOT_FILE_A;
    let clip_file_h = muschel_loc & NOT_FILE_H;

    let spot_1 = clip_file_a << 7;
    let spot_3 = clip_file_h << 9;

    let moves = spot_1 | spot_3;

    Bitboard::from(moves)
}

pub fn moewe_gen_moves(moewe: Bitboard, maximizing_player: bool) -> Bitboard {
    let moewe_loc = moewe.bits;

    let clip_file_a = moewe_loc & NOT_FILE_A;
    let clip_file_h = moewe_loc & NOT_FILE_H;

    let spot_2 = moewe_loc << 8;
    let spot_4 = clip_file_h << 1;
    let spot_6 = moewe_loc >> 8;
    let spot_8 = clip_file_a >> 1;

    let moves = spot_2 | spot_4 | spot_6 | spot_8;

    Bitboard::from(moves)
}

pub fn seestern_gen_moves(seestern: Bitboard) -> Bitboard {
    let seestern_loc = seestern.bits;

    let clip_file_a = seestern_loc & NOT_FILE_A;
    let clip_file_h = seestern_loc & NOT_FILE_H;

    let spot_1 = clip_file_a << 7;
    let spot_2 = seestern_loc << 8;
    let spot_3 = clip_file_h << 9;

    let spot_5 = clip_file_h >> 7;
    let spot_7 = clip_file_a >> 9;

    let moves = spot_1 | spot_2 | spot_3 | spot_5 | spot_7;

    Bitboard::from(moves)
}

pub fn robbe_gen_moves(robbe: Bitboard) -> Bitboard {
    let robbe_loc = robbe.bits;

    let clip_file_ab = NOT_FILE_A & NOT_FILE_B;
    let clip_file_a = NOT_FILE_A;
    let clip_file_h = NOT_FILE_H;
    let clip_file_ag = NOT_FILE_H & NOT_FILE_G;

    let spot_1 = (robbe_loc & clip_file_ab) << 6;
    let spot_2 = (robbe_loc & clip_file_a) << 15;
    let spot_3 = (robbe_loc & clip_file_h) << 17;
    let spot_4 = (robbe_loc & clip_file_ag) << 10;

    let spot_5 = (robbe_loc & clip_file_ag) >> 6;
    let spot_6 = (robbe_loc & clip_file_h) >> 15;
    let spot_7 = (robbe_loc & clip_file_a) >> 17;
    let spot_8 = (robbe_loc & clip_file_ab) >> 10;

    let moves = spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8;

    Bitboard::from(moves)
}

pub fn robbe_lookup(pos: u8) -> Bitboard {
    Bitboard::from(LOOKUP_ROBBEN[pos as usize])
}
