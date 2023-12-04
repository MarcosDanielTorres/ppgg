#[derive(Debug)]
pub struct Decoder<'de> {
    buf: &'de [u8],
    offset: usize,
    validate: bool,
}

impl<'de> Decoder<'de> {
    pub fn new(buf: &[u8], validate: bool) -> Decoder {
        Decoder {
            buf,
            offset: 0,
            validate,
        }
    }
}

pub enum DynSolType {
    Bool,
}

#[derive(Debug)]
pub enum DynSolValue {
    Bool(bool),
}

pub enum DynToken<'a> {
    Nothing,
    PackedSeq(&'a [u8]),
}

pub fn abi_decode_sequence(data: &[u8], decoder: &mut Decoder) -> DynSolValue {
    let val = abi_decode_inner(decoder, decode_sequence_populate);
    println!("{:?}", val);
    DynSolValue::Bool(false)
}

pub fn decode_sequence_populate<'d>(token: &mut DynToken<'d>, decoder: &mut Decoder<'d>) {
    println!("decode_sequence_populate");
}

/*
NO ANDA:
pub(crate) fn abi_decode_inner<'d, F>(
    decoder: &'d mut Decoder<'d>,
    f: F,
) -> &'d mut Decoder<'d>

pub(crate) fn abi_decode_inner<'d, 'b, F>(
    decoder: &'b mut Decoder<'b>,
    f: F,
) -> &'d mut Decoder<'d>

pub(crate) fn abi_decode_inner<'d, 'b, F>(
    decoder: &'b mut Decoder<'d>,
    f: F,
) -> &'d mut Decoder<'b>


parece que solo anda con el ejemplo de abajo
SI ANDA:
pub(crate) fn abi_decode_inner<'d, 'b, F>(
    decoder: &'b mut Decoder<'d>,
    f: F,
) -> &'b mut Decoder<'d>
*/
pub(crate) fn abi_decode_inner<'d, 'b, F>(decoder: &'b mut Decoder<'b>, f: F) -> &'b mut Decoder<'d>
where
    F: FnOnce(&mut DynToken<'d>, &mut Decoder<'d>) -> (),
{
    // let mut token = empty_dyn_token();
    let mut token = DynToken::Nothing;
    f(&mut token, decoder);
    // let value = detokenize(token).expect("invalid empty_dyn_token");
    // let value = detokenize(token);
    // value
    decoder
}

pub fn detokenize(token: DynToken) -> DynSolValue {
    match token {
        DynToken::Nothing => DynSolValue::Bool(false),
        DynToken::PackedSeq(data) => DynSolValue::Bool(true),
    }
}

fn main() {
    let data: &[u8; 3] = &[1, 2, 3];
    let xd = &mut Decoder::new(data, false);
    let res = abi_decode_sequence(data, xd);
    println!("{:?}", res);
}
