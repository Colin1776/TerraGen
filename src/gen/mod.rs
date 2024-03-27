pub mod chunk;
mod noise;

fn gen_chunk(x: i32, z: i32) -> chunk::Chunk {
    let chunk = chunk::Chunk::gen(x, z, noise::flat);

    chunk
}

pub fn get_chunk(x: i32, z: i32) -> [i32; 262144] {
    let chunk = gen_chunk(x, z);

    chunk.get_smth()
}
