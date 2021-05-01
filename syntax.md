# Syntax Reference

Since this is an extension of CSV, the basic format is the same as CSV.
The following describes the syntax of the expression for each item.

## Supported Types of Values

| Type Name | Syntax |
|---|---|
| Integer | Decimal, hexadecimal, octal, and binary numbers are supported. |
| Float | Only decimal floating point numbers are supported. |
| String | Be sure to enclose the string in double quotes. |
| Boolean | Specify true or false. |
| Ref | Specify the rows in uppercase letters and the columns in numbers. For example, the upper left is A1, and the one to the right is B1. |

## Supported Operators

| Operator | Meaning |
|---|---|
| + | Addition |
| - | Subtraction, minus |
| * | Multiplication |
| / | Division |
| ! | Logical NOT, bitwise inversion |
| && | Logical AND |
| &#124;&#124; | Logical OR |
| && | Logical AND |
| &#124;&#124; | Logical OR ||
| & | Bitwise AND |
| &#124; | Bitwise OR |
| ^ | Logical, bitwise XOR |
| &gt;&gt; | Right shift |
| << | Left shift |
| == | Equal |
| != | Unequal |
| < | Less than  |
| <= | Less than or Equal |
| &gt; | Greater than  |
| &gt;= | Greater than or Equal |

## Supported Functions

| Function | Description |
|---|---|
| ref(x, y) | Deprecated. Returns the value at (x, y). |
| if(condition, on_true, on_false) | If condition is true, this returns on_true. Otherwise, it returns on_false. |
| round(x) | Returns the nearest integer to a number. Round half-way cases away from 0.0. |
| floor(x) | Returns the largest integer less than or equal to a number. |
| ceil(x) | Returns the smallest integer greater than or equal to a number. |
| log(base, x) | Returns the logarithm of the number with respect to an arbitrary base. |
| ln(x) | Returns the natural logarithm of the number. |
| log2(x) | Returns the base 2 logarithm of the number. |
| log10(x) | Returns the base 10 logarithm of the number. |
| sqrt(x) | Returns the square root of a number. |
| pow(base, exp) | Raises the base to the exp power. |
| sin(x) | Computes the sine of a number (in radians). |
| cos(x) | Computes the cosine of a number (in radians). |
| tan(x) | Computes the tangent of a number (in radians). |
| asin(x) | Computes the arcsine of a number (in radians). |
| acos(x) | Computes the arccosine of a number (in radians). |
| atan(x) | Computes the arctangent of a number (in radians). |
| sinh(x) | Hyperbolic sine function. |
| cosh(x) | Hyperbolic cosine function. |
| tanh(x) | Hyperbolic tangent function. |
| asinh(x) | Inverse hyperbolic sine function. |
| acosh(x) | Inverse hyperbolic cosine function. |
| atanh(x) | Inverse hyperbolic tangent function. |
