# Calculator Module Design

## Overview

A simple calculator module providing basic arithmetic operations. The module will be implemented in Python as a set of focused sub-modules, each handling a specific operation, with a unified interface exposed at the package level.

## Module Structure

```
calculator/
├── __init__.py        # Public API — re-exports all operations
├── add.py             # Addition operation
├── subtract.py        # Subtraction operation
├── multiply.py        # Multiplication operation
└── divide.py          # Division operation
```

## Operations

### 1. Add (`calculator.add`)

```python
def add(a: float, b: float) -> float:
    """Return the sum of a and b."""
```

- Accepts two numeric arguments.
- Returns their sum as a float.

### 2. Subtract (`calculator.subtract`)

```python
def subtract(a: float, b: float) -> float:
    """Return the result of a minus b."""
```

- Accepts two numeric arguments.
- Returns `a - b` as a float.

### 3. Multiply (`calculator.multiply`)

```python
def multiply(a: float, b: float) -> float:
    """Return the product of a and b."""
```

- Accepts two numeric arguments.
- Returns `a * b` as a float.

### 4. Divide (`calculator.divide`)

```python
def divide(a: float, b: float) -> float:
    """Return the result of a divided by b.

    Raises:
        ZeroDivisionError: If b is zero.
    """
```

- Accepts two numeric arguments.
- Returns `a / b` as a float.
- Raises `ZeroDivisionError` when `b` is zero.

## Design Principles

- **Separation of concerns**: Each operation lives in its own module for clarity and independent testability.
- **Type hints**: All functions use type annotations for better tooling support.
- **Error handling**: Division by zero is propagated as a standard `ZeroDivisionError`.
- **Testing**: Each module should have a corresponding test file under `tests/`.

## Testing Strategy

Tests will live in a `tests/` directory mirroring the module structure:

```
tests/
├── test_add.py
├── test_subtract.py
├── test_multiply.py
└── test_divide.py
```

Each test file should cover:
- Normal cases with positive and negative numbers
- Edge cases (zero, very large numbers, floats)
- Error cases (division by zero for divide)
