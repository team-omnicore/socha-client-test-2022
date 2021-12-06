#Milestones - Speeding up the Seal

---

##With lookup table for robbe
| Perft level | Move-count | time taken |
| ----------- | ----------- | ----------- |
Perft  7   |      8,371,299   |      22.5424ms
Perft  8   |     81,336,867   |     201.1619ms
Perft  9   |    787,866,241   |     1.9480438s
Perft 10   |  7,765,826,375   |    18.6744612s
|
##Without rotating board:

>Pieces that move symmetrically in each direction do not need the board to be rotated

| Perft level | Move-count | time taken | Speed |
| ----------- | ----------- | ----------- | ----------- |
Perft  7   |     8,371,299   |      23.5355ms  |   355.7 MNodes/sec
Perft  8   |    81,336,867   |     147.4057ms  |   551.8 MNodes/sec
Perft  9   |   787,866,241   |     1.3335887s  |   590.8 MNodes/sec
Perft 10   | 7,765,826,375   |    12.3177942s  |   630.5 MNodes/sec

##After replacing the inner if statement:
```rust
if (double & new_robbe).bits != 0 {
    clone.board.double &= !new_robbe;
    clone.board.robben &= !new_robbe;
    clone.board.friendly &= !new_robbe;
    points_added += 1;
}
```
####And replacing it with:
````rust
let overlaps = Bitboard::from(((rhs.bits & lhs.bits) != 0)as u64 * u64::MAX)
let clip_robbe = !new_robbe | !overlaps;
clone.board.double &= clip_robbe;
clone.board.robben &= clip_robbe;
clone.board.friendly &= clip_robbe;
points_added += 1 & overlaps.bits as u8;
````
| Perft level | Move-count | time taken | Speed |
| ----------- | ----------- | ----------- | ----------- |
Perft  7 |         8,371,299 |         14.7538ms |     567.4 MNodes/sec
Perft  8 |        81,336,867 |         148.717ms |     546.9 MNodes/sec
Perft  9 |       787,866,241 |        1.3687529s |     575.6 MNodes/sec
Perft 10 |     7,765,826,375 |       12.0719279s |     643.3 MNodes/sec

##After replacing all ifs with non-branching code
```rust
let overlaps_enemy = overlaps(new_robbe, enemy);
let clip_if_overlaps_enemy = !new_robbe | !overlaps_enemy;

clone.board.enemy &= clip_if_overlaps_enemy;
clone.board.robben &= clip_if_overlaps_enemy;
clone.board.seesterne &= clip_if_overlaps_enemy;
clone.board.muscheln &= clip_if_overlaps_enemy;
clone.board.moewen &= clip_if_overlaps_enemy;

let mut points_added = 0u8;
let overlaps_double = overlaps(new_robbe, double);
let clip_if_overlaps_double = !new_robbe | !overlaps_double;
clone.board.double &= clip_if_overlaps_double;
clone.board.robben &= clip_if_overlaps_double;
clone.board.friendly &= clip_if_overlaps_double;
points_added += 1 & overlaps_double.bits as u8;
```
| Perft level | Move-count | time taken | Speed |
| ----------- | ----------- | ----------- | ----------- |
Perft  7 |         8,371,299 |          15.924ms |     525.7 MNodes/sec
Perft  8 |        81,336,867 |        146.0753ms |     556.8 MNodes/sec
Perft  9 |       787,866,241 |        1.4139018s |     557.2 MNodes/sec
Perft 10 |     7,765,826,375 |        13.113308s |     592.2 MNodes/sec

> Unfortunatly, this is definitely slower than removing just one if. This means, scrap the hard work bitch. Your work is of no value.

#### Proceed by using this:
````rust
let mut points_added = 0u8;
if (new_robbe & enemy).bits != 0 {
    clone.board.enemy &= !new_robbe;
    clone.board.robben &= !new_robbe;
    clone.board.seesterne &= !new_robbe;
    clone.board.muscheln &= !new_robbe;
    clone.board.moewen &= !new_robbe;

    let overlaps = Bitboard::from(((double.bits & new_robbe.bits) != 0)as u64 * u64::MAX);
    let clip_robbe = !new_robbe | !overlaps;
    clone.board.double &= clip_robbe;
    clone.board.robben &= clip_robbe;
    clone.board.friendly &= clip_robbe;
    points_added += 1 & overlaps.bits as u8;
}
````
> 'Tis a mixture of branching and non branching code. Right in the middle.

# Other functions

## Adding points for Leicht-Figuren

````rust
pub const fn get_points_1(maximizing_player: bool, new_moewe:u64)->u8{
    ((new_moewe & 0xFF00000000000000 != 0)as u8) * (maximizing_player as u8)
                | ((new_moewe & 0xFF != 0)as u8) * (!maximizing_player as u8)
}

pub const fn get_points_2(maximizing_player: bool, new_moewe:u64)->u8{
    ((new_moewe & 0xFF00000000000000 & ((maximizing_player as u64) * u64::MAX)
        | new_moewe & 0xFF & ((!maximizing_player as u64) * u64::MAX)) != 0) as u8
}
````
> Comparing both functions in assembly:
````assembly
example::get_points_1:
        xor     ecx, ecx
        mov     rax, rsi
        shr     rax, 56
        setne   cl
        xor     eax, eax
        test    sil, sil
        setne   al
        test    edi, edi
        cmovne  eax, ecx
        ret
example::get_points_2:
        test    edi, edi
        movabs  rax, -72057594037927936
        mov     ecx, 255
        cmovne  rcx, rax
        test    rcx, rsi
        setne   al
        ret
````
> So yeah, if we measure according to how many operations we have, then `get_points_2()` is superior.

# After hooking everything up
| Perft level | Move-count | time taken | Speed |
| ----------- | ----------- | ----------- | ----------- |
Perft  0 |                 17 |   800.00ns |      21.2 MNodes/sec
Perft  1 |                289 |     1.10µs |     262.7 MNodes/sec
Perft  2 |              5,066 |    11.20µs |     452.3 MNodes/sec
Perft  3 |             88,804 |   192.40µs |     461.6 MNodes/sec
Perft  4 |          1,658,540 |     3.36ms |     493.6 MNodes/sec
Perft  5 |         30,968,180 |    62.04ms |     499.2 MNodes/sec
Perft  6 |        608,131,591 |      1.17s |     517.9 MNodes/sec
Perft  7 |     11,933,458,089 |     22.91s |     520.9 MNodes/sec
Perft  8 |    244,687,696,714 |    447.59s |     546.7 MNodes/sec

> Dayum - thats slow, lets try using lookup tables for all

| Perft level | Move-count | time taken | Speed |
| ----------- | ----------- | ----------- | ----------- |
Perft  0 |                 17 |   600.00ns |      28.3 MNodes/sec
Perft  1 |                289 |     1.10µs |     262.7 MNodes/sec
Perft  2 |              5,066 |     9.80µs |     516.9 MNodes/sec
Perft  3 |             88,804 |   169.40µs |     524.2 MNodes/sec
Perft  4 |          1,658,540 |     2.96ms |     560.7 MNodes/sec
Perft  5 |         30,968,180 |    55.16ms |     561.5 MNodes/sec
Perft  6 |        608,131,591 |      1.02s |     597.7 MNodes/sec
Perft  7 |     11,933,458,089 |     19.75s |     604.1 MNodes/sec

> Ok that greatly improved times