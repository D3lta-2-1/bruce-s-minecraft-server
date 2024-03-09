use std::io::{
    prelude::{Read, Write},
    Result,
};

use super::{
    encoding::{Decoder, Encoder},
    var_string::{VarStringRead, VarStringWrite},
};

#[derive(derive_more::Deref, derive_more::From, derive_more::Into)]
pub struct Identifier(Var32767String);

#[derive(Debug, derive_more::From)]
pub struct Var32767String(String);

pub trait ToIdentifier {
    fn to_identifier(&self) -> Identifier;
}

impl<S: ToString> ToIdentifier for S {
    fn to_identifier(&self) -> Identifier {
        Identifier(self.to_string().into())
    }
}

impl Decoder for Var32767String {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Identifier(reader.read_var_string::<32767>()?.into()).into())
    }
}

impl Encoder for Var32767String {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_string(&self.0)?;
        Ok(())
    }
}

pub trait ReadIdentifier {
    fn read_identifier(&mut self) -> Result<Identifier>;
}

pub trait WriteIdentifier {
    fn write_identifier(&mut self, value: Identifier) -> Result<()>;
}

impl<R: Read> ReadIdentifier for R {
    fn read_identifier(&mut self) -> Result<Identifier> {
        Ok(Identifier(self.read_var_string::<32767>()?.into()).into())
    }
}
