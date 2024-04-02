pub(super) fn flat(_x: i32, _z: i32) -> i32 {
    // if _x == 0 && _z == 0 {
    //     0
    // } else if _x == 1 && _z == 0 {
    //     -1
    // } else {
    //     -1
    // }

    if _x >= 0 && _x < 16 && _z >= 00 && _z < 16 {
        return 14;
    }

    10
}
