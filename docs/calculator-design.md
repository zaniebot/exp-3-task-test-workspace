# Calculator Module Design

## Overview

This document describes the design for a simple calculator module that provides basic arithmetic operations. The module will be implemented in Python and organized so that each operation resides in its own submodule for clarity and maintainability.

## Module Structure

```
calculator/
├── __init__.py          # Package init, re-exports all operations
├── add.py               # Addition operation
├── subtract.py          # Subtraction operation
├── multiply.py          # Multiplication operation
└── divide.py            # Division operation
```

## Operations

### Add (`calculator/add.py`)

```python
def add(a: float, b: float) -> float:
    """Return the sum of a and b."""
```

### Subtract (`calculator/subtract.py`)

```python
def subtract(a: float, b: float) -> float:
    """Return the result of a minus b."""
```

### Multiply (`calculator/multiply.py`)

```python
def multiply(a: float, b: float) -> float:
    """Return the product of a and b."""
```

### Divide (`calculator/divide.py`)

```python
def divide(a: float, b: float) -> float:
    """Return the result of a divided by b.
    
    Raises:
        ZeroDivisionError: If b is zero.
    """
```

## Design Decisions

1. **Separate modules per operation** – Keeps each unit small and independently testable.
2. **Float parameters** – All inputs and outputs use `float` to support decimal arithmetic.
3. **Explicit error handling for division** – `divide` raises `ZeroDivisionError` when the divisor is zero rather than returning a sentinel value.
4. **Re-export from `__init__.py`** – Users can import directly from `calculator` (e.g., `from calculator import add`).

## Testing Strategy

Each operation should have its own test file under `tests/` with cases covering:
- Positive and negative numbers
- Zero as an operand
- Division by zero (expecting `ZeroDivisionError`)
- Large numbers and floating-point edge cases
