pub mod chunk;
mod noise;

fn gen_base_chunk() -> chunk::Chunk {
    let chunk = chunk::Chunk::gen(0, 0, noise::flat);

    chunk
}

pub fn get_base_chunk() -> [i32; 262144] {
    let chunk = gen_base_chunk();

    chunk.get_smth()
}
