use super::*;
use add::Add;
use bitwise_and::BitwiseAnd;
use bitwise_or::BitwiseOr;
use div::Div;
use equal::Equal;
use fn_acos::FnAcos;
use fn_acosh::FnAcosh;
use fn_asin::FnAsin;
use fn_asinh::FnAsinh;
use fn_atan::FnAtan;
use fn_atanh::FnAtanh;
use fn_avg::FnAvg;
use fn_ceil::FnCeil;
use fn_cos::FnCos;
use fn_cosh::FnCosh;
use fn_floor::FnFloor;
use fn_if::FnIf;
use fn_ln::FnLn;
use fn_log::FnLog;
use fn_log10::FnLog10;
use fn_log2::FnLog2;
use fn_pow::FnPow;
use fn_ref::FnRef;
use fn_round::FnRound;
use fn_sin::FnSin;
use fn_sinh::FnSinh;
use fn_sqrt::FnSqrt;
use fn_sum::FnSum;
use fn_tan::FnTan;
use fn_tanh::FnTanh;
use greater_than::GreaterThan;
use greater_than_equal::GreaterThanEqual;
use left_shift::LeftShift;
use less_than::LessThan;
use less_than_equal::LessThanEqual;
use logical_and::LogicalAnd;
use logical_or::LogicalOr;
use minus::Minus;
use modulo::Mod;
use mul::Mul;
use not::Not;
use not_equal::NotEqual;
use right_shift::RightShift;
use sub::Sub;
use xor::Xor;

pub fn parse(seq: &Vec<Token>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>) {
    if seq.first() == Some(&Token::ParenthesisEnd) && seq.last() == Some(&Token::ParenthesisBegin) {
        let inside_parentheses = &seq[1..seq.len() - 1];
        let (_, has_depth_zero) =
            inside_parentheses
                .iter()
                .fold((1, false), |(depth, has_depth_zero), token| {
                    let depth_next = match token {
                        Token::ParenthesisEnd => depth + 1,
                        Token::ParenthesisBegin => depth - 1,
                        _ => depth,
                    };
                    (depth_next, has_depth_zero || depth == 0)
                });
        if has_depth_zero == false {
            return parse(&inside_parentheses.to_vec());
        }
    }

    if let Some(x) = parse_for_binary_operator(seq, &[(Token::LogicalOr, LogicalOr::new)]) {
        x
    } else if let Some(x) = parse_for_binary_operator(seq, &[(Token::LogicalAnd, LogicalAnd::new)])
    {
        x
    } else if let Some(x) = parse_for_binary_operator(seq, &[(Token::BitwiseOr, BitwiseOr::new)]) {
        x
    } else if let Some(x) = parse_for_binary_operator(seq, &[(Token::Xor, Xor::new)]) {
        x
    } else if let Some(x) = parse_for_binary_operator(seq, &[(Token::BitwiseAnd, BitwiseAnd::new)])
    {
        x
    } else if let Some(x) = parse_for_binary_operator(
        seq,
        &[(Token::Equal, Equal::new), (Token::NotEqual, NotEqual::new)],
    ) {
        x
    } else if let Some(x) = parse_for_binary_operator(
        seq,
        &[
            (Token::LessThan, LessThan::new),
            (Token::LessThanEqual, LessThanEqual::new),
            (Token::GreaterThan, GreaterThan::new),
            (Token::GreaterThanEqual, GreaterThanEqual::new),
        ],
    ) {
        x
    } else if let Some(x) = parse_for_binary_operator(
        seq,
        &[
            (Token::LeftShift, LeftShift::new),
            (Token::RightShift, RightShift::new),
        ],
    ) {
        x
    } else if let Some(x) =
        parse_for_binary_operator(seq, &[(Token::Add, Add::new), (Token::Sub, Sub::new)])
    {
        x
    } else if let Some(x) = parse_for_binary_operator(
        seq,
        &[
            (Token::Mul, Mul::new),
            (Token::Div, Div::new),
            (Token::Mod, Mod::new),
        ],
    ) {
        x
    } else if let Some(x) =
        parse_for_unary_operator(seq, &[(Token::Sub, Minus::new), (Token::Not, Not::new)])
    {
        x
    } else if let Some(x) = parse_for_function(
        seq,
        &[
            (Token::FnRef, FnRef::new, 2),
            (Token::FnSum, FnSum::new, 1),
            (Token::FnAvg, FnAvg::new, 1),
            (Token::FnIf, FnIf::new, 3),
            (Token::FnRound, FnRound::new, 1),
            (Token::FnFloor, FnFloor::new, 1),
            (Token::FnCeil, FnCeil::new, 1),
            (Token::FnLog, FnLog::new, 2),
            (Token::FnLn, FnLn::new, 1),
            (Token::FnLog2, FnLog2::new, 1),
            (Token::FnLog10, FnLog10::new, 1),
            (Token::FnSqrt, FnSqrt::new, 1),
            (Token::FnPow, FnPow::new, 2),
            (Token::FnSin, FnSin::new, 1),
            (Token::FnCos, FnCos::new, 1),
            (Token::FnTan, FnTan::new, 1),
            (Token::FnAsin, FnAsin::new, 1),
            (Token::FnAcos, FnAcos::new, 1),
            (Token::FnAtan, FnAtan::new, 1),
            (Token::FnSinh, FnSinh::new, 1),
            (Token::FnCosh, FnCosh::new, 1),
            (Token::FnTanh, FnTanh::new, 1),
            (Token::FnAsinh, FnAsinh::new, 1),
            (Token::FnAcosh, FnAcosh::new, 1),
            (Token::FnAtanh, FnAtanh::new, 1),
        ],
    ) {
        x
    } else if seq.len() == 1 {
        Value::new(vec![vec![seq[0].clone()]])
    } else {
        (Box::new(Value::Error), vec![])
    }
}

fn parse_for_binary_operator(
    seq: &Vec<Token>,
    op_targets: &[(
        Token,
        fn(seqs: Vec<Vec<Token>>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>),
    )],
) -> Option<(Box<ThreadSafeNode>, Vec<(usize, usize)>)> {
    let mut seq_before_op = vec![];
    let mut seq_after_op = vec![];
    let (initializer, _) = seq.iter().fold((None, 0), |(initializer, depth), token| {
        let depth_next = match token {
            Token::ParenthesisEnd => depth + 1,
            Token::ParenthesisBegin => depth - 1,
            _ => depth,
        };

        if let Some(_) = initializer {
            seq_before_op.push(token.clone());
            (initializer, depth_next)
        } else if let (Some((_, initializer)), true, true) = (
            op_targets.iter().find(|(op_target, _)| op_target == token),
            depth == 0,
            seq_after_op.len() != 0,
        ) {
            (Some(initializer), 0)
        } else {
            seq_after_op.push(token.clone());
            (None, depth_next)
        }
    });

    if let (Some(initializer), true) = (initializer, seq_before_op.len() != 0) {
        Some(initializer(vec![seq_before_op, seq_after_op]))
    } else {
        None
    }
}

fn parse_for_unary_operator(
    seq: &Vec<Token>,
    op_targets: &[(
        Token,
        fn(seqs: Vec<Vec<Token>>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>),
    )],
) -> Option<(Box<ThreadSafeNode>, Vec<(usize, usize)>)> {
    if let (Some((_, initializer)), true) = (
        op_targets
            .iter()
            .find(|(op_target, _)| Some(op_target) == seq.last()),
        seq.len() > 2,
    ) {
        Some(initializer(vec![seq[..seq.len() - 1].to_vec()]))
    } else {
        None
    }
}

fn parse_for_function(
    seq: &Vec<Token>,
    op_targets: &[(
        Token,
        fn(seqs: Vec<Vec<Token>>) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>),
        usize,
    )],
) -> Option<(Box<ThreadSafeNode>, Vec<(usize, usize)>)> {
    if let (
        Some(&Token::ParenthesisEnd),
        Some(&Token::ParenthesisBegin),
        Some((_, initializer, num_of_args)),
    ) = (
        seq.first(),
        seq.get(seq.len() - 2),
        op_targets
            .iter()
            .find(|(op_target, _, _)| Some(op_target) == seq.last()),
    ) {
        let mut args = vec![];
        let mut arg = vec![];
        let mut depth = 0;
        for token in seq[1..seq.len() - 2].iter() {
            if token == &Token::ParenthesisEnd {
                depth += 1;
            } else if token == &Token::ParenthesisBegin {
                depth -= 1;
            }

            if depth == 0 && token == &Token::Comma {
                args.push(arg);
                arg = vec![];
            } else {
                arg.push(token.clone());
            }
        }

        if arg.len() != 0 {
            args.push(arg);
        }

        if &args.len() == num_of_args {
            args.reverse();
            Some(initializer(args))
        } else {
            None
        }
    } else {
        None
    }
}
