/*
val of 0 = air
val of 1 = rock
val of 2 = grass
val of 3 = water
*/

// UHH yeah need a new variable name for this lol
pub struct Chunk {
    smth: [i32; 262144],
}

impl Chunk {
    pub(super) fn gen(cx: i32, cz: i32, noise: fn(i32, i32) -> i32) -> Chunk {
        let mut chunk = Chunk { smth: [0; 262144] };

        for x in 0..16 {
            for z in 0..16 {
                // get the height of the surface
                let y = noise(x + (cx * 16), z + (cz * 16));

                // fill up to the surface layer with rock
                for i in 0..y {
                    chunk.smth[((x * 1024 * 16) + (z * 1024) + i) as usize] = 1;
                }

                chunk.smth[((x * 1024 * 16) + (z * 1024) + y) as usize] = 2;
            }
        }

        chunk
    }

    // fn val_at(&self, x: usize, y: usize, z: usize) -> i32 {
    //     return self.smth[(x * 1024 * 16) + (z * 1024) + y];
    // }

    pub(super) fn get_smth(&self) -> [i32; 262144] {
        self.smth
    }
}
