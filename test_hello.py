import sys
import pytest
from unittest.mock import patch
from hello import main, goodbye


def test_main_default(capsys):
    """Test main() with no arguments prints default greeting."""
    with patch("sys.argv", ["hello.py"]):
        main()
    captured = capsys.readouterr()
    assert captured.out == "Hello from the task queue!\n"


def test_main_with_name(capsys):
    """Test main() with --name flag prints personalized greeting."""
    with patch("sys.argv", ["hello.py", "--name", "Alice"]):
        main()
    captured = capsys.readouterr()
    assert captured.out == "Hello, Alice!\n"


def test_main_with_different_name(capsys):
    """Test main() with a different name."""
    with patch("sys.argv", ["hello.py", "--name", "Bob"]):
        main()
    captured = capsys.readouterr()
    assert captured.out == "Hello, Bob!\n"


def test_goodbye(capsys):
    """Test goodbye() prints goodbye message."""
    goodbye()
    captured = capsys.readouterr()
    assert captured.out == "Goodbye from the task queue!\n"
