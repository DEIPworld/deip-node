#[cfg(not(feature = "octopus"))]
impl Serialize for ForceTransferred {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(4))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(FROM, &self.1)?;
        s.serialize_entry(TO, &self.2)?;
        s.serialize_entry(AMOUNT, &self.3)?;
        s.end()
    }
}

#[cfg(not(feature = "octopus"))]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct MaxZombiesChangedEvent(T::AssetId, u32);
#[cfg(not(feature = "octopus"))]
impl Serialize for MaxZombiesChanged {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry("max_zombies", &self.1)?;
        s.end()
    }
}
