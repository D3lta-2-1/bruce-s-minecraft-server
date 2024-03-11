use std::io::{prelude::Write, Result};

use uuid::Uuid;

use crate::{
    io::prelude::{
        Encoder, F64Write, Identifier, U8Write, UuidWrite, VarIntSizedVecWrite, VarIntWrite,
    },
    net::prelude::{PacketId, Socket},
    server::prelude::GamePlayer,
};

pub struct UpdateAttributes {
    pub entity_id: i32,
    pub properties: Vec<AttributeProperty>,
}

impl Encoder for UpdateAttributes {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_var_i32(self.entity_id)?;
        buf.write_var_int_sized_vec(&self.properties)?;
        Ok(())
    }
}

pub struct AttributeProperty {
    pub key: Identifier,
    pub value: f64,
    pub modifiers: Vec<ModifierData>,
}

impl Encoder for AttributeProperty {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        self.key.encode_to_buffer(buf)?;
        buf.write_f64(self.value)?;
        buf.write_var_int_sized_vec(&self.modifiers)?;
        Ok(())
    }
}

pub struct ModifierData {
    pub uuid: Uuid,
    pub amount: f64,
    pub operation: ModifierOperation,
}

impl Encoder for ModifierData {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_uuid(self.uuid)?;
        buf.write_f64(self.amount)?;
        self.operation.encode_to_buffer(buf)?;
        Ok(())
    }
}

pub enum ModifierOperation {
    Add,
    Precentage,
    Multiply,
}

impl Encoder for ModifierOperation {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_u8(match self {
            ModifierOperation::Add => 0,
            ModifierOperation::Precentage => 1,
            ModifierOperation::Multiply => 2,
        })?;
        Ok(())
    }
}
