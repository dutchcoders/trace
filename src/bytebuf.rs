use std::fmt;

pub struct ByteBuf<'a>(pub &'a [u8]);

impl<'a> fmt::LowerHex for ByteBuf<'a> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0 {
            try!(fmtr.write_fmt(format_args!("{:02x}", byte)));
        }

        Ok(())
    }
}
