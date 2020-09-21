# binstruct

Automatically generate safe byte (de)serialize code for you at compile-time.

## Example

```rust
#[derive(Copy, Clone, BinEncode, PartialEq)]
#[binstruct(enum_repr(u8))]
enum GameStatus {
  #[binstruct(value = 1)]
  Pending,
  #[binstruct(value = 2)]
  Started,
}

#[derive(BinEncode, BinDecode)]
struct Slot {
  player_id: u8,
}

#[derive(BinEncode, BinDecode)]
struct Player {
  id: u8,
  name: CString,
}

#[derive(BinEncode, BinDecode)]
struct Game {
  #[binstruct(eq = 0xF7)]
  magic: u8,
  version: u8,
  name: CString,
  num_players: u8,
  #[binstruct(repeat = "num_players")]
  players: Vec<Player>,
  slots: [Slot; 24],
  #[binstruct(condition = "version == 2")]
  v2_field: Option<u8>,
}
```

will generate

```rust
#[binstruct(enum_repr(u8))]
enum GameStatus {
    #[binstruct(value = 1)]
    Pending,
    #[binstruct(value = 2)]
    Started,
}

impl binstruct::serialize::BinEncode for GameStatus {
    fn encode<T: binstruct::serialize::BufMut>(&self, buf: &mut T) {
        <u8 as binstruct::serialize::BinEncode>::encode(&u8::from(*self), buf);
    }
}
impl From<GameStatus> for u8 {
    fn from(v: GameStatus) -> u8 {
        match v {
            GameStatus::Pending => 1,
            GameStatus::Started => 2,
        }
    }
}

struct Slot {
    player_id: u8,
}

impl binstruct::serialize::BinEncode for Slot {
    fn encode<T: binstruct::serialize::BufMut>(&self, buf: &mut T) {
        <u8 as binstruct::serialize::BinEncode>::encode(&self.player_id, buf);
    }
}

impl binstruct::serialize::BinDecode for Slot {
    const MIN_SIZE: usize = <u8 as binstruct::serialize::BinDecode>::MIN_SIZE;
    const FIXED_SIZE: bool = <u8 as binstruct::serialize::BinDecode>::FIXED_SIZE;
    fn decode<T: binstruct::serialize::Buf>(
        buf: &mut T,
    ) -> Result<Self, binstruct::serialize::BinDecodeError> {
        if buf.remaining() < Self::MIN_SIZE {
            return Err(binstruct::serialize::BinDecodeError::incomplete());
        }
        let player_id = { <u8 as binstruct::serialize::BinDecode>::decode(buf)? };
        Ok(Self { player_id })
    }
}

struct Player {
    id: u8,
    name: CString,
}

impl binstruct::serialize::BinEncode for Player {
    fn encode<T: binstruct::serialize::BufMut>(&self, buf: &mut T) {
        <u8 as binstruct::serialize::BinEncode>::encode(&self.id, buf);
        <CString as binstruct::serialize::BinEncode>::encode(&self.name, buf);
    }
}

impl binstruct::serialize::BinDecode for Player {
    const MIN_SIZE: usize = <u8 as binstruct::serialize::BinDecode>::MIN_SIZE
        + <CString as binstruct::serialize::BinDecode>::MIN_SIZE;
    const FIXED_SIZE: bool = <u8 as binstruct::serialize::BinDecode>::FIXED_SIZE
        && <CString as binstruct::serialize::BinDecode>::FIXED_SIZE;
    fn decode<T: binstruct::serialize::Buf>(
        buf: &mut T,
    ) -> Result<Self, binstruct::serialize::BinDecodeError> {
        if buf.remaining() < Self::MIN_SIZE {
            return Err(binstruct::serialize::BinDecodeError::incomplete());
        }
        let id = { <u8 as binstruct::serialize::BinDecode>::decode(buf)? };
        let name = {
            if <CString as binstruct::serialize::BinDecode>::MIN_SIZE > 0
                && !<u8 as binstruct::serialize::BinDecode>::FIXED_SIZE
            {
                if buf.remaining() < <CString as binstruct::serialize::BinDecode>::MIN_SIZE {
                    return Err(binstruct::serialize::BinDecodeError::incomplete());
                }
            }
            <CString as binstruct::serialize::BinDecode>::decode(buf)?
        };
        Ok(Self { id, name })
    }
}

struct Game {
    #[binstruct(eq = 0xF7)]
    magic: u8,
    version: u8,
    name: CString,
    num_players: u8,
    #[binstruct(repeat = "num_players")]
    players: Vec<Player>,
    slots: [Slot; 24],
    #[binstruct(condition = "version == 2")]
    v2_field: Option<u8>,
}

impl binstruct::serialize::BinEncode for Game {
    fn encode<T: binstruct::serialize::BufMut>(&self, buf: &mut T) {
        <u8 as binstruct::serialize::BinEncode>::encode(&self.magic, buf);
        <u8 as binstruct::serialize::BinEncode>::encode(&self.version, buf);
        <CString as binstruct::serialize::BinEncode>::encode(&self.name, buf);
        <u8 as binstruct::serialize::BinEncode>::encode(&self.num_players, buf);
        <Vec<Player> as binstruct::serialize::BinEncode>::encode(&self.players, buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[0usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[1usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[2usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[3usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[4usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[5usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[6usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[7usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[8usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[9usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[10usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[11usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[12usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[13usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[14usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[15usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[16usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[17usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[18usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[19usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[20usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[21usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[22usize], buf);
        <Slot as binstruct::serialize::BinEncode>::encode(&self.slots[23usize], buf);
        if self.version == 2 {
            if let Some(ref v2_field) = self.v2_field {
                <u8 as binstruct::serialize::BinEncode>::encode(v2_field, buf);
            }
        }
    }
}

impl binstruct::serialize::BinDecode for Game {
    const MIN_SIZE: usize = <u8 as binstruct::serialize::BinDecode>::MIN_SIZE
        + <u8 as binstruct::serialize::BinDecode>::MIN_SIZE
        + <CString as binstruct::serialize::BinDecode>::MIN_SIZE
        + <u8 as binstruct::serialize::BinDecode>::MIN_SIZE
        + 0
        + (<Slot as binstruct::serialize::BinDecode>::MIN_SIZE * (24))
        + 0;
    const FIXED_SIZE: bool = <u8 as binstruct::serialize::BinDecode>::FIXED_SIZE
        && <u8 as binstruct::serialize::BinDecode>::FIXED_SIZE
        && <CString as binstruct::serialize::BinDecode>::FIXED_SIZE
        && <u8 as binstruct::serialize::BinDecode>::FIXED_SIZE
        && false
        && <Slot as binstruct::serialize::BinDecode>::FIXED_SIZE
        && false;
    fn decode<T: binstruct::serialize::Buf>(
        buf: &mut T,
    ) -> Result<Self, binstruct::serialize::BinDecodeError> {
        if buf.remaining() < Self::MIN_SIZE {
            return Err(binstruct::serialize::BinDecodeError::incomplete());
        }
        let magic = { <u8 as binstruct::serialize::BinDecode>::decode(buf)? };
        if magic != 0xF7 {
            return Err(binstruct::serialize::BinDecodeError::failure({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &[
                        "unexpected value for field `",
                        "`, expected `",
                        "`, got `",
                        "`",
                    ],
                    &match (&"magic", &0xF7, &magic) {
                        (arg0, arg1, arg2) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Debug::fmt),
                        ],
                    },
                ));
                res
            }));
        }
        let version = {
            if <u8 as binstruct::serialize::BinDecode>::MIN_SIZE
                + <CString as binstruct::serialize::BinDecode>::MIN_SIZE
                + <u8 as binstruct::serialize::BinDecode>::MIN_SIZE
                + 0
                + (<Slot as binstruct::serialize::BinDecode>::MIN_SIZE * (24))
                + 0
                > 0
                && !<u8 as binstruct::serialize::BinDecode>::FIXED_SIZE
            {
                if buf.remaining()
                    < <u8 as binstruct::serialize::BinDecode>::MIN_SIZE
                        + <CString as binstruct::serialize::BinDecode>::MIN_SIZE
                        + <u8 as binstruct::serialize::BinDecode>::MIN_SIZE
                        + 0
                        + (<Slot as binstruct::serialize::BinDecode>::MIN_SIZE * (24))
                        + 0
                {
                    return Err(binstruct::serialize::BinDecodeError::incomplete());
                }
            }
            <u8 as binstruct::serialize::BinDecode>::decode(buf)?
        };
        let name = {
            if <CString as binstruct::serialize::BinDecode>::MIN_SIZE
                + <u8 as binstruct::serialize::BinDecode>::MIN_SIZE
                + 0
                + (<Slot as binstruct::serialize::BinDecode>::MIN_SIZE * (24))
                + 0
                > 0
                && !<u8 as binstruct::serialize::BinDecode>::FIXED_SIZE
            {
                if buf.remaining()
                    < <CString as binstruct::serialize::BinDecode>::MIN_SIZE
                        + <u8 as binstruct::serialize::BinDecode>::MIN_SIZE
                        + 0
                        + (<Slot as binstruct::serialize::BinDecode>::MIN_SIZE * (24))
                        + 0
                {
                    return Err(binstruct::serialize::BinDecodeError::incomplete());
                }
            }
            <CString as binstruct::serialize::BinDecode>::decode(buf)?
        };
        let num_players = {
            if <u8 as binstruct::serialize::BinDecode>::MIN_SIZE
                + 0
                + (<Slot as binstruct::serialize::BinDecode>::MIN_SIZE * (24))
                + 0
                > 0
                && !<CString as binstruct::serialize::BinDecode>::FIXED_SIZE
            {
                if buf.remaining()
                    < <u8 as binstruct::serialize::BinDecode>::MIN_SIZE
                        + 0
                        + (<Slot as binstruct::serialize::BinDecode>::MIN_SIZE * (24))
                        + 0
                {
                    return Err(binstruct::serialize::BinDecodeError::incomplete());
                }
            }
            <u8 as binstruct::serialize::BinDecode>::decode(buf)?
        };
        let players = {
            if 0 + (<Slot as binstruct::serialize::BinDecode>::MIN_SIZE * (24)) + 0 > 0
                && !<u8 as binstruct::serialize::BinDecode>::FIXED_SIZE
            {
                if buf.remaining()
                    < 0 + (<Slot as binstruct::serialize::BinDecode>::MIN_SIZE * (24)) + 0
                {
                    return Err(binstruct::serialize::BinDecodeError::incomplete());
                }
            }
            binstruct::serialize::BinBufExt::get_repeated(buf, (num_players) as usize)?
        };
        let slots = {
            if (<Slot as binstruct::serialize::BinDecode>::MIN_SIZE * (24)) + 0 > 0 && !false {
                if buf.remaining()
                    < (<Slot as binstruct::serialize::BinDecode>::MIN_SIZE * (24)) + 0
                {
                    return Err(binstruct::serialize::BinDecodeError::incomplete());
                }
            }
            [
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
                <Slot as binstruct::serialize::BinDecode>::decode(buf)?,
            ]
        };
        let v2_field = {
            if 0 > 0 && !<Slot as binstruct::serialize::BinDecode>::FIXED_SIZE {
                if buf.remaining() < 0 {
                    return Err(binstruct::serialize::BinDecodeError::incomplete());
                }
            }
            if version == 2 {
                if <u8 as binstruct::serialize::BinDecode>::MIN_SIZE > 0
                    && buf.remaining() < <u8 as binstruct::serialize::BinDecode>::MIN_SIZE
                {
                    return Err(binstruct::serialize::BinDecodeError::incomplete());
                }
                Some(<u8 as binstruct::serialize::BinDecode>::decode(buf)?)
            } else {
                None
            }
        };
        Ok(Self {
            magic,
            version,
            name,
            num_players,
            players,
            slots,
            v2_field,
        })
    }
}
```
