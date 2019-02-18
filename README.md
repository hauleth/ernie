BERT - Next Generation
======================

Simple binary data exchange format that is based on [External Term Format][ETF]
in Erlang.  This is update on [BERT][] which is a little bit outdated right now.

### Name

BERT-NG is a temporary name before I find better name (probably Trekkie joke),
if you have an idea, then we are happy to hear you out.

### RPC

No, this document will not define RPC syntax because of 2 reasons:

1. This is out of scope of this document. It is meant to define only data
   encoding format, not services one will use it for.
2. Authors of this document believes that RPC are bad, and you should not use
   them. Instead use message passing between your services.

### Schema/structured data

This is meant to encode free-form data, if you want to encode structured data
with schema then we highly suggest you to take look into other formats like:

- ASN.1 and any of it's encodings
- ProtoBuffers
- Apache Thrift
- FlatBuffers
- Cap'n'Proto

Specification
-------------

### Introduction

1. The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
   "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this
   document are to be interpreted as described in [BCP 14][bcp14].

### Types

1. Integer

    Binary representation of integers.

    Implementation MUST support at least 8-bit unsigned and 32-bit signed
    integers with full precision.

2. 64-bit Float

    IEEE 754 double precision floats.

    Implementation MUST support full IEEE 754 double precision floating point
    values.

3. Tuple

    Fixed length sequence of values.

4. List

    Dynamic length sequence of values.

5. Binary

    Representation of bytes with arbitrary length. If used to store
    human-readable strings it MUST be encoded as correct UTF-8 string.

6. Map

    List of key-value pairs in arbitrary order. Keys MUST NOT appear twice in
    the same map.

### Encoding

1. Top level

    Binary encoded data MUST start with magic byte of decimal value of `131`
    followed by single byte `Tag` value followed by arbitrary length data:

        0        1       2
        +--------+-------+-------+-------
        | 131    | Tag   | Data ...
        +--------+-------+-------+-------

2. Integer

    Integer can be encoded in 3 different forms:

    - Values in range 0..255 (aka byte)

            0       1        2
            +-------+--------+
            | 97    | Data   |
            +-------+--------+

        Where `Data` is unsigned byte representation of value.

    - Values in range -4 294 967 295..4 294 967 294 (signed 32-bit value)

            0       1       2      3      4      5
            +-------+-------+------+------+------+
            | 98    | Data                       |
            +-------+-------+------+------+------+

        Where `Data` is signed 32-bit representation of integer in U2 big-endian
        encoding.

    - "Short" arbitrary long signed integers (up to 2040-bit)

            0       1       2       3     ...     N-3
            +-------+-------+-------+------+------+
            | 110   | N     | Sign  | Data        |
            +-------+-------+-------+------+------+

        `N` is big-endian 16-bit unsigned integer. `Data` represents big-endian
        encoded `N` byte long integer. `Sign` is `0` for positive integer
        and `1` for negative one.

    - "Long" arbitrary long signed integers (from 2041 to 524 288-bit long)

            0       1       2       3       4     ...     N-4
            +-------+-------+-------+-------+------+------+
            | 111   | N             | Sign  | Data        |
            +-------+-------+-------+-------+------+------+

        `N` is big-endian 16-bit unsigned integer. `Data` represents big-endian
        encoded `N` byte long integer. `Sign` is `0` for positive integer
        and `1` for negative one.

3. Float

        0       1       2        3       4       5       6       7       8       9
        +-------+-------+--------+-------+-------+-------+-------+-------+-------+
        | 70    | Data                                                           |
        +-------+-------+--------+-------+-------+-------+-------+-------+-------+

    Where `Data` is IEEE 754 double precision floating-point number encoded as
    big-endian.

4. Tuple

    - Tuples up to 255 elements

            0       1       2       ...       N
            +-------+-------+--------+--------+
            | 104   | Arity | Data ...        |
            +-------+-------+--------+--------+

    - Tuples up to 4 294 967 295 elements

            0       1       2        3        4        5       ...       N
            +-------+-------+--------+--------+--------+--------+--------+
            | 105   | Arity                            | Data ...        |
            +-------+-------+--------+--------+--------+--------+--------+

    Where `Arity` is big-endian unsigned integer declaring amount of elements in
    `Data`.

5. List

        0       1       2       3       4       5      ...      N       N+1
        +-------+-------+-------+-------+-------+-------+-------+-------+
        | 108   | Arity                         | Data ...      | 106   |
        +-------+-------+-------+-------+-------+-------+-------+-------+

    With special case for empty list:

        0       1
        +-------+
        | 106   |
        +-------+

    Where `Arity` is big-endian unsigned 32-bit integer declaring amount
    of elements in `Data`.

6. Binary

        0       1       2       3       4       5      ...      N-5
        +-------+-------+-------+-------+-------+-------+-------+
        | 109   | N                             | Data ...      |
        +-------+-------+-------+-------+-------+-------+-------+

    Where `N` is length of the `Data` in bytes.

7. Map

        0       1       2       3       4       5      ...      N
        +-------+-------+-------+-------+-------+-------+-------+
        | 116   | Arity                         | Data ...      |
        +-------+-------+-------+-------+-------+-------+-------+

    The `Arity` field is an unsigned 32-bit integer in big-endian format that
    determines the number of key-value pairs in the map. Key and value pairs
    (`Ki => Vi`) are encoded in section `Data` in the following order:
    `K1, V1, K2, V2,..., Kn, Vn`.

Open questions
--------------

1. Should we support atoms?
2. Should we support byte lists (aka Erlang strings)?
3. Should we allow defining improper lists?

License
-------

This document is released on [Creative Commons Attribution-ShareAlike 4.0
International License](http://creativecommons.org/licenses/by-sa/4.0/).
