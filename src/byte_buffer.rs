use std::{
    convert::TryInto,
    error::Error,
    io::{self, Cursor, Read},
};

pub struct ByteBuffer {
    buffer: Cursor<Vec<u8>>,
}

impl ByteBuffer {
    // Initialize a new ByteBuffer with the given data
    pub fn new(data: Vec<u8>) -> Self {
        ByteBuffer {
            buffer: Cursor::new(data),
        }
    }

    // Returns the current position of the cursor
    pub fn byte_offset(&self) -> u64 {
        self.buffer.position()
    }

    // Returns the number of bytes remaining from the current cursor position
    pub fn bytes_remaining(&self) -> u32 {
        (self.buffer.get_ref().len() as u64 - self.buffer.position()) as u32
    }

    // Reads a string of the specified length from the buffer
    pub fn read_string(&mut self, length: usize) -> io::Result<String> {
        let mut buffer = vec![0; length];
        self.buffer.read_exact(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

    pub fn read_string_ascii(&mut self, length: usize) -> io::Result<String> {
        let mut buffer = vec![0; length];
        self.buffer.read_exact(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

    pub fn read_string_utf16_be(&mut self, length: usize) -> Result<String, Box<dyn Error>> {
        if length % 2 != 0 {
            return Err("Length must be even, as UTF-16 characters are 2 bytes".into());
        }

        let mut buf = vec![0; length];
        self.buffer.read_exact(&mut buf)?;
        // Convert the byte buffer to a Vec<u16>, interpreting each pair of bytes as a big endian u16
        let u16_buffer: Vec<u16> = buf
            .chunks(2)
            .map(|chunk| u16::from_be_bytes(chunk.try_into().expect("Chunk should be of length 2")))
            .collect();

        // Convert the Vec<u16> into a String
        let s = String::from_utf16(&u16_buffer)?;
        Ok(s)
    }
    // Reads a u32 from the buffer
    pub fn read_uint32(&mut self) -> io::Result<u32> {
        let mut buffer = [0; 4];
        self.buffer.read_exact(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }

    // Reads a u64 from the buffer
    pub fn read_uint64(&mut self) -> io::Result<u64> {
        let mut buffer = [0; 8];
        self.buffer.read_exact(&mut buffer)?;
        Ok(u64::from_be_bytes(buffer))
    }

    // Reads a u8 from the buffer
    pub fn read_uint8(&mut self) -> io::Result<u8> {
        let mut buffer = [0; 1];
        self.buffer.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    // Reads a u8 array of the specified length from the buffer
    pub fn read_uint8_array(&mut self, data_length: usize) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0; data_length];
        self.buffer.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    // Resets the cursor to the beginning of the buffer
    pub fn reset(&mut self) {
        self.buffer.set_position(0);
    }

    // Skips the cursor forward by the specified number of bytes
    pub fn skip(&mut self, next_cursor: u64) {
        let new_position = self.buffer.position() + next_cursor;
        self.buffer.set_position(new_position);
    }
}

// fn main() -> io::Result<()> {
//     let data = b"Hello, world! 1234567890abcdef".to_vec();
//     let mut buffer = ByteBuffer::new(data);

//     println!("Byte offset: {}", buffer.byte_offset());
//     println!("String: {}", buffer.read_string(5)?);
//     println!("Byte offset: {}", buffer.byte_offset());
//     buffer.skip(2);
//     println!("String: {}", buffer.read_string(6)?);
//     println!("Uint32: {}", buffer.read_uint32()?);
//     println!("Uint64: {}", buffer.read_uint64()?);
//     println!("Uint8: {}", buffer.read_uint8()?);
//     let uint8_array = buffer.read_uint8_array(4)?;
//     println!("Uint8 array: {:?}", uint8_array);
//     buffer.reset();
//     println!("Reset Byte offset: {}", buffer.byte_offset());

//     Ok(())
// }
