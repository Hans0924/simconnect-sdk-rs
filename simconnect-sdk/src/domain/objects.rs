use crate::bindings;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ObjectId {
    User,
}

impl From<ObjectId> for u32 {
    fn from(object_id: ObjectId) -> Self {
        match object_id {
            ObjectId::User => bindings::SIMCONNECT_OBJECT_ID_USER,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DataSetFlag {
    Default,
    Tagged,
}

impl From<DataSetFlag> for u32 {
    fn from(flag: DataSetFlag) -> Self {
        match flag {
            DataSetFlag::Default => bindings::SIMCONNECT_DATA_SET_FLAG_DEFAULT,
            DataSetFlag::Tagged => bindings::SIMCONNECT_DATA_SET_FLAG_TAGGED,
        }
    }
    
}