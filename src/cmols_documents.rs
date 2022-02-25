use uuid::Uuid;
use byte_array::ByteArray;

struct CmolsDocument {
    id:Uuid,
    name:str,
    content:ByteArray,
}


//pub type CmolsDocuments = Response<CmolsDocument>;