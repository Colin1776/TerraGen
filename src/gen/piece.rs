pub enum Material {
    AIR,
    ROCK,
    SOIL,
    WATER,
}

/*
So, i want generator to generate stone for the ground underneath, dirt for a few blocks, and then grass at the surface
But the generator is supposed to be flexible so it should be able to generate any combinatoin of any block in many numbers of different biomes
i also want some of the stone to be replaced with different types of rock like in mc sometimes, and stone should generate above ground too in some cases
right now i dont need to worry abt this actually cuz i just need to have like stone, and grass or something for basic terrain testing i think
*/

pub struct Piece {
    pub name: String,
    pub material: Material,
}
