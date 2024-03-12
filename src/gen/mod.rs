pub mod chunk;

fn gen_base_chunk() -> chunk::Chunk {
    let chunk = chunk::Chunk::gen();

    chunk
}

pub fn get_base_chunk() -> [i32; 262144] {
    let chunk = gen_base_chunk();

    chunk.get_smth()
}
