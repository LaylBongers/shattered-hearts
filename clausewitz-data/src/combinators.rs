use combine::*;
use ::{CwValue, CwTable, CwKeyValue};

fn nl_ws<I>(input: I) -> ParseResult<(), I>
    where I: Stream<Item=char>
{
    let comment = (token('#'), skip_many(satisfy(|c| c != '\n'))).map(|_| ());;
    let mut nl_ws = space().or(newline()).map(|_| ()).or(comment);

    nl_ws.parse_state(input)
}

fn word<I>(input: I) -> ParseResult<String, I>
    where I: Stream<Item=char>
{
    let word_char = satisfy(|c: char|
        c.is_alphanumeric() || c == '.' || c == '_' || c == '-'
    );
    let word = many1::<String, _>(word_char);

    word.expected("word").parse_state(input)
}

fn escape_char(c: char) -> char {
    match c {
        '\'' => '\'',
        '"' => '"',
        '\\' => '\\',
        '/' => '/',
        'b' => '\u{0008}',
        'f' => '\u{000c}',
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        c => c,//Should never happen
    }
}

fn string_char<I>(input: I) -> ParseResult<char, I>
    where I: Stream<Item=char>
{
    let (c, input) = try!(any().parse_lazy(input));
    let mut back_slash_char = satisfy(|c| "\"\\/bfnrt".chars().find(|x| *x == c).is_some())
                                 .map(escape_char);
    match c {
        '\\' => input.combine(|input| back_slash_char.parse_state(input)),
        '"' => unexpected("\"").parse_state(input.into_inner()).map(|_| unreachable!()),
        _ => Ok((c, input)),
    }
}

fn string_literal<I>(input: I) -> ParseResult<String, I>
    where I: Stream<Item=char>
{
    let literal = between(
        string("\""),
        string("\""),
        many(parser(string_char))
    ).map(|v| v);

    literal.expected("string literal").parse_state(input)
}

fn eu4value<I>(input: I) -> ParseResult<CwValue, I>
    where I: Stream<Item=char>
{
    let value =
        parser(word)
            .map(|v| CwValue::String(v))
        .or(parser(string_literal)
            .map(|v| CwValue::String(v)))
        .or((token('{'), parser(table), token('}'))
            .map(|v| {
                let table = v.1;

                // Devolve table to array if keyless
                // TODO: Perhaps instead use try() to attempt to parse a table first, then an array
                if table.values.iter().all(|v| v.key == "") {
                    CwValue::Array(table.values.into_iter().map(|v| v.value).collect())
                } else {
                    CwValue::Table(table)
                }
            }));

    value.expected("value").parse_state(input)
}

fn key_value<I>(input: I) -> ParseResult<CwKeyValue, I>
    where I: Stream<Item=char>
{
    let key_value = (parser(word), spaces(), token('='), spaces(), parser(eu4value))
        .map(|v| CwKeyValue {
            key: v.0,
            value: v.4,
        });

    key_value.expected("key-value").parse_state(input)
}

fn keyless_value<I>(input: I) -> ParseResult<CwKeyValue, I>
    where I: Stream<Item=char>
{
    let key_value = parser(eu4value)
        .map(|v| CwKeyValue {
            key: "".into(),
            value: v,
        });

    key_value.expected("keyless value").parse_state(input)
}

fn table<I>(input: I) -> ParseResult<CwTable, I>
    where I: Stream<Item=char>
{
    let table = many(try(parser(key_value)).or(parser(keyless_value)).skip(skip_many(parser(nl_ws))))
        .map(|v| {
            CwTable {
                values: v
            }
        });

    (skip_many(parser(nl_ws)), table).map(|v| v.1).parse_state(input)
}

fn eu4data<I>(input: I) -> ParseResult<CwTable, I>
    where I: Stream<Item=char>
{
    parser(table).parse_state(input)
}

pub fn parse(text: &str) -> CwTable {
    parser(eu4data).parse(text).unwrap().0
}
