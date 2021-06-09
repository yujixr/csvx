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
    if seq.first() == Some(&Token::ParenthesisBegin) && seq.last() == Some(&Token::ParenthesisEnd) {
        let inside_parentheses = &seq[1..seq.len() - 1];
        let (_, has_depth_zero) =
            inside_parentheses
                .iter()
                .fold((1, false), |(depth, has_depth_zero), token| {
                    let depth_next = match token {
                        Token::ParenthesisBegin => depth + 1,
                        Token::ParenthesisEnd => depth - 1,
                        _ => depth,
                    };
                    (depth_next, has_depth_zero || depth == 0)
                });
        if has_depth_zero == false {
            return parse(&inside_parentheses.to_vec());
        }
    }

    let logical_or = parse_for_binary_operator(seq, Token::LogicalOr);
    if let Some((left, right)) = logical_or {
        return LogicalOr::new(vec![left, right]);
    }

    let logical_and = parse_for_binary_operator(seq, Token::LogicalAnd);
    if let Some((left, right)) = logical_and {
        return LogicalAnd::new(vec![left, right]);
    }

    let bitwise_or = parse_for_binary_operator(seq, Token::BitwiseOr);
    if let Some((left, right)) = bitwise_or {
        return BitwiseOr::new(vec![left, right]);
    }

    let bitwise_and = parse_for_binary_operator(seq, Token::BitwiseAnd);
    if let Some((left, right)) = bitwise_and {
        return BitwiseAnd::new(vec![left, right]);
    }

    let equal = parse_for_binary_operator(seq, Token::Equal);
    if let Some((left, right)) = equal {
        return Equal::new(vec![left, right]);
    }

    let not_equal = parse_for_binary_operator(seq, Token::NotEqual);
    if let Some((left, right)) = not_equal {
        return NotEqual::new(vec![left, right]);
    }

    let less_than = parse_for_binary_operator(seq, Token::LessThan);
    if let Some((left, right)) = less_than {
        return LessThan::new(vec![left, right]);
    }

    let less_than_equal = parse_for_binary_operator(seq, Token::LessThanEqual);
    if let Some((left, right)) = less_than_equal {
        return LessThanEqual::new(vec![left, right]);
    }

    let greater_than = parse_for_binary_operator(seq, Token::GreaterThan);
    if let Some((left, right)) = greater_than {
        return GreaterThan::new(vec![left, right]);
    }

    let greater_than_equal = parse_for_binary_operator(seq, Token::GreaterThanEqual);
    if let Some((left, right)) = greater_than_equal {
        return GreaterThanEqual::new(vec![left, right]);
    }

    let left_shift = parse_for_binary_operator(seq, Token::LeftShift);
    if let Some((left, right)) = left_shift {
        return LeftShift::new(vec![left, right]);
    }

    let right_shift = parse_for_binary_operator(seq, Token::RightShift);
    if let Some((left, right)) = right_shift {
        return RightShift::new(vec![left, right]);
    }

    let add = parse_for_binary_operator(seq, Token::Add);
    if let Some((left, right)) = add {
        return Add::new(vec![left, right]);
    }

    let sub = parse_for_binary_operator(seq, Token::Sub);
    if let Some((left, right)) = sub {
        return Sub::new(vec![left, right]);
    }

    let mul = parse_for_binary_operator(seq, Token::Mul);
    if let Some((left, right)) = mul {
        return Mul::new(vec![left, right]);
    }

    let div = parse_for_binary_operator(seq, Token::Div);
    if let Some((left, right)) = div {
        return Div::new(vec![left, right]);
    }

    let modulo = parse_for_binary_operator(seq, Token::Mod);
    if let Some((left, right)) = modulo {
        return Mod::new(vec![left, right]);
    }

    let xor = parse_for_binary_operator(seq, Token::Xor);
    if let Some((left, right)) = xor {
        return Xor::new(vec![left, right]);
    }

    let minus = parse_for_unary_operator(seq, Token::Sub);
    if let Some(leaf) = minus {
        return Minus::new(vec![leaf]);
    }

    let not = parse_for_unary_operator(seq, Token::Not);
    if let Some(leaf) = not {
        return Not::new(vec![leaf]);
    }

    let fn_ref = parse_for_function(seq, Token::FnRef, 2);
    if let Some(args) = fn_ref {
        return FnRef::new(args);
    }

    let fn_if = parse_for_function(seq, Token::FnIf, 3);
    if let Some(args) = fn_if {
        return FnIf::new(args);
    }

    let fn_round = parse_for_function(seq, Token::FnRound, 1);
    if let Some(args) = fn_round {
        return FnRound::new(args);
    }

    let fn_floor = parse_for_function(seq, Token::FnFloor, 1);
    if let Some(args) = fn_floor {
        return FnFloor::new(args);
    }

    let fn_ceil = parse_for_function(seq, Token::FnCeil, 1);
    if let Some(args) = fn_ceil {
        return FnCeil::new(args);
    }

    let fn_log = parse_for_function(seq, Token::FnLog, 2);
    if let Some(args) = fn_log {
        return FnLog::new(args);
    }

    let fn_ln = parse_for_function(seq, Token::FnLn, 1);
    if let Some(args) = fn_ln {
        return FnLn::new(args);
    }

    let fn_log2 = parse_for_function(seq, Token::FnLog2, 1);
    if let Some(args) = fn_log2 {
        return FnLog2::new(args);
    }

    let fn_log10 = parse_for_function(seq, Token::FnLog10, 1);
    if let Some(args) = fn_log10 {
        return FnLog10::new(args);
    }

    let fn_sqrt = parse_for_function(seq, Token::FnSqrt, 1);
    if let Some(args) = fn_sqrt {
        return FnSqrt::new(args);
    }

    let fn_pow = parse_for_function(seq, Token::FnPow, 2);
    if let Some(args) = fn_pow {
        return FnPow::new(args);
    }

    let fn_sin = parse_for_function(seq, Token::FnSin, 1);
    if let Some(args) = fn_sin {
        return FnSin::new(args);
    }

    let fn_cos = parse_for_function(seq, Token::FnCos, 1);
    if let Some(args) = fn_cos {
        return FnCos::new(args);
    }

    let fn_tan = parse_for_function(seq, Token::FnTan, 1);
    if let Some(args) = fn_tan {
        return FnTan::new(args);
    }

    let fn_asin = parse_for_function(seq, Token::FnAsin, 1);
    if let Some(args) = fn_asin {
        return FnAsin::new(args);
    }

    let fn_acos = parse_for_function(seq, Token::FnAcos, 1);
    if let Some(args) = fn_acos {
        return FnAcos::new(args);
    }

    let fn_atan = parse_for_function(seq, Token::FnAtan, 1);
    if let Some(args) = fn_atan {
        return FnAtan::new(args);
    }

    let fn_sinh = parse_for_function(seq, Token::FnSinh, 1);
    if let Some(args) = fn_sinh {
        return FnSinh::new(args);
    }

    let fn_cosh = parse_for_function(seq, Token::FnCosh, 1);
    if let Some(args) = fn_cosh {
        return FnCosh::new(args);
    }

    let fn_tanh = parse_for_function(seq, Token::FnTanh, 1);
    if let Some(args) = fn_tanh {
        return FnTanh::new(args);
    }

    let fn_asinh = parse_for_function(seq, Token::FnAsinh, 1);
    if let Some(args) = fn_asinh {
        return FnAsinh::new(args);
    }

    let fn_acosh = parse_for_function(seq, Token::FnAcosh, 1);
    if let Some(args) = fn_acosh {
        return FnAcosh::new(args);
    }

    let fn_atanh = parse_for_function(seq, Token::FnAtanh, 1);
    if let Some(args) = fn_atanh {
        return FnAtanh::new(args);
    }

    if seq.len() == 1 {
        return Value::new(vec![vec![seq[0].clone()]]);
    }

    (Box::new(Value::Error), vec![])
}

fn parse_for_binary_operator(
    seq: &Vec<Token>,
    op_target: Token,
) -> Option<(Vec<Token>, Vec<Token>)> {
    let mut seq_before_op = vec![];
    let mut seq_after_op = vec![];
    let (is_op_found, _) = seq
        .iter()
        .fold((false, 0), |(is_already_found, depth), token| {
            let depth_next = match token {
                Token::ParenthesisBegin => depth + 1,
                Token::ParenthesisEnd => depth - 1,
                _ => depth,
            };

            if is_already_found == false
                && token == &op_target
                && depth == 0
                && seq_before_op.len() != 0
            {
                (true, 0)
            } else {
                if is_already_found == false {
                    seq_before_op.push(token.clone());
                } else {
                    seq_after_op.push(token.clone());
                }
                (is_already_found, depth_next)
            }
        });

    if is_op_found == true {
        Some((seq_before_op, seq_after_op))
    } else {
        None
    }
}

fn parse_for_unary_operator(seq: &Vec<Token>, op_target: Token) -> Option<Vec<Token>> {
    if seq.first() == Some(&op_target) {
        Some(seq[1..].to_vec())
    } else {
        None
    }
}

fn parse_for_function(
    seq: &Vec<Token>,
    op_target: Token,
    num_of_args: usize,
) -> Option<Vec<Vec<Token>>> {
    if seq.len() > 3
        && seq.first() == Some(&op_target)
        && seq[1] == Token::ParenthesisBegin
        && seq.last() == Some(&Token::ParenthesisEnd)
    {
        let mut args = vec![];
        let mut arg = vec![];
        let mut depth = 0;
        for token in seq[2..seq.len() - 1].iter() {
            if token == &Token::ParenthesisBegin {
                depth += 1;
            } else if token == &Token::ParenthesisEnd {
                depth -= 1;
            }

            if depth == 0 && token == &Token::Comma {
                args.push(arg);
                arg = vec![];
            } else {
                arg.push(token.clone());
            }
        }
        args.push(arg);

        if args.len() == num_of_args {
            return Some(args);
        }
    }
    None
}
