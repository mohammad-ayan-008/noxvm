use chunk::Chunk;

mod chunk;
mod value;

fn main() {
    let mut chunk = Chunk::new();



    chunk.write_chunk(chunk::OpCode::Op_Constnats as u8, 1);

    let val = chunk.addConstant(1.2);
    chunk.write_chunk(val as u8 , 1);
   
    chunk.write_chunk(chunk::OpCode::Op_Constnats as u8, 1);
     let val = chunk.addConstant(1.7);
    chunk.write_chunk(val as u8 , 1);

    chunk.write_chunk(chunk::OpCode::Return as u8, 2);

    chunk.disassembleChunk("test chunk");
}
