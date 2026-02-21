from unittest.mock import patch

from hello import main


def test_main_output(capsys):
    """Test that main() prints the expected greeting."""
    with patch("sys.argv", ["hello.py"]):
        main()
    captured = capsys.readouterr()
    assert captured.out == "Hello from the task queue!\n"


def test_main_no_stderr(capsys):
    """Test that main() produces no stderr output."""
    with patch("sys.argv", ["hello.py"]):
        main()
    captured = capsys.readouterr()
    assert captured.err == ""


def test_main_with_name(capsys):
    """Test that main() prints a personalized greeting when --name is provided."""
    with patch("sys.argv", ["hello.py", "--name", "Alice"]):
        main()
    captured = capsys.readouterr()
    assert captured.out == "Hello, Alice!\n"
