# Problem

I need a Record structure written in rust. The structure has the fields id:i128, created unix timestamp, updated unix timestamp, deleted unix timestamp. data: utf8 array, checksum : sha256 fingerprint of the data array.
The struct is serializable and deserializable to json and csv.

