pub mod chunk;
mod noise;
pub mod piece;

fn gen_chunk(x: i32, z: i32) -> chunk::Chunk {
    let chunk = chunk::Chunk::gen(x, z, noise::flat);

    chunk
}

pub fn get_chunk(x: i32, z: i32) -> chunk::Chunk {
    let begin = std::time::Instant::now();
    let chunk = gen_chunk(x, z);
    let elapsed = std::time::Instant::now() - begin;
    // println!("{:?}", elapsed);

    chunk
}

pub fn setup_generator(pieces: Vec<piece::Piece>) {}
