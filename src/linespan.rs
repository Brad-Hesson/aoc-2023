use std::{marker::PhantomData, ops::Range};

use line_span::LineSpans;
use winnow::{
    combinator::fail,
    error::ParserError,
    stream::{Location, Offset, Stream, StreamIsPartial},
    PResult, Parser,
};

use std::fmt::Debug;

#[derive(Debug)]
pub struct LineSpan {
    pub line: usize,
    pub span: Range<usize>,
}

pub fn linespan<P, O>(parser: P) -> LineSpanP<P, O> {
    LineSpanP {
        parser,
        o: Default::default(),
    }
}

pub struct LineSpanP<P, O> {
    parser: P,
    o: PhantomData<O>,
}

impl<'i, P, O: Debug, E> Parser<LineLocated<'i>, LineSpan, E> for LineSpanP<P, O>
where
    P: Parser<LineLocated<'i>, O, E>,
    E: ParserError<LineLocated<'i>>,
{
    fn parse_next(&mut self, input: &mut LineLocated<'i>) -> PResult<LineSpan, E> {
        let start = input.location();
        let line = input
            .lines
            .binary_search_by_key(&start, |r| r.end - 1)
            .unwrap_or_else(|a| a);
        if line == input.lines.len() {
            return fail.parse_next(input);
        }
        let line_start = input.lines[line].start;
        self.parser.parse_next(input).map(move |_| {
            let end = input.location();
            LineSpan {
                line,
                span: (start - line_start..end - line_start),
            }
        })
    }
}

pub fn with_linespan<P>(parser: P) -> WithLinespanP<P> {
    WithLinespanP { parser }
}
pub struct WithLinespanP<P> {
    parser: P,
}
impl<'i, P, O, E> Parser<LineLocated<'i>, (O, LineSpan), E> for WithLinespanP<P>
where
    P: Parser<LineLocated<'i>, O, E>,
    E: ParserError<LineLocated<'i>>,
{
    fn parse_next(&mut self, input: &mut LineLocated<'i>) -> PResult<(O, LineSpan), E> {
        let start = input.location();
        let line = input
            .lines
            .binary_search_by_key(&start, |r| r.end - 1)
            .unwrap_or_else(|a| a);
        if line == input.lines.len() {
            return fail.parse_next(input);
        }
        let line_start = input.lines[line].start;
        self.parser.parse_next(input).map(move |output| {
            let end = input.location();
            (
                output,
                LineSpan {
                    line,
                    span: (start - line_start..end - line_start),
                },
            )
        })
    }
}

pub struct LineLocated<'i> {
    initial: &'i str,
    input: &'i str,
    lines: Vec<Range<usize>>,
}
impl<'i> LineLocated<'i> {
    pub fn new(input: &'i str) -> Self {
        let lines = input.line_spans().map(|ls| ls.range_with_ending()).collect();
        LineLocated {
            lines,
            initial: input,
            input,
        }
    }
}
impl<'i> Debug for LineLocated<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.input.fmt(f)
    }
}
impl<'i> Location for LineLocated<'i> {
    fn location(&self) -> usize {
        self.input.offset_from(&self.initial)
    }
}
impl<'i> Offset<<&str as Stream>::Checkpoint> for LineLocated<'i> {
    fn offset_from(&self, start: &<&str as Stream>::Checkpoint) -> usize {
        self.input.offset_from(start)
    }
}
impl<'i> Stream for LineLocated<'i> {
    type Token = <&'i str as Stream>::Token;

    type Slice = <&'i str as Stream>::Slice;

    type IterOffsets = <&'i str as Stream>::IterOffsets;

    type Checkpoint = <&'i str as Stream>::Checkpoint;

    fn iter_offsets(&self) -> Self::IterOffsets {
        self.input.iter_offsets()
    }

    fn eof_offset(&self) -> usize {
        self.input.eof_offset()
    }

    fn next_token(&mut self) -> Option<Self::Token> {
        self.input.next_token()
    }

    fn offset_for<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Token) -> bool,
    {
        self.input.offset_for(predicate)
    }

    fn offset_at(&self, tokens: usize) -> Result<usize, winnow::error::Needed> {
        self.input.offset_at(tokens)
    }

    fn next_slice(&mut self, offset: usize) -> Self::Slice {
        self.input.next_slice(offset)
    }

    fn checkpoint(&self) -> Self::Checkpoint {
        self.input.checkpoint()
    }

    fn reset(&mut self, checkpoint: Self::Checkpoint) {
        self.input.reset(checkpoint)
    }

    fn raw(&self) -> &dyn std::fmt::Debug {
        self.input.raw()
    }
}
impl<'i> StreamIsPartial for LineLocated<'i> {
    type PartialState = <&'i str as StreamIsPartial>::PartialState;

    fn complete(&mut self) -> Self::PartialState {
        self.input.complete()
    }

    fn restore_partial(&mut self, state: Self::PartialState) {
        self.input.restore_partial(state)
    }

    fn is_partial_supported() -> bool {
        <&'i str as StreamIsPartial>::is_partial_supported()
    }
}
