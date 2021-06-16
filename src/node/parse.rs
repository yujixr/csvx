use super::*;

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

    if let Some(x) = parse_for_binary_operator(seq, &[(Token::LogicalOr, logical_or::Node::new)]) {
        x
    } else if let Some(x) =
        parse_for_binary_operator(seq, &[(Token::LogicalAnd, logical_and::Node::new)])
    {
        x
    } else if let Some(x) =
        parse_for_binary_operator(seq, &[(Token::BitwiseOr, bitwise_or::Node::new)])
    {
        x
    } else if let Some(x) = parse_for_binary_operator(seq, &[(Token::Xor, xor::Node::new)]) {
        x
    } else if let Some(x) =
        parse_for_binary_operator(seq, &[(Token::BitwiseAnd, bitwise_and::Node::new)])
    {
        x
    } else if let Some(x) = parse_for_binary_operator(
        seq,
        &[
            (Token::Equal, equal::Node::new),
            (Token::NotEqual, not_equal::Node::new),
        ],
    ) {
        x
    } else if let Some(x) = parse_for_binary_operator(
        seq,
        &[
            (Token::LessThan, less_than::Node::new),
            (Token::LessThanEqual, less_than_equal::Node::new),
            (Token::GreaterThan, greater_than::Node::new),
            (Token::GreaterThanEqual, greater_than_equal::Node::new),
        ],
    ) {
        x
    } else if let Some(x) = parse_for_binary_operator(
        seq,
        &[
            (Token::LeftShift, left_shift::Node::new),
            (Token::RightShift, right_shift::Node::new),
        ],
    ) {
        x
    } else if let Some(x) = parse_for_binary_operator(
        seq,
        &[(Token::Add, add::Node::new), (Token::Sub, sub::Node::new)],
    ) {
        x
    } else if let Some(x) = parse_for_binary_operator(
        seq,
        &[
            (Token::Mul, mul::Node::new),
            (Token::Div, div::Node::new),
            (Token::Mod, modulo::Node::new),
        ],
    ) {
        x
    } else if let Some(x) = parse_for_unary_operator(
        seq,
        &[(Token::Sub, minus::Node::new), (Token::Not, not::Node::new)],
    ) {
        x
    } else if let Some(x) = parse_for_function(
        seq,
        &[
            (Token::FnRef, fn_ref::Node::new, 2),
            (Token::FnSum, fn_sum::Node::new, 1),
            (Token::FnAvg, fn_avg::Node::new, 1),
            (Token::FnIf, fn_if::Node::new, 3),
            (Token::FnRound, fn_round::Node::new, 1),
            (Token::FnFloor, fn_floor::Node::new, 1),
            (Token::FnCeil, fn_ceil::Node::new, 1),
            (Token::FnLog, fn_log::Node::new, 2),
            (Token::FnLn, fn_ln::Node::new, 1),
            (Token::FnLog2, fn_log2::Node::new, 1),
            (Token::FnLog10, fn_log10::Node::new, 1),
            (Token::FnSqrt, fn_sqrt::Node::new, 1),
            (Token::FnPow, fn_pow::Node::new, 2),
            (Token::FnSin, fn_sin::Node::new, 1),
            (Token::FnCos, fn_cos::Node::new, 1),
            (Token::FnTan, fn_tan::Node::new, 1),
            (Token::FnAsin, fn_asin::Node::new, 1),
            (Token::FnAcos, fn_acos::Node::new, 1),
            (Token::FnAtan, fn_atan::Node::new, 1),
            (Token::FnSinh, fn_sinh::Node::new, 1),
            (Token::FnCosh, fn_cosh::Node::new, 1),
            (Token::FnTanh, fn_tanh::Node::new, 1),
            (Token::FnAsinh, fn_asinh::Node::new, 1),
            (Token::FnAcosh, fn_acosh::Node::new, 1),
            (Token::FnAtanh, fn_atanh::Node::new, 1),
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
    if seq.len() < 3 {
        return None;
    }

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
