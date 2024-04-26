enum RespDataType {
    SimpleString, //  '+'
    Error, // '-' 
    Integer, // ':'
    BulkString, // '$'
    Array(Vec<RespType>), // '*'
    Null
}

